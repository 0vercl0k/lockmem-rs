// Axel '0vercl0k' Souchet - July 6 2026
#[cfg(not(all(target_os = "windows", target_pointer_width = "64")))]
compile_error!("This binary can only be built on windows");

mod bindings;
mod bindings_sys;
mod error;
mod handle;
mod human;
mod privileges;
mod process;
mod utils;

use std::io::Write;
use std::num::Saturating;
use std::sync::LazyLock;
use std::time::{Duration, Instant};
use std::{env, io};

use error::Result;
use log::{debug, trace};
use process::Process;

use crate::bindings::{MEM_FREE, MEM_RESERVE, PAGE_GUARD, PAGE_NOACCESS};
use crate::bindings_sys::E_ACCESSDENIED;
use crate::error::Error;
use crate::human::ToHuman;
use crate::privileges::PRIVILEGE_MANAGER;
use crate::utils::turn_on_vt;

const SPINNER: [&str; 4] = ["◐", "◓", "◑", "◒"];

static VT_ON: LazyLock<bool> = LazyLock::new(|| turn_on_vt().is_ok());

fn lockmem(p: &Process) -> Result<()> {
    let (prefix, suffix) = if *VT_ON {
        // "\x1b[2K" is an ANSI escape sequence that clears the current line
        // w/o moving the cursor. "\r" is used to move the
        // cursor back at the start.
        ("\x1b[2K\r", "")
    } else {
        // If we couldn't turn on virtual terminal processing, we'll stick with
        // one update per line.
        ("", "\n")
    };

    let mut amount = Saturating(0);
    let mut spinner = 0;
    let mut last_update = Instant::now().checked_sub(Duration::from_mins(1)).unwrap();
    for m in p.iter_mem()? {
        let start = try_from!(u64, m.BaseAddress.addr());
        let end = start.saturating_add(try_from!(u64, m.RegionSize));
        let range = start..end;

        if (m.State & (MEM_FREE | MEM_RESERVE)) != 0 {
            let desc = if (m.State & MEM_FREE) != 0 {
                "MEM_FREE"
            } else {
                "MEM_RESERVE"
            };

            trace!("{range:#x?} is in a {desc} state, so skipping");
            continue;
        }

        if (m.Protect & (PAGE_GUARD | PAGE_NOACCESS)) != 0 {
            let desc = if (m.Protect & PAGE_GUARD) != 0 {
                "PAGE_GUARD"
            } else {
                "PAGE_NOACCESS"
            };

            trace!("{range:#x?} is a {desc} region, so skipping");
            continue;
        }

        debug!("Handling {range:#x?}");
        amount += range.end - range.start;
        p.grown_and_lock_mem(range.into())?;

        if last_update.elapsed().as_secs() >= 1 {
            let mut stdout = io::stdout();
            write!(
                stdout,
                "{}{} {}/{}: Locked {}{}",
                prefix,
                SPINNER[spinner % SPINNER.len()],
                p.name(),
                p.pid(),
                amount.0.human_bytes(),
                suffix
            )
            .map_err(|e| Error::from(format!("writing to stdout failed w/ {e}")))?;

            spinner += 1;
            stdout
                .flush()
                .map_err(|e| Error::from(format!("flushing stdout failed w/ {e}")))?;
            last_update = Instant::now();
        }
    }

    println!(
        "\x1b[2K\r✓ {}/{}: Locked {}",
        p.name(),
        p.pid(),
        amount.0.human_bytes()
    );

    Ok(())
}

fn open_and_lockmem(pid_or_name: &str) -> Result<()> {
    let p = match pid_or_name.parse::<u32>() {
        Ok(pid) => Process::from_pid(pid),
        Err(_) => Process::from_name(pid_or_name),
    }?;

    match p {
        None => Err("no process found".into()),
        Some(p) => lockmem(&p),
    }
}

fn main() {
    #[cfg(debug_assertions)]
    env_logger::init();

    let mut args = env::args();
    if args.len() <= 1 {
        println!("./lockmem-rs.exe <name | pid>");
        return;
    }

    let pid_or_name = args.nth(1).unwrap();
    match open_and_lockmem(&pid_or_name) {
        Ok(()) => {}
        Err(Error::Win32(e)) => {
            if e.code() == E_ACCESSDENIED {
                println!("got access denied.. trying to get SeDebugPrivilege..");
                match PRIVILEGE_MANAGER.lock().unwrap().set_sedebug() {
                    Ok(()) => {
                        println!("got SeDebugPrivilege, trying again..");
                        if let Err(e) = open_and_lockmem(&pid_or_name) {
                            println!("failed again w/ {e}; maybe PPL?");
                        }
                    }
                    Err(e) => {
                        println!(
                            "couldn't get SeDebugPrivilege (failed w/ {e}), try from an admin prompt?"
                        );
                    }
                }
            }
        }
        Err(e) => panic!("failed to open process: {e}"),
    }
}
