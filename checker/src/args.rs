use std::{
    borrow::Cow,
    env,
    path::{Path, PathBuf},
};

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
    /// Lengths of the input collections to consider for running the entire
    /// permutations of collections with these lenghts through the checker
    /// program.
    #[arg(value_parser = 1..)]
    max: Vec<usize>,
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
    /// File to save the result of permuting the collection. Leave empty to not
    /// save a symbolic representation of the permutation.
    #[arg(short, long, value_name = "PATH")]
    perms_file: Option<String>,
}

impl Args {
    pub(crate) fn cap(&self) -> &[usize] {
        let Self { max, .. } = self;

        max
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

    pub(crate) fn perms_fie(&self) -> Option<PathBuf> {
        if let Some(path) = &self.perms_file {
            PathBuf::from(path.clone()).into()
        } else {
            Option::default()
        }
    }
}
