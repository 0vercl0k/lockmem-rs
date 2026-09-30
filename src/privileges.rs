// Axel '0vercl0k' Souchet - December 8th 2024
use core::slice;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::sync::{LazyLock, Mutex};

use windows_core::{PCSTR, PSTR};

use crate::bindings::{
    ERROR_NOT_ALL_ASSIGNED, SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_QUERY,
};
use crate::bindings_sys::{
    AdjustTokenPrivileges, GetCurrentProcess, GetTokenInformation, HANDLE, LUID,
    LookupPrivilegeNameA, LookupPrivilegeValueA, OpenProcessToken, TOKEN_PRIVILEGES,
    TokenPrivileges,
};
use crate::error::Error;
use crate::handle::{Handle, ProcessHandle};
use crate::utils::AlignedAlloc;
use crate::{Result, try_from, try_from_usize};

const SE_TIME_ZONE_PRIVILEGE: &CStr = c"SeTimeZonePrivilege";
const SE_IMPERSONATE_PRIVILEGE: &CStr = c"SeImpersonatePrivilege";
const SE_INCREASE_BASE_PRIORITY_PRIVILEGE: &CStr = c"SeIncreaseBasePriorityPrivilege";
const SE_DEBUG_PRIVILEGE: &CStr = c"SeDebugPrivilege";
const SE_SYSTEM_ENVIRONMENT_PRIVILEGE: &CStr = c"SeSystemEnvironmentPrivilege";
const SE_CHANGE_NOTIFY_PRIVILEGE: &CStr = c"SeChangeNotifyPrivilege";
const SE_CREATE_PAGEFILE_PRIVILEGE: &CStr = c"SeCreatePagefilePrivilege";
const SE_BACKUP_PRIVILEGE: &CStr = c"SeBackupPrivilege";
const SE_UNDOCK_PRIVILEGE: &CStr = c"SeUndockPrivilege";
const SE_CREATE_GLOBAL_PRIVILEGE: &CStr = c"SeCreateGlobalPrivilege";
const SE_INCREASE_WORKING_SET_PRIVILEGE: &CStr = c"SeIncreaseWorkingSetPrivilege";
const SE_INCREASE_QUOTA_PRIVILEGE: &CStr = c"SeIncreaseQuotaPrivilege";
const SE_SECURITY_PRIVILEGE: &CStr = c"SeSecurityPrivilege";
const SE_SYSTEM_PROFILE_PRIVILEGE: &CStr = c"SeSystemProfilePrivilege";
const SE_SYSTEMTIME_PRIVILEGE: &CStr = c"SeSystemtimePrivilege";
const SE_LOAD_DRIVER_PRIVILEGE: &CStr = c"SeLoadDriverPrivilege";
const SE_SHUTDOWN_PRIVILEGE: &CStr = c"SeShutdownPrivilege";
const SE_MANAGE_VOLUME_PRIVILEGE: &CStr = c"SeManageVolumePrivilege";
const SE_PROFILE_SINGLE_PROCESS_PRIVILEGE: &CStr = c"SeProfileSingleProcessPrivilege";
const SE_CREATE_SYMBOLIC_LINK_PRIVILEGE: &CStr = c"SeCreateSymbolicLinkPrivilege";
const SE_DELEGATE_SESSION_USER_IMPERSONATE_PRIVILEGE: &CStr =
    c"SeDelegateSessionUserImpersonatePrivilege";
const SE_REMOTE_SHUTDOWN_PRIVILEGE: &CStr = c"SeRemoteShutdownPrivilege";
const SE_TAKE_OWNERSHIP_PRIVILEGE: &CStr = c"SeTakeOwnershipPrivilege";
const SE_RESTORE_PRIVILEGE: &CStr = c"SeRestorePrivilege";

const PRIVILEGES: &[&CStr] = &[
    SE_TIME_ZONE_PRIVILEGE,
    SE_IMPERSONATE_PRIVILEGE,
    SE_INCREASE_BASE_PRIORITY_PRIVILEGE,
    SE_DEBUG_PRIVILEGE,
    SE_SYSTEM_ENVIRONMENT_PRIVILEGE,
    SE_CHANGE_NOTIFY_PRIVILEGE,
    SE_CREATE_PAGEFILE_PRIVILEGE,
    SE_BACKUP_PRIVILEGE,
    SE_UNDOCK_PRIVILEGE,
    SE_CREATE_GLOBAL_PRIVILEGE,
    SE_INCREASE_WORKING_SET_PRIVILEGE,
    SE_INCREASE_QUOTA_PRIVILEGE,
    SE_SECURITY_PRIVILEGE,
    SE_SYSTEM_PROFILE_PRIVILEGE,
    SE_SYSTEMTIME_PRIVILEGE,
    SE_LOAD_DRIVER_PRIVILEGE,
    SE_SHUTDOWN_PRIVILEGE,
    SE_MANAGE_VOLUME_PRIVILEGE,
    SE_PROFILE_SINGLE_PROCESS_PRIVILEGE,
    SE_CREATE_SYMBOLIC_LINK_PRIVILEGE,
    SE_DELEGATE_SESSION_USER_IMPERSONATE_PRIVILEGE,
    SE_REMOTE_SHUTDOWN_PRIVILEGE,
    SE_TAKE_OWNERSHIP_PRIVILEGE,
    SE_RESTORE_PRIVILEGE,
];

pub(crate) static PRIVILEGE_MANAGER: Mutex<LazyLock<PrivilegeManager>> =
    Mutex::new(LazyLock::new(|| PrivilegeManager::current().unwrap()));

static NAME_TO_LUIDS: LazyLock<HashMap<&CStr, LUID>> = LazyLock::new(|| {
    let mut name_to_luids = HashMap::with_capacity(PRIVILEGES.len());
    for &p in PRIVILEGES {
        let mut luid = LUID::default();
        unsafe {
            LookupPrivilegeValueA(
                PCSTR::null(),
                PCSTR::from_raw(p.to_bytes_with_nul().as_ptr()),
                &raw mut luid,
            )
        }
        .unwrap();
        name_to_luids.insert(p, luid);
    }

    name_to_luids
});

#[derive(Debug, Default)]
pub(crate) struct PrivilegeManager {
    token: Handle,
    privileges: HashMap<CString, bool>,
}

impl PrivilegeManager {
    fn current() -> Result<Self> {
        const MAX_PRIVILEGE_NAME_LEN: usize = 64;
        let process =
            ProcessHandle::from_handle(Handle::wrap(unsafe { GetCurrentProcess() })).unwrap();
        let mut token = HANDLE::default();
        unsafe {
            OpenProcessToken(
                process.as_raw(),
                TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
                &raw mut token,
            )
        }
        .ok()
        .map_err(|e| Error::win32(format!("OpenProcessToken(p={process})"), e))?;

        let token = Handle::adopt(token);
        let mut needed = 0;
        unsafe { GetTokenInformation(token.as_raw(), TokenPrivileges, None, 0, &raw mut needed) }
            .ok()
            .map_err(|e| Error::win32(format!("GetTokenInformation1(tok={token}"), e))?;

        let mut info = AlignedAlloc::<TOKEN_PRIVILEGES>::new(try_from_usize!(needed));
        let mut written = 0;
        unsafe {
            GetTokenInformation(
                token.as_raw(),
                TokenPrivileges,
                Some(info.as_mut_ptr().cast()),
                needed,
                &raw mut written,
            )
        }
        .ok()
        .map_err(|e| Error::win32(format!("GetTokenInformation2(tok={token})"), e))?;

        if written != needed {
            return Err("size changed in between the two GetTokenInformation calls".into());
        }

        let p = info.as_ptr();
        let count = unsafe { (*p).PrivilegeCount }.try_into()?;
        let luids = unsafe { slice::from_raw_parts((*p).Privileges.as_ptr(), count) };

        let mut privileges = HashMap::new();
        for luid in luids {
            let mut name_len = 0;
            unsafe { LookupPrivilegeNameA(None, &raw const luid.Luid, None, &raw mut name_len) }
                .ok()
                .map_err(|e| {
                    Error::win32(
                        format!(
                            "LookupPrivilegeNameA1(luid={:#x}{:#x}",
                            luid.Luid.HighPart, luid.Luid.LowPart
                        ),
                        e,
                    )
                })?;

            let name_len = try_from_usize!(name_len).clamp(1, MAX_PRIVILEGE_NAME_LEN);
            let mut name = vec![0u8; name_len];
            let mut name_len_u32 = try_from!(u32, name_len);
            unsafe {
                LookupPrivilegeNameA(
                    None,
                    &raw const luid.Luid,
                    Some(PSTR::from_raw(name.as_mut_ptr())),
                    &raw mut name_len_u32,
                )
            }
            .ok()
            .map_err(|e| {
                Error::win32(
                    format!(
                        "LookupPrivilegeNameA2(luid={:#x}{:#x}",
                        luid.Luid.HighPart, luid.Luid.LowPart
                    ),
                    e,
                )
            })?;

            let name = CString::from_vec_with_nul(name)?;
            let enabled = (luid.Attributes & SE_PRIVILEGE_ENABLED) != 0;
            privileges.insert(name, enabled);
        }

        Ok(Self { token, privileges })
    }

    pub(crate) fn enable(&mut self, p: &CStr) -> Result<()> {
        let luid = NAME_TO_LUIDS.get(p).unwrap();
        let mut privs = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            ..Default::default()
        };

        privs.Privileges[0].Luid = *luid;
        privs.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;

        unsafe {
            AdjustTokenPrivileges(
                self.token.as_raw(),
                false,
                Some(&raw const privs),
                try_from!(u32, size_of_val(&privs)),
                None,
                None,
            )
        }
        .ok()
        .map_err(|e| {
            Error::win32(
                format!(
                    "AdjustTokenPrivileges(tok={}, luid={:#x}{:#x})",
                    self.token, luid.HighPart, luid.LowPart
                ),
                e,
            )
        })?;

        let e = windows_core::WIN32_ERROR::from_thread();
        if e.0 == ERROR_NOT_ALL_ASSIGNED {
            return Err(Error::win32(
                format!(
                    "AdjustTokenPrivileges(tok={}, luid={:#x}{:#x}) succedded but privs ere not assigned; are you admin?",
                    self.token, luid.HighPart, luid.LowPart
                ),
                e.into(),
            ));
        }

        self.privileges
            .entry(p.into())
            .and_modify(|e| *e = true)
            .or_insert(true);

        Ok(())
    }

    pub(crate) fn set_sedebug(&mut self) -> Result<()> {
        if self.sedebug() {
            return Ok(());
        }

        self.enable(SE_DEBUG_PRIVILEGE)
    }

    pub(crate) fn is_held(&self, p: &CStr) -> bool {
        if let Some(&enabled) = self.privileges.get(p) {
            enabled
        } else {
            false
        }
    }

    pub(crate) fn sedebug(&self) -> bool {
        self.is_held(SE_DEBUG_PRIVILEGE)
    }
}
