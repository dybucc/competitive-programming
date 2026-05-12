#![feature(string_from_utf8_lossy_owned)]

use std::{
    cmp::Reverse,
    env,
    ffi::OsString,
    fmt::Write as FmtWrite,
    fs,
    io::{self, BufWriter, Write as IoWrite},
    process::{Command, Stdio},
};

use anyhow::{anyhow, bail};
use clap::Parser;
use itertools::Itertools;
use tracing::info;

// TODO: reorganize this into modules and refactor manual argument handling, and
// get this to follow the `xtask` pattern.

#[derive(Debug, Parser)]
#[command(
    disable_version_flag = true,
    disable_help_subcommand = true,
    disable_colored_help = true,
    about = None,
    long_about = None,
)]
struct Args {
    /// Input collection to sort and check the output of its permutations.
    input: String,
    #[arg(short, long, conflicts_with = "descendingly")]
    ascendingly: bool,
    #[arg(short, long, conflicts_with = "ascendingly")]
    descendingly: bool,
}

impl Args {
    fn input(&self) -> impl AsRef<str> {
        let Self { input, .. } = self;

        input
    }

    fn sort_order(&self) -> SortOrder {
        let Self { ascendingly, .. } = *self;

        if ascendingly {
            return SortOrder::new(SortOrderKind::Ascendingly);
        }

        SortOrder::new(SortOrderKind::Descendingly)
    }
}

#[derive(Debug)]
struct SortOrder {
    repr: SortOrderRepr,
}

impl SortOrder {
    fn new(order: SortOrderKind) -> Self {
        match order {
            SortOrderKind::Ascendingly => Self {
                repr: SortOrderRepr::Ascendingly,
            },
            SortOrderKind::Descendingly => Self {
                repr: SortOrderRepr::Descendingly,
            },
        }
    }

    fn order(&self) -> SortOrderKind {
        let Self { repr } = self;

        repr.map_public()
    }
}

#[derive(Debug, Clone, Copy)]
enum SortOrderKind {
    Ascendingly,
    Descendingly,
}

#[derive(Debug, Clone, Copy)]
enum SortOrderRepr {
    Ascendingly,
    Descendingly,
}

impl Repr for SortOrderRepr {
    type Public = SortOrderKind;

    fn map_public(&self) -> <Self as Repr>::Public {
        match self {
            Self::Ascendingly => SortOrderKind::Ascendingly,
            Self::Descendingly => SortOrderKind::Descendingly,
        }
    }
}

trait Repr {
    type Public;

    fn map_public(&self) -> <Self as Repr>::Public;
}

#[tracing::instrument(err(level = "info"))]
fn main() -> anyhow::Result<()> {
    let mut args = Args::parse();
    let mut args = env::args_os();

    info!(?args);

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

    info!(input);

    let sort_order = args
        .next()
        .map(OsString::into_string)
        .map(|res| {
            res.map_err(|_| anyhow!("cli args should contian sort order of final collection"))
        })
        .ok_or_else(|| anyhow!("cli args should contian sort order of final collection"))??;

    info!(sort_order);

    let dir = args
        .next()
        .map(fs::canonicalize)
        .ok_or_else(|| anyhow!("cli args should contain dir of binary to test"))??;

    info!(dir = ?dir.display());

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

    info!(proc_input = ?input);

    let mut stdout = BufWriter::new(io::stdout().lock());
    let mut sorted = input.clone();

    if ascendingly {
        sorted.sort_unstable();
    } else {
        sorted.sort_unstable_by_key(|n| Reverse(*n));
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
                .spawn()?;

            if let Some(mut stdin) = cmd.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }

            let out = String::from_utf8_lossy_owned(cmd.wait_with_output()?.stdout);

            info!(stdout = out);

            write!(stdout, "perm = {perm:?}, sol = {out}")?;

            Ok::<_, anyhow::Error>(())
        })
}
