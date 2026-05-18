use std::{
    cmp::Reverse,
    env,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::anyhow;
use tracing::info;

use super::Outcome;
use crate::args::{SortOrder, SortOrderKind};

#[derive(Debug)]
pub(super) struct Perm {
    inner: Vec<usize>,
}

impl Perm {
    pub(super) fn sort(&self, order: SortOrder) -> Vec<usize> {
        let Perm { inner } = self;
        let mut out = inner.clone();

        match order.order() {
            SortOrderKind::Ascendingly => out.sort_unstable(),
            SortOrderKind::Descendingly => out.sort_unstable_by_key(|n| Reverse(*n)),
        }

        out
    }

    pub(super) fn check(
        &self,
        bin_dir: impl AsRef<Path>,
        sorted: &[usize],
    ) -> anyhow::Result<Outcome> {
        let mut cmd = Command::new(
            env::var_os("CARGO")
                .map(|cargo_bin| String::from_utf8_lossy_owned(cargo_bin.into_encoded_bytes()))
                .ok_or_else(|| anyhow!("cargo binary was not set in `$CARGO`"))?,
        )
        .arg("r")
        .current_dir(bin_dir)
        .stderr(Stdio::null())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

        debug_assert_eq!(self.inner.len(), sorted.len());

        let byteified = self
            .inner
            .iter()
            .enumerate()
            .chain(sorted.iter().enumerate())
            .try_fold(
                {
                    let mut out = Vec::with_capacity(2 + 4 * self.inner.len());

                    writeln!(out, "{}", self.inner.len())?;

                    out
                },
                |mut out, (i, num)| {
                    if i == self.inner.len() - 1 {
                        writeln!(out, "{num}")?;
                    } else {
                        write!(out, "{num} ")?;
                    }

                    anyhow::Ok(out)
                },
            )?;

        info!(
            perm = ?self.inner,
            byteified_repr = String::from_utf8_lossy_owned(byteified.clone()),
        );

        cmd.stdin
            .take()
            .map(|mut stdin| stdin.write_all(&byteified));

        let out = String::from_utf8_lossy_owned(cmd.wait_with_output()?.exit_ok()?.stdout);
        info!(cmd_output = out);

        Ok(Outcome::from_str(out.trim()))
    }

    pub(super) fn new(inner: Vec<usize>) -> Self {
        Self { inner }
    }
}
