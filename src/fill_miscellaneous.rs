use std::path::PathBuf;

use cargo_toml::{Inheritable, OptionalFile, Package, Publish, Resolver};
use promptly::{prompt, ReadlineError};

fn fill_workplace(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `workspace` field.");
    println!("Description: \"Path to the workspace for the package.\"");
    let workspace = loop {
        let c: String = prompt(
            "Please choose the method of entering the workspace.\n\
            \n\
            1. Skip (recommended for non-workplaces).\n\
            2. Enter the workspace manually.\n\
            ",
        )?;
        match c.as_str() {
            "1" => return Ok(()),
            "2" => {
                let workspace: String = prompt("Please enter the workspace")?;
                break workspace;
            }
            _ => println!("Invalid input."),
        }
    };
    package.workspace = Some(workspace);
    Ok(())
}

fn fill_build(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `build` field.");
    println!("Description: \"Path to the package build script.\"");
    let build = loop {
        let c: String = prompt(
            "Please choose the method of entering the path to the build script.\n\
            \n\
            1. Skip.\n\
            2. Enter the path to the build script manually.\n\
            ",
        )?;
        match c.as_str() {
            "1" => return Ok(()),
            "2" => {
                let build: String = prompt("Please enter the build")?;
                break build;
            }
            _ => println!("Invalid input."),
        }
    };
    let build = OptionalFile::Path(PathBuf::from(build));
    package.build = Some(build);
    Ok(())
}

fn fill_links(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `links` field.");
    println!("Description: \"Name of the native library the package links with.\"");
    let links = loop {
        let c: String = prompt(
            "Please choose the method of entering the name of the native library.\n\
            \n\
            1. Skip.\n\
            2. Enter the name of the native library manually.\n\
            ",
        )?;
        match c.as_str() {
            "1" => return Ok(()),
            "2" => {
                let links: String = prompt("Please enter the name of the native library")?;
                break links;
            }
            _ => println!("Invalid input."),
        }
    };
    package.links = Some(links);
    Ok(())
}

fn fill_exclude(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `exclude` field.");
    println!("Description: \"Files to exclude when publishing.\"");
    let exclude = loop {
        let c: String = prompt(
            "Please choose the method of entering the files to exclude.\n\
            \n\
            1. Skip.\n\
            2. Enter the files to exclude manually.\n\
            ",
        )?;
        match c.as_str() {
            "1" => return Ok(()),
            "2" => {
                let exclude: String =
                    prompt("Please enter the comma-separated paths to the files to exclude")?;
                break exclude;
            }
            _ => println!("Invalid input."),
        }
    };
    let exclude = exclude
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();
    package.exclude = Inheritable::Set(exclude);
    Ok(())
}

fn fill_include(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `include` field.");
    println!("Description: \"Files to include when publishing.\"");
    let include = loop {
        let c: String = prompt(
            "Please choose the method of entering the files to include.\n\
            \n\
            1. Skip.\n\
            2. Enter the files to include manually.\n\
            ",
        )?;
        match c.as_str() {
            "1" => return Ok(()),
            "2" => {
                let include: String =
                    prompt("Please enter the comma-separated paths to the files to include")?;
                break include;
            }
            _ => println!("Invalid input."),
        }
    };
    let include = include
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();
    package.include = Inheritable::Set(include);
    Ok(())
}

fn fill_publish(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `publish` field.");
    println!("Description: \"Can be used to prevent publishing the package.\"");

    let publish = loop {
        let c: String = prompt(
            "Please choose the method of entering the `publish` field.\n\
            \n\
            1. Skip.\n\
            2. Enter the `publish` field manually as a boolean.\n\
            3. Enter the `publish` field manually as a comma-separated string with registries.\n\
            ",
        )?;
        match c.as_str() {
            "1" => return Ok(()),
            "2" => {
                let publish: bool = prompt("Please enter the `publish` field")?;
                break Publish::Flag(publish);
            }
            "3" => {
                let publish: String = prompt("Please enter the comma-separated registries")?;
                let publish = publish
                    .split(",")
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<_>>();
                break Publish::Registry(publish);
            }
            _ => println!("Invalid input."),
        }
    };
    package.publish = Inheritable::Set(publish);
    Ok(())
}

fn fill_default_run(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `default-run` field.");
    println!("Description: \"The default binary to run by `cargo run`.\"");

    let default_run = loop {
        let c: String = prompt(
            "Please choose the method of entering the `default-run` field.\n\
            \n\
            1. Skip.\n\
            2. Enter the `default-run` field manually.\n\
            ",
        )?;
        match c.as_str() {
            "1" => return Ok(()),
            "2" => {
                let default_run: String = prompt("Please enter the `default-run` field")?;
                break default_run;
            }
            _ => println!("Invalid input."),
        }
    };
    package.default_run = Some(default_run);
    Ok(())
}

fn fill_autobins(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `autobins` field.");
    println!("Description: \"Disables binary auto discovery.\"");

    let autobins = loop {
        let c: String = prompt(
            "Please choose the method of entering the `autobins` field.\n\
            \n\
            1. Skip (the value defaults to `true`).\n\
            2. Disable binary auto discovery. \n\
            ",
        )?;
        match c.as_str() {
            "1" => return Ok(()),
            "2" => {
                break false;
            }
            _ => println!("Invalid input."),
        }
    };
    package.autobins = autobins;
    Ok(())
}

fn fill_autoexamples(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `autoexamples` field.");
    println!("Description: \"Disables example auto discovery.\"");

    let autoexamples = loop {
        let c: String = prompt(
            "Please choose the method of entering the `autoexamples` field.\n\
            \n\
            1. Skip (the value defaults to `true`).\n\
            2. Disable example auto discovery. \n\
            ",
        )?;
        match c.as_str() {
            "1" => return Ok(()),
            "2" => {
                break false;
            }
            _ => println!("Invalid input."),
        }
    };
    package.autoexamples = autoexamples;
    Ok(())
}

fn fill_autotests(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `autotests` field.");
    println!("Description: \"Disables test auto discovery.\"");

    let autotests = loop {
        let c: String = prompt(
            "Please choose the method of entering the `autotests` field.\n\
            \n\
            1. Skip (the value defaults to `true`).\n\
            2. Disable test auto discovery. \n\
            ",
        )?;
        match c.as_str() {
            "1" => return Ok(()),
            "2" => {
                break false;
            }
            _ => println!("Invalid input."),
        }
    };
    package.autotests = autotests;
    Ok(())
}

fn fill_autobenches(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `autobenches` field.");
    println!("Description: \"Disables bench auto discovery.\"");

    let autobenches = loop {
        let c: String = prompt(
            "Please choose the method of entering the `autobenches` field.\n\
            \n\
            1. Skip (the value defaults to `true`).\n\
            2. Disable bench auto discovery. \n\
            ",
        )?;
        match c.as_str() {
            "1" => return Ok(()),
            "2" => {
                break false;
            }
            _ => println!("Invalid input."),
        }
    };

    package.autobenches = autobenches;
    Ok(())
}

fn fill_resolver(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `resolver` field.");
    println!("Description: \"Sets the dependency resolver to use.\"");

    let resolver = loop {
        let c: String = prompt(
            "Please choose the method of entering the `resolver` field.\n\
            \n\
            1. Skip (recommended).\n\
            2. Version 1.\n\
            3. Version 2.\n\
            ",
        )?;
        match c.as_str() {
            "1" => return Ok(()),
            "2" => {
                break Resolver::V1;
            }
            "3" => {
                break Resolver::V2;
            }
            _ => println!("Invalid input."),
        }
    };
    package.resolver = Some(resolver);
    Ok(())
}

pub(crate) fn fill_miscellaneous(package: &mut Package) -> Result<(), ReadlineError> {
    // metadata is absent because it is meant to be used and filled by external tools
    println!(
        "Filling the miscellaneous fields, namely\n\
    \n\
    - workspace\n\
    - build\n\
    - links\n\
    - exclude\n\
    - include\n\
    - publish\n\
    - default-run\n\
    - autobins\n\
    - autoexamples\n\
    - autotests\n\
    - autobenches\n\
    - resolver\n\
    "
    );
    loop {
        let c: String = prompt(
            "Please choose the method of entering the miscellaneous fields.\n\
            \n\
            1. Skip.\n\
            2. Enter the miscellaneous fields manually.\n\
            ",
        )?;
        if c.as_str() == "1" {
            return Ok(());
        }
        if c.as_str() != "2" {
            println!("Invalid input.");
            continue;
        }
        break;
    }

    fill_workplace(package)?;
    fill_build(package)?;
    fill_links(package)?;
    fill_exclude(package)?;
    fill_include(package)?;
    fill_publish(package)?;
    fill_default_run(package)?;
    fill_autobins(package)?;
    fill_autoexamples(package)?;
    fill_autotests(package)?;
    fill_autobenches(package)?;
    fill_resolver(package)?;

    Ok(())
}
