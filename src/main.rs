use cargo_toml::{Inheritable, OptionalFile, Package};
use std::path::PathBuf;
// TODO: replace promptly with a prompt library
// that supports reusing the buffer.
use promptly::{prompt, ReadlineError};

mod fill_miscellaneous;
mod fill_rust_version;

use fill_miscellaneous::fill_miscellaneous;
use fill_rust_version::fill_rust_version;

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
    println!("Description: \"The authors of the package.\"");
    // TODO: find ways to obtain the author's info with his permission

    let authors = loop {
        let c: String = prompt(
            "Please choose the method of entering the authors.\n\
            \n\
            1. Extract a single author from git config.\n\
            2. Enter the authors manually.\n\
            ",
        )?;
        match c.as_str() {
            "1" => {
                let name: Vec<u8> = std::process::Command::new("git")
                    .args(&["config", "--get", "user.name"])
                    .output()
                    .unwrap_or_else(|e| panic!("Failed to run `git config --get user.name`: {}", e))
                    .stdout;
                let name = String::from_utf8(name).unwrap_or_else(|e| panic!("Failed to parse the result of `git config --get user.name` as a UTF-8 string: {}", e));
                let email: Vec<u8> = std::process::Command::new("git")
                    .args(&["config", "--get", "user.email"])
                    .output()
                    .unwrap_or_else(|e| {
                        panic!("Failed to run `git config --get user.email`: {}", e)
                    })
                    .stdout;
                let email = String::from_utf8(email).unwrap_or_else(|e| panic!("Failed to parse the result of `git config --get user.email` as a UTF-8 string: {}", e));
                let author = format!("{} <{}>", name.trim(), email.trim());
                println!("Extracted author: {}", author);
                if !prompt("Is this correct? (Y/n)")? {
                    continue;
                }
                break vec![author];
            }
            "2" => {
                let authors: String = prompt(
                    "Please enter comma-separated authors, e.g. `Dmitrii Demenev <demenev.dmitriy1@gmail.com>`\n",
                )?;
                let authors = authors
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<_>>();
                break authors;
            }
            _ => println!("Invalid input."),
        }
    };
    package.authors.set(authors);
    println!();
    Ok(())
}

fn fill_description(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `description` field.");
    println!("Description: \"A description of the package.\"");
    let description: String = prompt("Please enter the crate description")?;
    package.description = Some(Inheritable::Set(description));
    println!();
    Ok(())
}

fn fill_documentation(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `documentation` field.");
    println!("Description: \"The URL of the package documentation.\"");
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
    println!();
    Ok(())
}

fn fill_readme(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `readme` field.");
    println!("Description: \"Path to the package’s README file.\"");
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
    println!();
    Ok(())
}

fn fill_homepage(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `homepage` field.");
    println!("URL of the package homepage.");
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
    println!();
    Ok(())
}

fn fill_repository(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `repository` field.");
    println!("Description: \"URL of the package source repository.\"");
    let repository = loop {
        let c: String = prompt(
            "Please choose the method of entering the repository.\n\
            \n\
            1. Skip (discouraged).\n\
            2. Extract with `git config --get remote.origin.url`.\n\
            3. Enter the repository URL manually.\n\
            ",
        )?;
        match c.as_str() {
            "1" => return Ok(()),
            "2" => {
                let name: Vec<u8> = std::process::Command::new("git")
                    .args(&["config", "--get", "remote.origin.url"])
                    .output()
                    .unwrap_or_else(|e| {
                        panic!("Failed to run `git config --get remote.origin.url`: {}", e)
                    })
                    .stdout;
                let url = String::from_utf8(name).unwrap_or_else(|e| panic!("Failed to parse the result of `git config --get remote.origin.url` as a UTF-8 string: {}", e));
                let url = url
                    .trim()
                    .strip_suffix(".git")
                    .unwrap_or_else(|| panic!("Failed to strip the `.git` suffix from the result of `git config --get remote.origin.url`"));

                println!("Guessed repository: {}", url);
                if !prompt("Is this correct? (Y/n)")? {
                    continue;
                }
                break url.to_string();
            }
            "3" => {
                let url: String = prompt("Please enter the repository URL")?;
                break url;
            }
            _ => println!("Invalid input."),
        }
    };
    package.repository = Some(Inheritable::Set(repository));
    println!();
    Ok(())
}

fn fill_license(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `license` field.");
    println!("Description: \"The package license.\"");
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
    println!();
    Ok(())
}

fn fill_license_file(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `license-file` field.");
    println!("Description: \"Path to the text of the license.\"");
    let license_file = loop {
        let c: String = prompt(
            "Please choose the method of entering the license file.\n\
            \n\
            1. Default to `LICENSE`.\n\
            2. Enter the license file path manually.\n\
            3. Skip.\n\
            ",
        )?;
        match c.as_str() {
            "1" => break "LICENSE".to_string(),
            "2" => {
                let path: String = prompt("Please enter the license file path")?;
                break path;
            }
            "3" => return Ok(()),
            _ => println!("Invalid input."),
        }
    };
    let license_file = PathBuf::from(license_file);
    package.license_file = Some(Inheritable::Set(license_file));
    println!();
    Ok(())
}

fn fill_keywords(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `keywords` field.");
    println!("Description: \"The keywords of the package.\"");
    let keywords = loop {
        let c: String = prompt(
            "Please choose the method of entering the keywords.\n\
            \n\
            1. Skip.\n\
            2. Enter the keywords manually.\n\
            ",
        )?;
        match c.as_str() {
            "1" => return Ok(()),
            "2" => {
                let keywords: String = prompt("Please enter the keywords separated by comma")?;
                break keywords;
            }
            _ => println!("Invalid input."),
        }
    };
    let keywords = keywords
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();
    package.keywords = Inheritable::Set(keywords);
    println!();
    Ok(())
}

fn fill_categories(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `categories` field.");
    println!("Description: \"Categories of the package.\"");
    println!("See <https://crates.io/category_slugs> for the list of categories.");
    let categories = loop {
        let c: String = prompt(
            "Please choose the method of entering the categories.\n\
            \n\
            1. Skip.\n\
            2. Enter the categories manually.\n\
            ",
        )?;
        match c.as_str() {
            "1" => return Ok(()),
            "2" => {
                let categories: String =
                    prompt("Please enter the categories separated by comma\n")?;
                break categories;
            }
            _ => println!("Invalid input."),
        }
    };
    let categories = categories
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();
    package.categories = Inheritable::Set(categories);
    println!();
    Ok(())
}

fn main() {
    let mut manifest = read_toml();
    let package = manifest
        .package
        .as_mut()
        .expect("Cargo.toml has no package section");

    fill_authors(package).unwrap();
    fill_rust_version(package).unwrap();
    fill_description(package).unwrap();
    fill_documentation(package).unwrap();
    fill_readme(package).unwrap();
    fill_homepage(package).unwrap();
    fill_repository(package).unwrap();
    fill_license(package).unwrap();
    fill_license_file(package).unwrap();
    fill_keywords(package).unwrap();
    fill_categories(package).unwrap();
    // A bunch of fields with niche use cases.
    fill_miscellaneous(package).unwrap();

    let toml = toml::ser::to_string_pretty(&manifest).unwrap();
    println!("Cargo.toml:\n\n{}", &toml);
    if prompt("Save the changes? (Y/n)").unwrap() {
        std::fs::write("Cargo.toml", &toml).unwrap();
    }
}
