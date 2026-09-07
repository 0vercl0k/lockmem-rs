// Axel '0vercl0k' Souchet - July 15 2026
use std::alloc::{Layout, alloc, dealloc};
use std::ffi::CString;
use std::marker::PhantomData;

use windows_core::{PCSTR, PSTR};

use crate::bindings::{ENABLE_VIRTUAL_TERMINAL_PROCESSING, TOKEN_QUERY};
use crate::bindings_sys::{
    GetConsoleMode, GetCurrentProcess, GetModuleFileNameA, GetStdHandle, GetTokenInformation,
    HANDLE, MAX_PATH, OpenProcessToken, STD_OUTPUT_HANDLE, SW_NORMAL, SetConsoleMode,
    ShellExecuteA, TokenElevationType,
};
use crate::error::Result;
use crate::handle::Handle;

#[macro_export]
macro_rules! try_from {
    ($t: ty, $v: expr) => {
        <$t>::try_from($v).unwrap()
    };
}
#[macro_export]
macro_rules! try_from_usize {
    ($v: expr) => {
        usize::try_from($v).unwrap()
    };
}

#[derive(Debug)]
pub(crate) struct AlignedAlloc<T> {
    ptr: *mut u8,
    layout: Layout,
    _marker: PhantomData<T>,
}

impl<T> AlignedAlloc<T> {
    pub(crate) fn new(bytes: usize) -> Self {
        let layout = Layout::from_size_align(bytes, align_of::<T>()).unwrap();
        let ptr = unsafe { alloc(layout) };
        assert!(!ptr.is_null(), "alloc failed");

        Self {
            ptr,
            layout,
            _marker: PhantomData,
        }
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.cast()
    }

    pub(crate) fn as_ptr(&self) -> *const T {
        self.ptr.cast()
    }
}

impl<T> Drop for AlignedAlloc<T> {
    fn drop(&mut self) {
        unsafe { dealloc(self.ptr, self.layout) };
    }
}

#[derive(Debug)]
pub(crate) enum TokenKind {
    Default,
    Limited,
    Full,
}

pub(crate) fn relaunch_elevated(target: &str) -> Result<()> {
    let mut path = vec![0; try_from_usize!(MAX_PATH)];
    let path_len =
        unsafe { GetModuleFileNameA(None, PSTR(path.as_mut_ptr()), path.len().try_into()?) };
    assert!(path_len > 0);

    path.resize(try_from_usize!(path_len), 0);
    let path = CString::new(path).unwrap();

    let params = CString::new(format!("--elevated {target}")).unwrap();
    let operation = c"runas";

    let status = unsafe {
        ShellExecuteA(
            None,
            PCSTR::from_raw(operation.as_ptr().cast()),
            PCSTR::from_raw(path.as_ptr().cast()),
            PCSTR::from_raw(params.as_ptr().cast()),
            PCSTR::null(),
            SW_NORMAL,
        )
    };

    if status.0 as usize <= 32 {
        Err(format!("ShellExecuteA failed w/ {}", status.0 as usize).into())
    } else {
        Ok(())
    }
}

pub(crate) fn limited_token() -> Result<TokenKind> {
    let process = Handle::wrap(unsafe { GetCurrentProcess() });
    let mut token = HANDLE::default();
    unsafe { OpenProcessToken(*process, TOKEN_QUERY, &raw mut token) }.ok()?;
    let token = Handle::adopt(token);

    let mut token_ty = 0u32;
    let mut needed = 0;
    assert!(
        unsafe {
            GetTokenInformation(
                *token,
                TokenElevationType,
                Some((&raw mut token_ty).cast()),
                size_of_val(&token_ty).try_into().unwrap(),
                &raw mut needed,
            )
        }
        .as_bool()
    );

    assert_eq!(try_from_usize!(needed), size_of_val(&token_ty));

    Ok(match token_ty {
        1 => TokenKind::Default,
        2 => TokenKind::Full,
        3 => TokenKind::Limited,
        v => panic!("unexpected value {v}"),
    })
}

pub(crate) fn turn_on_vt() -> Result<()> {
    let h = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
    let mut mode = 0;
    if !unsafe { GetConsoleMode(h, &raw mut mode) }.as_bool() {
        return Err(windows_core::Error::from_thread().into());
    }

    mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
    if !unsafe { SetConsoleMode(h, mode) }.as_bool() {
        return Err(windows_core::Error::from_thread().into());
    }

    Ok(())
}
