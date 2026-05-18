#![feature(exit_status_error, string_from_utf8_lossy_owned)]

use std::{
    cmp::Reverse,
    env,
    fmt::Write as FmtWrite,
    fs::{self, File},
    io::{self, BufWriter, Write as IoWrite},
    path::Path,
    process::{Command, Stdio},
    sync::Mutex,
};

use anyhow::Context;
use clap::Parser;
use itertools::Itertools;
use tracing::info;

use crate::{
    args::{Args, SortOrder, SortOrderKind},
    translator::Translator,
};

mod args;
mod repr;
mod translator;

// Symbolic representation of results on permutations.
// 4 element permutation:
// i i i p p i i p i i i p i i i p p i i i i i i p
// - - - + + - - + - - - + - - - + + - - - - - - +

// TODO: stop outputting the checker's solution, and use the now modified
// argument for the lengths of the input collections to permute, to write to the
// output file all of the "translated" solutions (to have a side-by-side
// comparison of the representation of solutions for each permutation number.)

#[tracing::instrument(err(level = "info"))]
fn main() -> anyhow::Result<()> {
    if cfg!(debug_assertions) {
        tracing_subscriber::fmt()
            .with_line_number(false)
            .with_thread_ids(false)
            .with_ansi(false)
            .with_target(false)
            .with_level(false)
            .with_file(false)
            .with_thread_names(false)
            .without_time()
            .with_writer(
                env::current_dir()
                    .and_then(|pwd| File::create(pwd.join("debug.log")))
                    .map(Mutex::new)
                    .context("failed to initialize logging facilities")?,
            )
            .try_init()
            .map_err(anyhow::Error::from_boxed)?;
    }

    let args = Args::parse();

    info!(?args);

    let cap = args.cap();
    let sort_order = args.sort_order();
    let dir = args.dir()?;

    let mut stdout = BufWriter::new(io::stdout().lock());
    let perms: Vec<_> = (1..=cap).permutations(cap).collect();
    let sorted = match sort_order.order() {
        SortOrderKind::Ascendingly => {
            let mut out = perms.first().cloned().unwrap();
            out.sort_unstable();

            out
        }
        SortOrderKind::Descendingly => {
            let mut out = perms.first().cloned().unwrap();
            out.sort_unstable_by_key(|n| Reverse(*n));

            out
        }
    };

    if let Some(perms_file) = args.perms_fie() {
        with_perms_file(perms_file, &dir, sort_order, perms.clone())?;
    }

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
            .arg("r")
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

fn with_perms_file(
    path: impl AsRef<Path>,
    bin_dir: impl AsRef<Path>,
    order: SortOrder,
    perms: Vec<Vec<usize>>,
) -> anyhow::Result<()> {
    let mut translator = Translator::new(order);

    translator.extend(perms);

    fs::write(path, translator.translate_all().check(bin_dir)?)?;

    Ok(())
}
