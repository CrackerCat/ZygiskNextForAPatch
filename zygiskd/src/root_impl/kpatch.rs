use std::process::{Command, Stdio};
use crate::constants::KPATCH_VER_CODE;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub enum Version {
    Supported,
    TooOld,
}

pub fn get_kpatch() -> Option<crate::root_impl::kpatch::Version> {
    let version: Option<i32> = Command::new("exec")
        .arg("/data/adb/kpatch")
        .arg("-v")
        .stdout(Stdio::piped())
        .spawn().ok()
        .and_then(|child| child.wait_with_output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .and_then(|output| output.trim().parse().ok());
    version.map(|version| {
        if version >= KPATCH_VER_CODE {
            Version::Supported
        } else {
            Version::TooOld
        }
    })
}

fn read_package_config() -> Result<Vec<Vec<String>>, std::io::Error> {
    let file = File::open("/data/adb/ap/package_config")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
    Ok(lines.iter().map(|line| line.split(",").map(String::from).collect()).collect())
}

pub fn uid_granted_root(uid: i32) -> bool {
    let package_config = read_package_config().unwrap_or_default();

    package_config.iter().any(|parts| {
        parts[3] == uid.to_string() && parts[2] == "1"
    })
}

pub fn uid_should_umount(uid: i32) -> bool {
    let package_config = read_package_config().unwrap_or_default();

    let result = package_config.iter().any(|parts| {
        if parts[3] == uid.to_string() && parts[1] == "0" {
            false
        } else {
            true
        }        
    });

    result
}