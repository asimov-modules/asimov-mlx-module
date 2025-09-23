// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

#[cfg(feature = "std")]
extern crate std;

use asimov_module::{prelude::*, tracing};
use core::error::Error;

extern crate alloc;
use alloc::vec;

#[derive(Clone, Debug, bon::Builder)]
#[builder(on(String, into))]
pub struct Options {
    #[builder(default = "mlx-community/Llama-3.2-3B-Instruct-4bit")]
    pub model: String,
}

#[cfg(feature = "std")]
pub fn generate(input: impl AsRef<str>, options: &Options) -> Result<Vec<String>, Box<dyn Error>> {
    use std::process::Stdio;

    let mut cmd = std::process::Command::new("mlx_lm.generate");

    cmd.env("NO_COLOR", "1"); // See: https://no-color.org
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.args(["--model", &options.model]);
    cmd.args(["--prompt", input.as_ref()]);
    cmd.args(["--verbose", "False"]);

    let output = cmd
        .spawn()
        .inspect_err(|err| tracing::error!(?err, "unable to run mlx"))?
        .wait_with_output()
        .inspect_err(|err| tracing::error!(?err, "mlx execution failed"))?;

    if !output.stderr.is_empty() {
        String::from_utf8(output.stderr)
            .inspect(|stderr| tracing::debug!(stderr))
            .ok();
    }

    let stdout = String::from_utf8(output.stdout)?;

    Ok(vec![stdout])
}
