#![allow(
    nonstandard_style,
    clippy::upper_case_acronyms,
    clippy::unreadable_literal,
    clippy::ptr_as_ptr,
    clippy::cast_possible_wrap
)]
#[inline]
pub unsafe fn AdjustTokenPrivileges(
    tokenhandle: HANDLE,
    disableallprivileges: bool,
    newstate: Option<*const TOKEN_PRIVILEGES>,
    bufferlength: u32,
    previousstate: Option<*mut TOKEN_PRIVILEGES>,
    returnlength: Option<*mut u32>,
) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AdjustTokenPrivileges(tokenhandle : HANDLE, disableallprivileges : windows_core::BOOL, newstate : *const TOKEN_PRIVILEGES, bufferlength : u32, previousstate : *mut TOKEN_PRIVILEGES, returnlength : *mut u32) -> windows_core::BOOL);
    unsafe {
        AdjustTokenPrivileges(
            tokenhandle,
            disableallprivileges.into(),
            newstate.unwrap_or(core::mem::zeroed()) as _,
            bufferlength,
            previousstate.unwrap_or(core::mem::zeroed()) as _,
            returnlength.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn CloseHandle(hobject: HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CloseHandle(hobject : HANDLE) -> windows_core::BOOL);
    unsafe { CloseHandle(hobject) }
}
#[inline]
pub unsafe fn CreateToolhelp32Snapshot(dwflags: u32, th32processid: u32) -> HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateToolhelp32Snapshot(dwflags : u32, th32processid : u32) -> HANDLE);
    unsafe { CreateToolhelp32Snapshot(dwflags, th32processid) }
}
#[inline]
pub unsafe fn DuplicateHandle(
    hsourceprocesshandle: HANDLE,
    hsourcehandle: HANDLE,
    htargetprocesshandle: HANDLE,
    lptargethandle: *mut HANDLE,
    dwdesiredaccess: u32,
    binherithandle: bool,
    dwoptions: u32,
) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DuplicateHandle(hsourceprocesshandle : HANDLE, hsourcehandle : HANDLE, htargetprocesshandle : HANDLE, lptargethandle : *mut HANDLE, dwdesiredaccess : u32, binherithandle : windows_core::BOOL, dwoptions : u32) -> windows_core::BOOL);
    unsafe {
        DuplicateHandle(
            hsourceprocesshandle,
            hsourcehandle,
            htargetprocesshandle,
            lptargethandle as _,
            dwdesiredaccess,
            binherithandle.into(),
            dwoptions,
        )
    }
}
#[inline]
pub unsafe fn GetConsoleMode(hconsolehandle: HANDLE, lpmode: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleMode(hconsolehandle : HANDLE, lpmode : *mut u32) -> windows_core::BOOL);
    unsafe { GetConsoleMode(hconsolehandle, lpmode as _) }
}
#[inline]
pub unsafe fn GetCurrentProcess() -> HANDLE {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentProcess() -> HANDLE);
    unsafe { GetCurrentProcess() }
}
#[inline]
pub unsafe fn GetLastError() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetLastError() -> u32);
    unsafe { GetLastError() }
}
#[inline]
pub unsafe fn GetProcessId(process: HANDLE) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetProcessId(process : HANDLE) -> u32);
    unsafe { GetProcessId(process) }
}
#[inline]
pub unsafe fn GetProcessWorkingSetSizeEx(
    hprocess: HANDLE,
    lpminimumworkingsetsize: *mut usize,
    lpmaximumworkingsetsize: *mut usize,
    flags: *mut u32,
) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessWorkingSetSizeEx(hprocess : HANDLE, lpminimumworkingsetsize : *mut usize, lpmaximumworkingsetsize : *mut usize, flags : *mut u32) -> windows_core::BOOL);
    unsafe {
        GetProcessWorkingSetSizeEx(
            hprocess,
            lpminimumworkingsetsize as _,
            lpmaximumworkingsetsize as _,
            flags as _,
        )
    }
}
#[inline]
pub unsafe fn GetStdHandle(nstdhandle: u32) -> HANDLE {
    windows_core::link!("kernel32.dll" "system" fn GetStdHandle(nstdhandle : u32) -> HANDLE);
    unsafe { GetStdHandle(nstdhandle) }
}
#[inline]
pub unsafe fn GetTokenInformation(
    tokenhandle: HANDLE,
    tokeninformationclass: TOKEN_INFORMATION_CLASS,
    tokeninformation: Option<*mut core::ffi::c_void>,
    tokeninformationlength: u32,
    returnlength: *mut u32,
) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetTokenInformation(tokenhandle : HANDLE, tokeninformationclass : TOKEN_INFORMATION_CLASS, tokeninformation : *mut core::ffi::c_void, tokeninformationlength : u32, returnlength : *mut u32) -> windows_core::BOOL);
    unsafe {
        GetTokenInformation(
            tokenhandle,
            tokeninformationclass,
            tokeninformation.unwrap_or(core::mem::zeroed()) as _,
            tokeninformationlength,
            returnlength as _,
        )
    }
}
#[inline]
pub unsafe fn LookupPrivilegeNameA<P0>(
    lpsystemname: P0,
    lpluid: *const LUID,
    lpname: Option<windows_core::PSTR>,
    cchname: *mut u32,
) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn LookupPrivilegeNameA(lpsystemname : windows_core::PCSTR, lpluid : *const LUID, lpname : windows_core::PSTR, cchname : *mut u32) -> windows_core::BOOL);
    unsafe {
        LookupPrivilegeNameA(
            lpsystemname.param().abi(),
            lpluid,
            lpname.unwrap_or(core::mem::zeroed()) as _,
            cchname as _,
        )
    }
}
#[inline]
pub unsafe fn LookupPrivilegeValueA<P0, P1>(
    lpsystemname: P0,
    lpname: P1,
    lpluid: *mut LUID,
) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn LookupPrivilegeValueA(lpsystemname : windows_core::PCSTR, lpname : windows_core::PCSTR, lpluid : *mut LUID) -> windows_core::BOOL);
    unsafe {
        LookupPrivilegeValueA(
            lpsystemname.param().abi(),
            lpname.param().abi(),
            lpluid as _,
        )
    }
}
#[inline]
pub unsafe fn NtQueryObject(
    handle: Option<HANDLE>,
    objectinformationclass: OBJECT_INFORMATION_CLASS,
    objectinformation: Option<*mut core::ffi::c_void>,
    objectinformationlength: u32,
    returnlength: Option<*mut u32>,
) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryObject(handle : HANDLE, objectinformationclass : OBJECT_INFORMATION_CLASS, objectinformation : *mut core::ffi::c_void, objectinformationlength : u32, returnlength : *mut u32) -> windows_core::NTSTATUS);
    unsafe {
        NtQueryObject(
            handle.unwrap_or(core::mem::zeroed()) as _,
            objectinformationclass,
            objectinformation.unwrap_or(core::mem::zeroed()) as _,
            objectinformationlength,
            returnlength.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn OpenProcess(dwdesiredaccess: u32, binherithandle: bool, dwprocessid: u32) -> HANDLE {
    windows_core::link!("kernel32.dll" "system" fn OpenProcess(dwdesiredaccess : u32, binherithandle : windows_core::BOOL, dwprocessid : u32) -> HANDLE);
    unsafe { OpenProcess(dwdesiredaccess, binherithandle.into(), dwprocessid) }
}
#[inline]
pub unsafe fn OpenProcessToken(
    processhandle: HANDLE,
    desiredaccess: u32,
    tokenhandle: *mut HANDLE,
) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn OpenProcessToken(processhandle : HANDLE, desiredaccess : u32, tokenhandle : *mut HANDLE) -> windows_core::BOOL);
    unsafe { OpenProcessToken(processhandle, desiredaccess, tokenhandle as _) }
}
#[inline]
pub unsafe fn Process32FirstW(hsnapshot: HANDLE, lppe: *mut PROCESSENTRY32W) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Process32FirstW(hsnapshot : HANDLE, lppe : *mut PROCESSENTRY32W) -> windows_core::BOOL);
    unsafe { Process32FirstW(hsnapshot, lppe as _) }
}
#[inline]
pub unsafe fn Process32NextW(hsnapshot: HANDLE, lppe: *mut PROCESSENTRY32W) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Process32NextW(hsnapshot : HANDLE, lppe : *mut PROCESSENTRY32W) -> windows_core::BOOL);
    unsafe { Process32NextW(hsnapshot, lppe as _) }
}
#[inline]
pub unsafe fn QueryFullProcessImageNameW(
    hprocess: HANDLE,
    dwflags: u32,
    lpexename: windows_core::PWSTR,
    lpdwsize: *mut u32,
) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryFullProcessImageNameW(hprocess : HANDLE, dwflags : u32, lpexename : windows_core::PWSTR, lpdwsize : *mut u32) -> windows_core::BOOL);
    unsafe { QueryFullProcessImageNameW(hprocess, dwflags, lpexename, lpdwsize as _) }
}
#[inline]
pub unsafe fn SetConsoleMode(hconsolehandle: HANDLE, dwmode: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleMode(hconsolehandle : HANDLE, dwmode : u32) -> windows_core::BOOL);
    unsafe { SetConsoleMode(hconsolehandle, dwmode) }
}
#[inline]
pub unsafe fn SetProcessWorkingSetSizeEx(
    hprocess: HANDLE,
    dwminimumworkingsetsize: usize,
    dwmaximumworkingsetsize: usize,
    flags: u32,
) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessWorkingSetSizeEx(hprocess : HANDLE, dwminimumworkingsetsize : usize, dwmaximumworkingsetsize : usize, flags : u32) -> windows_core::BOOL);
    unsafe {
        SetProcessWorkingSetSizeEx(
            hprocess,
            dwminimumworkingsetsize,
            dwmaximumworkingsetsize,
            flags,
        )
    }
}
#[inline]
pub unsafe fn VirtualQueryEx(
    hprocess: HANDLE,
    lpaddress: Option<*const core::ffi::c_void>,
    lpbuffer: *mut MEMORY_BASIC_INFORMATION,
    dwlength: usize,
) -> usize {
    windows_core::link!("kernel32.dll" "system" fn VirtualQueryEx(hprocess : HANDLE, lpaddress : *const core::ffi::c_void, lpbuffer : *mut MEMORY_BASIC_INFORMATION, dwlength : usize) -> usize);
    unsafe {
        VirtualQueryEx(
            hprocess,
            lpaddress.unwrap_or(core::mem::zeroed()) as _,
            lpbuffer as _,
            dwlength,
        )
    }
}
pub const DUPLICATE_SAME_ACCESS: i32 = 2;
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: i32 = 4;
pub const ERROR_NOT_ALL_ASSIGNED: i32 = 1300;
pub const E_ACCESSDENIED: windows_core::HRESULT = windows_core::HRESULT(0x80070005_u32 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HANDLE(pub *mut core::ffi::c_void);
pub const INVALID_HANDLE_VALUE: HANDLE = HANDLE(-1 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LUID {
    pub LowPart: u32,
    pub HighPart: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LUID_AND_ATTRIBUTES {
    pub Luid: LUID,
    pub Attributes: u32,
}
pub const MAX_PATH: i32 = 260;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut core::ffi::c_void,
    pub AllocationBase: *mut core::ffi::c_void,
    pub AllocationProtect: u32,
    pub RegionSize: usize,
    pub State: u32,
    pub Protect: u32,
    pub Type: u32,
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut core::ffi::c_void,
    pub AllocationBase: *mut core::ffi::c_void,
    pub AllocationProtect: u32,
    pub PartitionId: u16,
    pub RegionSize: usize,
    pub State: u32,
    pub Protect: u32,
    pub Type: u32,
}
pub const MEM_FREE: i32 = 65536;
pub const MEM_RESERVE: i32 = 8192;
pub type OBJECT_INFORMATION_CLASS = i32;
pub const ObjectTypeInformation: OBJECT_INFORMATION_CLASS = 2;
pub const PAGE_GUARD: i32 = 256;
pub const PAGE_NOACCESS: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROCESSENTRY32W {
    pub dwSize: u32,
    pub cntUsage: u32,
    pub th32ProcessID: u32,
    pub th32DefaultHeapID: usize,
    pub th32ModuleID: u32,
    pub cntThreads: u32,
    pub th32ParentProcessID: u32,
    pub pcPriClassBase: i32,
    pub dwFlags: u32,
    pub szExeFile: [u16; 260],
}
impl Default for PROCESSENTRY32W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROCESS_QUERY_INFORMATION: i32 = 1024;
pub const PROCESS_SET_QUOTA: i32 = 256;
pub const PROCESS_VM_OPERATION: i32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PUBLIC_OBJECT_TYPE_INFORMATION {
    pub TypeName: UNICODE_STRING,
    pub Reserved: [u32; 22],
}
impl Default for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const QUOTA_LIMITS_HARDWS_MAX_DISABLE: i32 = 8;
pub const QUOTA_LIMITS_HARDWS_MIN_ENABLE: i32 = 1;
pub const SE_PRIVILEGE_ENABLED: i32 = 2;
pub const STATUS_INCOMPATIBLE_FILE_MAP: windows_core::NTSTATUS =
    windows_core::NTSTATUS(0xC000004D_u32 as _);
pub const STATUS_INFO_LENGTH_MISMATCH: windows_core::NTSTATUS =
    windows_core::NTSTATUS(0xC0000004_u32 as _);
pub const STATUS_SUCCESS: windows_core::NTSTATUS = windows_core::NTSTATUS(0x0_u32 as _);
pub const STATUS_WAS_LOCKED: windows_core::NTSTATUS = windows_core::NTSTATUS(0x40000019_u32 as _);
pub const STATUS_WORKING_SET_QUOTA: windows_core::NTSTATUS =
    windows_core::NTSTATUS(0xC00000A1_u32 as _);
pub const STD_OUTPUT_HANDLE: u32 = 4294967285;
pub const TH32CS_SNAPPROCESS: i32 = 2;
pub const TOKEN_ADJUST_PRIVILEGES: i32 = 32;
pub type TOKEN_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_PRIVILEGES {
    pub PrivilegeCount: u32,
    pub Privileges: [LUID_AND_ATTRIBUTES; 1],
}
impl Default for TOKEN_PRIVILEGES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TOKEN_QUERY: i32 = 8;
pub const TokenPrivileges: TOKEN_INFORMATION_CLASS = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNICODE_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: windows_core::PWSTR,
}

