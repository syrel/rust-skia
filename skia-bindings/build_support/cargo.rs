//! Support function for communicating with cargo's variables and outputs.

#![allow(dead_code)]

use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::{env, fmt, fs, io};

pub fn warning(warn: impl AsRef<str>) {
    println!("cargo:warning={}", warn.as_ref());
}

pub fn output_directory() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

pub fn rerun_if_changed(path: impl AsRef<Path>) {
    println!("cargo:rerun-if-changed={}", path.as_ref().to_str().unwrap());
}

/// Returns the value of an environment variable and notify cargo that the build
/// should rereun if it changes.
pub fn env_var(name: impl AsRef<str>) -> Option<String> {
    let name = name.as_ref();
    rerun_if_env_changed(name);
    env::var(name).ok()
}

/// Notify cargo that it should rerun the build if the environment
/// variable changes.
pub fn rerun_if_env_changed(name: impl AsRef<str>) {
    println!("cargo:rerun-if-env-changed={}", name.as_ref())
}

pub fn add_link_libs(libs: &[impl AsRef<str>]) {
    libs.iter().for_each(|s| add_link_lib(s.as_ref()))
}

pub fn add_link_lib(lib: impl AsRef<str>) {
    println!("cargo:rustc-link-lib={}", lib.as_ref());
}

pub fn add_link_search(dir: impl AsRef<str>) {
    println!("cargo:rustc-link-search={}", dir.as_ref());
}

#[derive(Clone, Debug)]
pub struct Target {
    pub architecture: String,
    pub vendor: String,
    pub system: String,
    pub abi: Option<String>,
}

impl Target {
    pub fn is_windows(&self) -> bool {
        self.system == "windows"
    }

    /// Convert a library name to a filename.
    pub fn library_to_filename(&self, name: impl AsRef<str>) -> PathBuf {
        let name = name.as_ref();
        if self.is_windows() {
            format!("{}.lib", name).into()
        } else {
            format!("lib{}.a", name).into()
        }
    }

    pub fn as_strs(&self) -> (&str, &str, &str, Option<&str>) {
        (
            self.architecture.as_str(),
            self.vendor.as_str(),
            self.system.as_str(),
            self.abi.as_deref(),
        )
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}",
            &self.architecture, &self.vendor, &self.system
        )?;

        if let Some(ref abi) = self.abi {
            write!(f, "-{}", abi)
        } else {
            Result::Ok(())
        }
    }
}

pub fn target() -> Target {
    let target_str = env::var("TARGET").unwrap();
    parse_target(target_str)
}

pub fn target_crt_static() -> bool {
    cfg!(target_feature = "crt-static")
}

pub fn host() -> Target {
    let host_str = env::var("HOST").unwrap();
    println!("HOST: {}", host_str);
    parse_target(host_str)
}

fn parse_target(target_str: impl AsRef<str>) -> Target {
    let target_str = target_str.as_ref();
    let target: Vec<String> = target_str.split('-').map(|s| s.into()).collect();
    if target.len() < 3 {
        panic!("Failed to parse TARGET {}", target_str);
    }

    let abi = if target.len() > 3 {
        Some(target[3].clone())
    } else {
        None
    };

    Target {
        architecture: target[0].clone(),
        vendor: target[1].clone(),
        system: target[2].clone(),
        abi,
    }
}

/// Returns `true` if the target should be built in release mode, `false`, if in debug mode.
///
/// We can not assume that the build profile of the build.rs script reflects the build
/// profile that the target needs.
pub fn build_release() -> bool {
    match env::var("PROFILE").unwrap().as_str() {
        "release" => true,
        "debug" => false,
        profile => panic!(
            "PROFILE '{}' is not supported by this build script",
            profile
        ),
    }
}

/// Are we inside a crate?
pub fn is_crate() -> bool {
    crate_repository_hash().is_ok()
}

// If we are building from within a crate, return the full commit hash
// of the repository the crate was packaged from.
pub fn crate_repository_hash() -> io::Result<String> {
    let vcs_info = fs::read_to_string(".cargo_vcs_info.json")?;
    let value: serde_json::Value = serde_json::from_str(&vcs_info)?;
    let git = value.get("git").expect("failed to get 'git' property");
    let sha1 = git.get("sha1").expect("failed to get 'sha1' property");
    Ok(sha1.as_str().unwrap().into())
}

pub fn package_version() -> String {
    env::var("CARGO_PKG_VERSION").unwrap().as_str().into()
}

/// Parses Cargo.toml and returns the metadadata specifed in the
/// [package.metadata] section.
pub fn get_metadata() -> Vec<(String, String)> {
    use toml::{de, value};

    let cargo_toml = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("missing environment variable CARGO_MANIFEST_DIR"),
    )
    .join("Cargo.toml");
    let str = fs::read_to_string(cargo_toml).expect("Failed to read Cargo.toml");
    let root: value::Table =
        de::from_str::<value::Table>(&str).expect("Failed to parse Cargo.toml");
    let manifest_table: &value::Table = root
        .get("package")
        .expect("section [package] missing")
        .get("metadata")
        .expect("section [package.metadata] missing")
        .as_table()
        .unwrap();

    manifest_table
        .iter()
        .map(|(a, b)| (a.clone(), b.as_str().unwrap().to_owned()))
        .collect()
}
