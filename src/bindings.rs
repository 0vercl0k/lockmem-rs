// Axel '0vercl0k' Souchet - September 4 2026
use crate::bindings_sys;

// [This discussion](https://github.com/microsoft/windows-rs/issues/4725) talks
// about why the below is needed.
//
// The TL'DR is that those types are defined with the preprocessor in the
// Windows header and so `windows-bindgen` has no type information. It defaults
// to a signed integer like C/C++ would, but because implicit conversions don't
// happen in Rust we need to force it ourselves. The below constants are defined
// as `u32` because the functions that takes those as arguments expect `u32`s.
// So in order to not have to convert every time they are used, we redefine them
// here.
//
// There is work ongoing to address some of those issues by improving /
// annotating further the Windows headers themselves so that the parser can
// take better decisions, see https://github.com/microsoft/win32metadata/pull/2295.

macro_rules! as_u32 {
    ($($cst: ident),* ) => {
        $(
            pub const $cst: u32 = bindings_sys::$cst as u32;
        )*
    };
}

as_u32! {
    DUPLICATE_SAME_ACCESS, TH32CS_SNAPPROCESS, QUOTA_LIMITS_HARDWS_MIN_ENABLE, QUOTA_LIMITS_HARDWS_MAX_DISABLE,
    PROCESS_SET_QUOTA, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, TOKEN_QUERY,
    MEM_FREE, MEM_RESERVE, PAGE_GUARD, PAGE_NOACCESS, ENABLE_VIRTUAL_TERMINAL_PROCESSING, TOKEN_ADJUST_PRIVILEGES, SE_PRIVILEGE_ENABLED_BY_DEFAULT, SE_PRIVILEGE_ENABLED, ERROR_NOT_ALL_ASSIGNED
}
