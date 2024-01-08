use std::process::{Command, Stdio};
use crate::constants::KPATCH_VER_CODE;
use std::fs::File;
use std::io::{BufRead, BufReader};
use csv::Reader;
use serde::Deserialize;

pub enum Version {
    Supported,
    TooOld,
}

pub fn get_kpatch() -> Option<crate::root_impl::kpatch::Version> {
    let version: Option<i32> = Command::new("/data/adb/kpatch")
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

#[derive(Deserialize)]
struct PackageConfig {
    pkg: String,
    exclude: i32,
    allow: i32,
    uid: i32,
    to_uid: i32,
    sctx: String,
}


fn read_package_config() -> Result<Vec<PackageConfig>, std::io::Error> {
    let file = File::open("/data/adb/ap/package_config")?;
    let mut reader = csv::Reader::from_reader(file);

    let mut package_configs = Vec::new();
    for record in reader.deserialize() {
        match record {
            Ok(config) => package_configs.push(config),
            Err(error) => {
                println!("Error deserializing record: {}", error);
            }
        }
    }

    Ok(package_configs)
}

pub fn uid_granted_root(uid: i32) -> bool {
    match read_package_config() {
        Ok(package_configs) => {
            package_configs
                .iter()
                .find(|config| config.uid == uid)
                .map(|config| config.allow == 1)
                .unwrap_or(false)
        }
        Err(err) => {
            println!("Error reading package config: {}", err);
            return false;
        }
    }
}

pub fn uid_should_umount(uid: i32) -> bool {
    match read_package_config() {
        Ok(package_configs) => {
            package_configs
                .iter()
                .find(|config| config.uid == uid)
                .map(|config| {
                    if let Some(exclude) = config.exclude {
                        if exclude == 1 {
                            true
                        } else {
                            false
                        }
                    } else {
                        true
                    }
                })
                .unwrap_or(false)
        }
        Err(err) => {
            println!("Error reading package configs");
            false
        }
    }
}