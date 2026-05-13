#![feature(string_from_utf8_lossy_owned)]

use std::{
    cmp::Reverse,
    env,
    fmt::Write as FmtWrite,
    fs::File,
    io::{self, BufWriter, Write as IoWrite},
    process::{Command, Stdio},
    sync::Mutex,
};

use anyhow::{Context, anyhow};
use clap::Parser;
use itertools::Itertools;
use tracing::info;

use crate::args::{Args, SortOrderKind};

mod args;
mod repr;

// FIXME: there's an issue with the way we capture output from the command we
// launch for the binary checker, as instead of silently getting the stdout into
// a sink of our own, it's outputting the `stdout` of each launched child
// process.

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
            .init();
    }

    let args = Args::parse();

    info!(?args);

    let input = args.input();
    let sort_order = args.sort_order();
    let dir = args.dir()?;

    info!(input = input.as_ref(), ?sort_order, dir = %dir.display());

    let input = input
        .as_ref()
        .split_ascii_whitespace()
        .map(str::parse)
        .map(|res| {
            res.map_err(|_| {
                anyhow!(
                    "input collection should contain only whitespace-separated integer radix 10 \
                     digits"
                )
            })
        })
        .collect::<anyhow::Result<Vec<usize>>>()?;

    info!(proc_input = ?input);

    let mut stdout = BufWriter::new(io::stdout().lock());
    let mut sorted = input.clone();

    match sort_order.order() {
        SortOrderKind::Ascendingly => sorted.sort_unstable(),
        SortOrderKind::Descendingly => sorted.sort_unstable_by_key(|n| Reverse(*n)),
    }

    info!(sorted_input = ?sorted);

    input
        .iter()
        .copied()
        .permutations(input.len())
        .try_for_each(|perm| {
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
                .spawn()?;

            if let Some(mut stdin) = cmd.stdin.take() {
                write!(stdin, "{input}")?;
            }

            let out = String::from_utf8_lossy_owned(cmd.wait_with_output()?.stdout);

            info!(stdout = out);

            write!(stdout, "perm = {perm:?}, sol = {out}")?;

            Ok(())
        })
}
