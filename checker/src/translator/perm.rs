use std::{
    cmp::Reverse,
    env,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::anyhow;

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
                .ok_or_else(|| anyhow!("failed to fetch binary for cargo through `$CARGO`"))?,
        )
        .args(["r"])
        .current_dir(bin_dir)
        .stderr(Stdio::null())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

        let Self { inner } = self;

        debug_assert_eq!(inner.len(), sorted.len());

        let mut byteified = Vec::with_capacity(2 + 4 * inner.len());

        writeln!(byteified, "{}", inner.len())?;

        inner
            .iter()
            .enumerate()
            .chain(sorted.iter().enumerate())
            .try_for_each(|(i, num)| {
                if i == inner.len() - 1 {
                    writeln!(byteified, "{num}")?;
                } else {
                    write!(byteified, "{num} ")?;
                }

                anyhow::Ok(())
            })?;

        cmd.stdin
            .take()
            .map(|mut stdin| stdin.write_all(&byteified));

        Ok(Outcome::from_str(
            String::from_utf8_lossy_owned(cmd.wait_with_output()?.stdout).trim(),
        ))
    }

    pub(super) fn new(inner: Vec<usize>) -> Self {
        Self { inner }
    }
}
