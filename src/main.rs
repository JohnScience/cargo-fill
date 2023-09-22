use std::path::PathBuf;

use cargo_toml::OptionalFile;
use cargo_toml::Package;
// TODO: replace promptly with a prompt library
// that supports reusing the buffer.
use cargo_toml::Inheritable;
use promptly::{prompt, ReadlineError};

mod fill_rust_version;

use fill_rust_version::fill_rust_version;

#[allow(dead_code)]
const DEFAULT_CARGO_TOML: &'static str = r#"
[package]
name = "cargo-fill"
version = "0.1.0"
edition = "2021"
"#;

#[allow(dead_code)]
const FILLED_CARGO_TOML: &'static str = r#"
[package]
name = "maybe-owned-trait"
version = "0.2.1"
authors = ["Dmitrii Demenev <demenev.dmitriy1@gmail.com>"]
edition = "2021"
# rust-version
description = "Either an owned or borrowed value, with type known at compile time."
documentation = "https://docs.rs/maybe-owned-trait"
readme = "README.md"
# homepage = ""
repository = "https://github.com/JohnScience/maybe-owned-trait"
license = "MIT OR Apache-2.0"
# license-file | not needed
keywords = ["cow", "owned", "borrowed"]
categories = ["rust-patterns"]
# workspace | not needed
# build | not needed
# links | not needed
# exclude | not needed
# include | not needed
# publish | not needed
# metadata | not needed
# default-run | not needed
# autobins | not needed
# autoexamples | not needed
# autotests | not needed
# autobenches | not needed
# resolver | not needed
"#;

fn read_toml() -> cargo_toml::Manifest {
    let mut cur_dir = std::env::current_dir()
        .unwrap_or_else(|e| panic!("Failed to get the current directory: {}", e));
    cur_dir.push("Cargo.toml");
    let toml = std::fs::read_to_string(cur_dir)
        .unwrap_or_else(|e| panic!("Failed to read Cargo.toml: {}", e));
    toml::from_str(&toml).unwrap_or_else(|e| panic!("Failed to parse Cargo.toml: {}", e))
}

fn fill_authors(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `authors` field.");
    // TODO: find ways to obtain the author's info with his permission

    let authors: String =
        prompt("Please enter your info, e.g. `Dmitrii Demenev <demenev.dmitriy1@gmail.com>`")?;
    package.authors.set(vec![authors]);
    Ok(())
}

fn fill_description(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `description` field.");
    let description: String = prompt("Please enter the crate description")?;
    package.description = Some(Inheritable::Set(description));
    Ok(())
}

fn fill_documentation(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `documentation` field.");
    let documentation = loop {
        let c: String = prompt(format!(
            "Please choose the method of entering the documentation.\n\
            \n\
            1. Default to `https://docs.rs/{package_name}`.\n\
            2. Enter the documentation URL manually.\n\
            3. Skip (suggested for binary crates).\n\
        ",
            package_name = package.name
        ))?;
        match c.as_str() {
            "1" => break format!("https://docs.rs/{}", package.name),
            "2" => {
                let url: String = prompt("Please enter the documentation URL")?;
                break url;
            }
            "3" => return Ok(()),
            _ => println!("Invalid input."),
        }
    };
    package.documentation = Some(Inheritable::Set(documentation));
    Ok(())
}

fn fill_readme(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `readme` field.");
    let readme = loop {
        let c: String = prompt(format!(
            "Please choose the method of entering the README.\n\
            \n\
            1. Default to `README.md`.\n\
            2. Enter the README path manually.\n\
            3. Skip (discouraged).\n\
        ",
        ))?;
        match c.as_str() {
            "1" => break "README.md".to_string(),
            "2" => {
                let path: String = prompt("Please enter the README path")?;
                break path;
            }
            "3" => return Ok(()),
            _ => println!("Invalid input."),
        }
    };
    let readme = OptionalFile::Path(PathBuf::from(readme));
    package.readme = Inheritable::Set(readme);
    Ok(())
}

fn fill_homepage(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `homepage` field.");
    loop {
        let c: String = prompt(format!(
            "Please choose the method of entering the homepage.\n\
            \n\
            1. Skip.\n\
            2. Enter the homepage URL manually.\n\
        "
        ))?;
        match c.as_str() {
            "1" => break,
            "2" => {
                let url: String = prompt("Please enter the homepage URL")?;
                package.homepage = Some(Inheritable::Set(url));
                break;
            }
            _ => println!("Invalid input."),
        }
    }
    Ok(())
}

fn fill_repository(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `repository` field.");
    let repository = loop {
        let c: String = prompt(
            "Please choose the method of entering the repository.\n\
            \n\
            1. Skip (discouraged).\n\
            2. Enter the repository URL manually.\n\
            ",
        )?;
        match c.as_str() {
            "1" => return Ok(()),
            "2" => {
                let url: String = prompt("Please enter the repository URL")?;
                break url;
            }
            _ => println!("Invalid input."),
        }
    };
    package.repository = Some(Inheritable::Set(repository));
    Ok(())
}

fn fill_license(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `license` field.");
    let license = loop {
        let c: String = prompt(
            "Please choose the method of entering the license.\n\
            \n\
            1. Default to `MIT OR Apache-2.0` (permissive).\n\
            2. Enter the license SPDX identifier manually.\n\
            ",
        )?;
        match c.as_str() {
            "1" => return Ok(()),
            "2" => {
                let license: String = prompt("Please enter the license")?;
                break license;
            }
            _ => println!("Invalid input."),
        }
    };
    package.license = Some(Inheritable::Set(license));
    Ok(())
}

fn main() {
    let mut manifest = read_toml();
    let package = manifest
        .package
        .as_mut()
        .expect("Cargo.toml has no package section");

    // fill_authors(package).unwrap();
    // fill_rust_version(package).unwrap();
    // fill_description(package).unwrap();
    // fill_documentation(package).unwrap();
    // fill_readme(package).unwrap();
    // fill_homepage(package).unwrap();
    // fill_repository(package).unwrap();
    // fill_license(package).unwrap();

    println!(
        "Cargo.toml:\n\n{}",
        toml::ser::to_string_pretty(&manifest).unwrap()
    );
}
