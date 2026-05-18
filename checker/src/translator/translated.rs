use std::{fmt::Write, path::Path};

use tracing::info;

use super::Perm;
use crate::{args::SortOrder, translator::OutcomeKind};

#[derive(Debug)]
pub(crate) struct Translated<'a> {
    src: &'a [Perm],
    sorted: Vec<usize>,
}

impl<'a> Translated<'a> {
    pub(super) fn new(input: &'a [Perm], order: SortOrder) -> Self {
        Self {
            src: input,
            sorted: input.first().map(|perm| perm.sort(order)).unwrap(),
        }
    }

    pub(crate) fn check(&mut self, bin_dir: impl AsRef<Path>) -> anyhow::Result<String> {
        let outcomes = self
            .src
            .iter()
            .map(|perm| {
                let out = perm.check(&bin_dir, &self.sorted);
                info!(?perm, outcome = ?out);

                out
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        let mut out = outcomes
            .into_iter()
            .try_fold(String::new(), |mut out, outcome| {
                outcome.with(|outcome| {
                    match outcome {
                        OutcomeKind::Possible => write!(out, "+")?,
                        OutcomeKind::Impossible => write!(out, "-")?,
                    }

                    anyhow::Ok(out)
                })
            })?;

        writeln!(out)?;

        Ok(out)
    }
}
