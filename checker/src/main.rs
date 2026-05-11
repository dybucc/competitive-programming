#![feature(exit_status_error, string_from_utf8_lossy_owned)]

use std::{
    cmp::Reverse,
    env,
    ffi::OsString,
    fmt::Write as FmtWrite,
    fs,
    io::{self, BufWriter, Write as IoWrite},
    process::Command,
};

use anyhow::{anyhow, bail};
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut args = env::args_os();
    let input = args
        .nth(1)
        .map(OsString::into_string)
        .map(|res| {
            res.map_err(|_| {
                anyhow!(
                    "cli args should contain input collection to permute with \
                     whitespace-separated digits"
                )
            })
        })
        .ok_or_else(|| {
            anyhow!(
                "cli args should contain input collection to permute with whitespace-separated \
                 digits"
            )
        })??;
    let sort_order = args
        .next()
        .map(OsString::into_string)
        .map(|res| {
            res.map_err(|_| anyhow!("cli args should contian sort order of final collection"))
        })
        .ok_or_else(|| anyhow!("cli args should contian sort order of final collection"))??;
    let dir = args
        .next()
        .map(fs::canonicalize)
        .ok_or_else(|| anyhow!("cli args should contain dir of binary to test"))??;

    eprintln!("dir = {}", dir.display());

    let ascendingly = if sort_order == "-a" {
        true
    } else if sort_order == "-d" {
        false
    } else {
        bail!("sort order option should be one of `-a` or `-d`");
    };
    let input = input
        .trim()
        .split_ascii_whitespace()
        .map(str::parse)
        .map(|res| {
            res.map_err(|_| {
                anyhow!("input collection should contain only whitespace-separated sequences")
            })
        })
        .collect::<anyhow::Result<Vec<usize>>>()?;

    eprintln!("input = {input:?}");

    let mut stdout = BufWriter::new(io::stdout().lock());

    let mut sorted = input.clone();
    if ascendingly {
        sorted.sort_unstable();
    } else {
        sorted.sort_unstable_by_key(|n| Reverse(*n));
    }

    eprintln!("sorted = {sorted:?}");

    // TODO: fix the way we pass input into the program to use a pipe or what not
    // instead of directly passing the permutation through the cli args (the target
    // binary reads from `stdin` and not from the arguments issued to it.)
    input
        .iter()
        .copied()
        .permutations(input.len())
        .for_each(|perm| {
            write!(
                stdout,
                "perm = {:?}, sol = {}",
                perm,
                String::from_utf8_lossy_owned(
                    Command::new("cargo")
                        .args([
                            "r",
                            "--",
                            &perm
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
                                })
                        ])
                        .current_dir(&dir)
                        .output()
                        .unwrap()
                        .exit_ok()
                        .unwrap()
                        .stdout
                )
                .trim()
            )
            .unwrap();
        });

    Ok(())
}
