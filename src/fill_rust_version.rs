use std::process::{Command, Output, Stdio};

use cargo_toml::{Inheritable, Package};
use crossbeam::thread::scope;
use promptly::{prompt, ReadlineError};

const PROMPT: &'static str = r#"
Please choose the strategy to fill the `rust-version` field:

1. Skip (default).
2. Run `cargo-msrv` <https://github.com/foresterre/cargo-msrv> and fill the field with the result.
3. Enter the version manually (e.g. `1.54.0`).
"#;

struct TeeWriter<'a, W0: std::io::Write, W1: std::io::Write> {
    w0: &'a mut W0,
    w1: &'a mut W1,
}

impl<'a, W0: std::io::Write, W1: std::io::Write> TeeWriter<'a, W0, W1> {
    fn new(w0: &'a mut W0, w1: &'a mut W1) -> Self {
        Self { w0, w1 }
    }
}

impl<'a, W0: std::io::Write, W1: std::io::Write> std::io::Write for TeeWriter<'a, W0, W1> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // We have to use write_all() otherwise what happens if different
        // amounts are written?
        self.w0.write_all(buf)?;
        self.w1.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.w0.flush()?;
        self.w1.flush()?;
        Ok(())
    }
}

fn run_msrv(package: &mut Package) -> Result<(), ()> {
    let mut command = Command::new("cargo");
    // Based on question "Capture and inherit stdout and stderr using std::process::Command" on SO
    // Source: https://stackoverflow.com/questions/71141122/capture-and-inherit-stdout-and-stderr-using-stdprocesscommand
    command
        .arg("msrv")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped());

    let mut child = command.spawn().expect("failed to spawn cargo-msrv");
    // These expects should be guaranteed to be ok because we used piped().
    let mut child_stdout = child.stdout.take().expect("logic error getting stdout");
    let mut child_stderr = child.stderr.take().expect("logic error getting stderr");

    let Output {
        status,
        stdout,
        stderr,
    } = scope(|s| {
        let stdout_thread = s.spawn(|_| -> std::io::Result<Vec<u8>> {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            let mut stdout_log = Vec::<u8>::new();
            let mut tee = TeeWriter::new(&mut stdout, &mut stdout_log);
            std::io::copy(&mut child_stdout, &mut tee)?;
            Ok(stdout_log)
        });
        let stderr_thread = s.spawn(|_| -> std::io::Result<Vec<u8>> {
            let stderr = std::io::stderr();
            let mut stderr = stderr.lock();
            let mut stderr_log = Vec::<u8>::new();
            let mut tee = TeeWriter::new(&mut stderr, &mut stderr_log);

            std::io::copy(&mut child_stderr, &mut tee)?;
            Ok(stderr_log)
        });

        let status = child.wait().expect("child wasn't running");

        let stdout_log = stdout_thread
            .join()
            .expect("stdout thread panicked")
            .expect("stdout thread failed");
        let stderr_log = stderr_thread
            .join()
            .expect("stderr thread panicked")
            .expect("stderr thread failed");

        Output {
            status,
            stdout: stdout_log,
            stderr: stderr_log,
        }
    })
    .expect("stdout/stderr thread panicked");

    if !status.success() {
        eprintln!("cargo-msrv failed with status: {}", status);
        eprintln!("stderr: {}", String::from_utf8_lossy(&stderr));
        eprintln!("stdout: {}", String::from_utf8_lossy(&stdout));
        return Err(());
    };
    let stderr = String::from_utf8_lossy(&stderr);
    let msrv = {
        // E.g. "Check for toolchain '1.61.0-x86_64-pc-windows-msvc' succeeded"
        let msg = stderr
            .lines()
            .rev()
            .find(|s| s.ends_with("succeeded"))
            .unwrap();
        let (msrv, _suffix) = msg
            .strip_prefix("Check for toolchain '")
            .unwrap()
            .split_once('-')
            .unwrap();
        msrv
    };
    package.rust_version = Some(Inheritable::Set(msrv.to_string()));
    Ok(())
}

pub(crate) fn fill_rust_version(package: &mut Package) -> Result<(), ReadlineError> {
    println!("Filling the `rust-version` field.");
    println!("Description: \" The minimal supported Rust version.\"");
    loop {
        let strat: String = prompt(PROMPT)?;
        match strat.as_str() {
            "1" => {
                // skip
                break;
            }
            "2" => {
                if let Err(_) = run_msrv(package) {
                    continue;
                }
                break;
            }
            "3" => {
                let version: String = prompt("Please enter the version, e.g. `1.54.0`")?;
                package.rust_version = Some(Inheritable::Set(version));
                break;
            }
            _ => println!("{}", PROMPT),
        };
    }
    Ok(())
}
