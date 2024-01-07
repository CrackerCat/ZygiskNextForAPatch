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

pub fn uid_granted_root(uid: i32) -> bool {
    let file = File::open("/data/adb/ap/package_config").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().collect::<Vec<String>>();

    let result = lines.iter().any(|line| {
        let parts = line.split(",").collect::<Vec<&str>>();

        if parts[3] == &uid.to_string() {
            return parts[2] == "1";
        } else {
            return false;
        }
    });

    result
}



pub fn uid_should_umount(uid: i32) -> bool {
    let file = File::open("/data/adb/ap/package_config").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().collect::<Vec<String>>();

    let result = lines.iter().any(|line| {
        let parts = line.split(",").collect::<Vec<&str>>();

        if parts[3] == &uid.to_string() {
            return parts[1] == "1";
        } else {
            return true;
        }
    });

    result
}
