// Axel '0vercl0k' Souchet - August 20 2023
use std::ops::Deref;
use std::path::PathBuf;

use log::debug;
use windows_core::PWSTR;

use crate::bindings::DUPLICATE_SAME_ACCESS;
use crate::bindings_sys::{
    CloseHandle, DuplicateHandle, GetCurrentProcess, GetProcessId, HANDLE, INVALID_HANDLE_VALUE,
    MAX_PATH, NtQueryObject, ObjectTypeInformation, PUBLIC_OBJECT_TYPE_INFORMATION,
    QueryFullProcessImageNameW, STATUS_INFO_LENGTH_MISMATCH,
};
use crate::utils::AlignedAlloc;
use crate::{Result, try_from, try_from_usize};

/// A [`HANDLE`] that gets closed automatically if owned.
#[derive(Debug)]
pub struct Handle {
    handle: HANDLE,
    owned: bool,
}

// SAFETY: Although a `HANDLE` is typed as a `void*` it actually is an integer;
// regardless it is safe to send across threads.
unsafe impl Send for Handle {}

impl Deref for Handle {
    type Target = HANDLE;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl Default for Handle {
    fn default() -> Self {
        Self {
            handle: INVALID_HANDLE_VALUE,
            owned: false,
        }
    }
}

impl From<HANDLE> for Handle {
    fn from(value: HANDLE) -> Self {
        Self::adopt(value)
    }
}

impl Handle {
    /// Adopt a handle, and own it going forward.
    pub fn adopt(handle: HANDLE) -> Self {
        Self {
            handle,
            owned: true,
        }
    }

    /// Wrap a handle but don't own it.
    pub fn wrap(handle: HANDLE) -> Self {
        Self {
            handle,
            owned: false,
        }
    }

    pub fn is_owned(&self) -> bool {
        self.owned
    }

    pub fn is_invalid(&self) -> bool {
        self.handle.0.is_null() || self.handle == INVALID_HANDLE_VALUE
    }

    pub fn duplicate(&self) -> Result<Self> {
        Self::duplicate_from(self)
    }

    pub fn duplicate_from(handle: &Self) -> Result<Self> {
        let mut duplicated_handle = INVALID_HANDLE_VALUE;
        unsafe {
            DuplicateHandle(
                GetCurrentProcess(),
                **handle,
                GetCurrentProcess(),
                &raw mut duplicated_handle,
                0,
                false,
                DUPLICATE_SAME_ACCESS,
            )
        }
        .ok()?;

        Ok(duplicated_handle.into())
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        if !self.is_owned() || self.is_invalid() {
            return;
        }

        let closed = unsafe { CloseHandle(self.handle) };
        debug_assert!(closed.as_bool());
    }
}

#[derive(Debug)]
pub struct ProcessHandle(Handle);

impl Deref for ProcessHandle {
    type Target = HANDLE;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ProcessHandle {
    fn new(handle: Handle) -> Self {
        Self(handle)
    }

    pub fn duplicate(&self) -> Result<ProcessHandle> {
        self.0.duplicate().map(ProcessHandle::new)
    }

    pub fn pid(&self) -> u32 {
        let ret = unsafe { GetProcessId(*self.0) };
        assert_ne!(ret, 0);

        ret
    }

    pub fn from_handle(handle: Handle) -> Option<Self> {
        let mut needed_len = 0u32;
        assert_eq!(
            unsafe {
                NtQueryObject(
                    Some(*handle),
                    ObjectTypeInformation,
                    None,
                    0,
                    Some(&raw mut needed_len),
                )
            },
            STATUS_INFO_LENGTH_MISMATCH
        );

        let mut info =
            AlignedAlloc::<PUBLIC_OBJECT_TYPE_INFORMATION>::new(try_from_usize!(needed_len));
        let status = unsafe {
            NtQueryObject(
                Some(*handle),
                ObjectTypeInformation,
                Some(info.as_mut_ptr().cast()),
                needed_len,
                Some(&raw mut needed_len),
            )
        };

        if status.is_err() {
            debug!("NtQueryObject failed w/ {:#x}", status.0);
            return None;
        }

        let Ok(typename) =
            String::from_utf16(unsafe { (*(info.as_ptr())).TypeName.Buffer.as_wide() })
        else {
            return None;
        };

        if typename == "Process" {
            Some(Self::new(handle))
        } else {
            None
        }
    }

    pub fn query_full_process_image_name(&self) -> Result<PathBuf> {
        let mut buffer = [0u16; MAX_PATH as usize];
        let mut buffer_len = try_from!(u32, buffer.len());

        unsafe {
            QueryFullProcessImageNameW(
                *self.0,
                0,
                PWSTR::from_raw(buffer.as_mut_ptr()),
                &raw mut buffer_len,
            )
        }
        .ok()?;

        let s = String::from_utf16(&buffer[..buffer_len as usize])?.to_lowercase();

        Ok(PathBuf::from(s))
    }
}
