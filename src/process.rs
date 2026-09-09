// Axel '0vercl0k' Souchet - December 6th 2024
use std::ffi::c_void;
use std::ptr::null;
use std::range::Range;

use log::debug;
use windows_core::NTSTATUS;

use crate::bindings::{
    PROCESS_QUERY_INFORMATION, PROCESS_SET_QUOTA, PROCESS_VM_OPERATION,
    QUOTA_LIMITS_HARDWS_MAX_DISABLE, QUOTA_LIMITS_HARDWS_MIN_ENABLE, TH32CS_SNAPPROCESS,
};
use crate::bindings_sys::{
    CreateToolhelp32Snapshot, GetProcessWorkingSetSizeEx, HANDLE, MEMORY_BASIC_INFORMATION,
    OpenProcess, PROCESSENTRY32W, Process32FirstW, Process32NextW, STATUS_INCOMPATIBLE_FILE_MAP,
    STATUS_SUCCESS, STATUS_WAS_LOCKED, SetProcessWorkingSetSizeEx, VirtualQueryEx,
};
use crate::error::{Error, Result};
use crate::handle::{Handle, ProcessHandle};
use crate::human::ToHuman;
use crate::try_from_usize;

#[link(name = "ntdll")]
unsafe extern "system" {
    pub fn NtLockVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut u64,
        RegionSize: *mut usize,
        MapType: u32,
    ) -> NTSTATUS;
}

#[derive(Debug)]
pub struct ProcessesIter {
    snapshot: Handle,
    first: bool,
}

impl ProcessesIter {
    fn new(snapshot: Handle) -> Self {
        Self {
            snapshot,
            first: true,
        }
    }
}

impl Iterator for ProcessesIter {
    type Item = PROCESSENTRY32W;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pe32 = PROCESSENTRY32W::default();
        pe32.dwSize = size_of_val(&pe32).try_into().unwrap();

        if self.first {
            self.first = false;

            assert!(unsafe { Process32FirstW(*self.snapshot, &raw mut pe32) }.as_bool());

            Some(pe32)
        } else {
            match unsafe { Process32NextW(*self.snapshot, &raw mut pe32) }.ok() {
                Err(_) => None,
                _ => Some(pe32),
            }
        }
    }
}

#[derive(Debug)]
pub struct Processes;

impl Processes {
    pub fn iter() -> Result<ProcessesIter> {
        let h = Handle::adopt(unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) });
        if h.is_invalid() {
            return Err(Error::Win32(windows_core::Error::from_thread()));
        }

        Ok(ProcessesIter::new(h))
    }
}

#[derive(Debug)]
pub struct VirtMemIterator {
    handle: ProcessHandle,
    addr: *const c_void,
}

impl Iterator for VirtMemIterator {
    type Item = MEMORY_BASIC_INFORMATION;

    fn next(&mut self) -> Option<Self::Item> {
        let mut mem_info = MEMORY_BASIC_INFORMATION::default();

        if unsafe {
            VirtualQueryEx(
                *self.handle,
                Some(self.addr),
                &raw mut mem_info,
                size_of_val(&mem_info),
            )
        } == 0
        {
            return None;
        }

        self.addr = unsafe { mem_info.BaseAddress.byte_add(mem_info.RegionSize) }.cast();

        Some(mem_info)
    }
}

#[derive(Debug)]
pub struct Process {
    pid: u32,
    name: String,
    handle: ProcessHandle,
}

impl Process {
    pub fn from_handle(handle: ProcessHandle) -> Result<Self> {
        let pid = handle.pid();
        let path = handle.query_full_process_image_name()?;
        let name = path.file_name().unwrap().to_str().unwrap().to_string();

        Ok(Self { pid, name, handle })
    }

    pub fn grown_and_lock_mem(&self, range: Range<u64>) -> Result<u64> {
        const MAP_PROCESS: u32 = 1;
        let mut range_len = try_from_usize!(range.end - range.start);
        let mut start = range.start;
        let status = unsafe {
            NtLockVirtualMemory(
                *self.handle,
                &raw mut start,
                &raw mut range_len,
                MAP_PROCESS,
            )
        };

        'check_lock: {
            match status {
                STATUS_SUCCESS => {}
                STATUS_INCOMPATIBLE_FILE_MAP => {
                    debug!(
                        "NtLockVirtualMemory on {range:#x?} returned STATUS_INCOMPATIBLE_FILE_MAP"
                    );
                }
                STATUS_WAS_LOCKED => {
                    debug!("{range:#x?} is locked already, skipping");
                }
                _ => break 'check_lock,
            }

            return Ok(range.end - range.start);
        }

        debug!("NtLockVirtualMemory failed w/ {:#x}", status.0);

        let mut minimum_ws_len = 0;
        let mut maximum_ws_len = 0;
        let mut flags = 0;
        unsafe {
            GetProcessWorkingSetSizeEx(
                *self.handle,
                &raw mut minimum_ws_len,
                &raw mut maximum_ws_len,
                &raw mut flags,
            )
        }
        .ok()?;

        minimum_ws_len += range_len;
        maximum_ws_len += range_len;

        debug!(
            "growing ws to {} {minimum_ws_len}",
            minimum_ws_len.human_bytes()
        );

        flags = QUOTA_LIMITS_HARDWS_MIN_ENABLE | QUOTA_LIMITS_HARDWS_MAX_DISABLE;

        unsafe { SetProcessWorkingSetSizeEx(*self.handle, minimum_ws_len, maximum_ws_len, flags) }
            .ok()?;

        let status = unsafe {
            NtLockVirtualMemory(
                *self.handle,
                &raw mut start,
                &raw mut range_len,
                MAP_PROCESS,
            )
        };

        if status != STATUS_SUCCESS {
            debug!("second attempt locking {range:#x?} failed w/ {status:#?}");
            return Err(format!("failed to lock {range:#x?} w/ {status:#?}").into());
        }

        debug!("second attempt locking {range:#x?} worked!");

        Ok(range.end - range.start)
    }

    pub fn iter_mem(&self) -> Result<VirtMemIterator> {
        Ok(VirtMemIterator {
            handle: self.handle.duplicate()?,
            addr: null(),
        })
    }

    /// Create a [`Process`] a process from a pid.
    pub fn from_pid(pid: u32) -> Result<Option<Self>> {
        let handle = Handle::adopt(unsafe {
            OpenProcess(
                PROCESS_SET_QUOTA | PROCESS_QUERY_INFORMATION | PROCESS_VM_OPERATION,
                false,
                pid,
            )
        });

        if handle.is_invalid() {
            debug!("failed to open pid {pid}");
            return Err(windows_core::Error::from_thread().into());
        }

        let Some(h) = ProcessHandle::from_handle(handle) else {
            debug!("failed to verify that the opened process handle is actually a process handle");
            return Ok(None);
        };

        Self::from_handle(h).map(Some)
    }

    /// Find a process by its name.
    pub fn from_name(name: &str) -> Result<Option<Self>> {
        let Some(pe32) = Processes::iter()?.find(|pe32| {
            let null_idx = pe32
                .szExeFile
                .iter()
                .position(|&c| c == 0)
                .expect("no NULL terminator in szExeFile")
                .clamp(0, size_of_val(&pe32.szExeFile) - 1);
            let pname = String::from_utf16_lossy(&pe32.szExeFile[..null_idx]);

            pname.eq_ignore_ascii_case(name)
        }) else {
            debug!("failed to find process '{name}'");
            return Ok(None);
        };

        Self::from_pid(pe32.th32ProcessID)
    }

    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
