use std::path::Path;

use super::Perm;
use crate::args::SortOrder;

#[derive(Debug)]
pub(crate) struct Translated<'a> {
    src: &'a [Perm],
    sorted: Vec<usize>,
}

impl<'a> Translated<'a> {
    pub(crate) fn new(input: &'a [Perm], order: SortOrder) -> Self {
        Self {
            src: input,
            sorted: input.first().map(|perm| perm.sort(order)).unwrap(),
        }
    }

    pub(crate) fn check(&mut self, bin_dir: impl AsRef<Path>) -> anyhow::Result<()> {
        let Self { src, sorted } = self;

        // TODO: finish up this routine, and perform further processing of the `input`
        // argument in the `Args` type.
        let outcomes = src
            .iter()
            .map(|perm| perm.check(&bin_dir, sorted))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(())
    }
}
