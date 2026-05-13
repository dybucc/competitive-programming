use std::{borrow::Cow, env, path::Path};

use clap::Parser;

mod sort_order;

pub(crate) use crate::args::sort_order::{SortOrder, SortOrderKind};

#[derive(Debug, Parser)]
#[command(
    disable_version_flag = true,
    disable_help_subcommand = true,
    disable_colored_help = true,
    about = None,
    long_about = None,
)]
pub(crate) struct Args {
    /// Input collection to sort and check the output of its permutations.
    input: String,
    /// Whether to sort the input collection ascendingly. Is exclusive with
    /// `--descendingly`.
    #[arg(short, long, required = true, conflicts_with = "descendingly")]
    ascendingly: bool,
    /// Whether to sort the input collection descendingly. Is exclusive with
    /// `--ascendingly`.
    #[arg(short, long, required = true, conflicts_with = "ascendingly")]
    descendingly: bool,
    /// The directory of the binary to execute. Leave empty to use the pwd.
    #[arg(long, value_name = "PATH")]
    directory: Option<String>,
}

impl Args {
    pub(crate) fn input(&self) -> impl AsRef<str> {
        let Self { input, .. } = self;

        input
    }

    pub(crate) fn sort_order(&self) -> SortOrder {
        let Self { ascendingly, .. } = *self;

        if ascendingly {
            SortOrder::new(SortOrderKind::Ascendingly)
        } else {
            SortOrder::new(SortOrderKind::Descendingly)
        }
    }

    pub(crate) fn dir(&self) -> anyhow::Result<Cow<'_, Path>> {
        if let Self {
            directory: Some(path),
            ..
        } = self
        {
            Ok(Path::new(path).into())
        } else {
            env::current_dir().map(Into::into).map_err(Into::into)
        }
    }
}
