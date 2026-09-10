// Axel '0vercl0k' Souchet - July 15 2026
use std::alloc::{Layout, alloc, dealloc};
use std::marker::PhantomData;

use crate::bindings::ENABLE_VIRTUAL_TERMINAL_PROCESSING;
use crate::bindings_sys::{GetConsoleMode, GetStdHandle, STD_OUTPUT_HANDLE, SetConsoleMode};
use crate::error::Result;

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

pub(crate) fn turn_on_vt() -> Result<()> {
    let h = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
    let mut mode = 0;
    if !unsafe { GetConsoleMode(h, &raw mut mode) }.as_bool() {
        return Err(format!(
            "GetConsoleMode failed w/ {}",
            windows_core::Error::from_thread()
        )
        .into());
    }

    mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
    if !unsafe { SetConsoleMode(h, mode) }.as_bool() {
        return Err(format!(
            "SetConsoleMode failed w/ {}",
            windows_core::Error::from_thread()
        )
        .into());
    }

    Ok(())
}
