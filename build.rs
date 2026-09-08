// Axel '0vercl0k' Souchet - July 19 2026
use std::collections::BTreeSet;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

fn sort_apis(mut f: &File) -> Result<(), Box<dyn Error>> {
    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let mut offset = 0;
    let mut line_it = content.split_inclusive('\n');
    for line in &mut line_it {
        offset += line.len();
        if line.starts_with("--filter") {
            break;
        }
    }

    let mut apis = BTreeSet::new();
    for api in line_it {
        // Strips the line feed off. This is useful to treat every line the
        // same; the last one in the file won't have a line feed, so this allows
        // us to write a line feed below for every API unconditionally (not just
        // for the last one).
        apis.insert(api.trim_end());
    }

    f.set_len(offset.try_into()?)?;
    f.seek(SeekFrom::Start(offset.try_into()?))?;

    for api in apis {
        writeln!(f, "{api}")?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    const BINDINGS_TXT: &str = "bindings.txt";
    const BINDINGS_SYS_RS: &str = "src/bindings_sys.rs";
    println!("cargo::rerun-if-changed={BINDINGS_TXT}");

    sort_apis(&File::options().read(true).write(true).open(BINDINGS_TXT)?)?;
    windows_bindgen::bindgen(["--out", BINDINGS_SYS_RS, "--flat", "--etc", BINDINGS_TXT]);

    let mut bindings_sys = String::new();
    File::open(BINDINGS_SYS_RS)?.read_to_string(&mut bindings_sys)?;

    let mut f = File::options()
        .truncate(true)
        .write(true)
        .open(BINDINGS_SYS_RS)?;
    writeln!(
        f,
        "#![allow(
    nonstandard_style,
    clippy::upper_case_acronyms,
    clippy::unreadable_literal,
    clippy::ptr_as_ptr,
    clippy::cast_possible_wrap
)]"
    )?;
    writeln!(f, "{bindings_sys}")?;
    // writeln!(f, "}}")?;

    Ok(())
}
