#![feature(exit_status_error, string_from_utf8_lossy_owned)]

use std::{
    cmp::Reverse,
    env,
    fmt::Write as FmtWrite,
    fs::File,
    io::{self, BufWriter, Write as IoWrite},
    process::{Command, Stdio},
    sync::Mutex,
};

use anyhow::Context;
use clap::Parser;
use itertools::Itertools;
use tracing::info;

use crate::args::{Args, SortOrderKind};

mod args;
mod repr;

// Symbolic representation of results on permutations.
// 4 element permutation:
// - - - + + - - + - - - + - - - + + - - - - - - +

// TODO: implement a "translation" to the above symbolic representation after
// having computed the permutations and check if there are patterns in that
// representation.

#[tracing::instrument(err(level = "info"))]
fn main() -> anyhow::Result<()> {
    if cfg!(debug_assertions) {
        macro_rules! init_msg {
            () => {
                "failed to initialize logging facilities"
            };
        }

        tracing_subscriber::fmt()
            .with_line_number(false)
            .with_thread_ids(false)
            .with_ansi(false)
            .with_target(false)
            .with_level(false)
            .with_file(false)
            .with_thread_names(false)
            .without_time()
            .with_writer(Mutex::new(
                File::create(
                    env::current_dir()
                        .map(|pwd| pwd.join("debug.log"))
                        .context(init_msg!())?,
                )
                .context(init_msg!())?,
            ))
            .try_init()
            .map_err(anyhow::Error::from_boxed)?;
    }

    let args = Args::parse();

    info!(?args);

    let cap = args.cap();
    let sort_order = args.sort_order();
    let dir = args.dir()?;

    let mut stdout = BufWriter::new(io::stdout().lock());
    let sorted: Vec<_> = match sort_order.order() {
        SortOrderKind::Ascendingly => (1..=cap).sorted_unstable().collect(),
        SortOrderKind::Descendingly => (1..=cap).sorted_unstable_by_key(|n| Reverse(*n)).collect(),
    };

    info!(sorted_input = ?sorted);

    let perms: Vec<_> = (1..=cap).permutations(cap).collect();

    perms.iter().try_for_each(|perm| {
        info!(?perm);

        let input = perm
            .iter()
            .enumerate()
            .chain(sorted.iter().enumerate())
            .fold(format!("{}\n", perm.len()), |mut out, (i, num)| {
                if i == perm.len() - 1 {
                    writeln!(out, "{num}").unwrap();
                } else {
                    write!(out, "{num} ").unwrap();
                }

                out
            });

        info!(stringified_perm = input);

        let mut cmd = Command::new("cargo")
            .args(["r", "--", &input])
            .current_dir(&dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;

        if let Some(mut stdin) = cmd.stdin.take() {
            write!(stdin, "{input}")?;
        }

        let out = String::from_utf8_lossy_owned(cmd.wait_with_output()?.exit_ok()?.stdout);

        info!(checker_sol = out);

        write!(stdout, "perm = {perm:?}, sol = {out}")?;

        Ok(())
    })
}
