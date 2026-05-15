use std::{
    env,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::anyhow;

use super::Outcome;

#[derive(Debug)]
pub(super) struct Perm {
    inner: Vec<usize>,
}

impl Perm {
    pub(super) fn check(&self, bin_dir: impl AsRef<Path>) -> anyhow::Result<Outcome> {
        let cargo = env::var_os("CARGO")
            .map(|cargo_bin| String::from_utf8_lossy_owned(cargo_bin.into_encoded_bytes()))
            .ok_or_else(|| anyhow!("failed to fetch binary for cargo through `$CARGO`"))?;

        let mut cmd = Command::new(cargo)
            .args(["r"])
            .current_dir(bin_dir)
            .stderr(Stdio::null())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let Self { inner } = self;

        let stringified =
            inner
                .iter()
                .try_fold(Vec::with_capacity(inner.len()), |mut out, num| {
                    write!(out, "{num}")?;

                    anyhow::Ok(out)
                })?;

        cmd.stdin
            .take()
            .map(|mut stdin| stdin.write_all(stringified.trim_ascii()));

        Ok(Outcome::from_str(
            String::from_utf8_lossy_owned(cmd.wait_with_output()?.stdout).trim(),
        ))
    }

    pub(super) fn new(inner: Vec<usize>) -> Self {
        Self { inner }
    }
}
