use std::path::Path;

use super::Perm;

#[derive(Debug)]
pub(crate) struct Translated<'a> {
    src: Vec<&'a Perm>,
    res: Option<String>,
}

impl<'a> FromIterator<&'a Perm> for Translated<'a> {
    fn from_iter<T: IntoIterator<Item = &'a Perm>>(iter: T) -> Self {
        Self {
            src: iter.into_iter().collect(),
            res: Option::default(),
        }
    }
}

impl<'a> Translated<'a> {
    pub(crate) fn check(&mut self, bin_dir: impl AsRef<Path>) -> anyhow::Result<()> {
        // TODO: finish up this interface and use in `main`.

        let Self { src, res } = self;
        let outcomes = src
            .iter()
            .map(|perm| perm.check(&bin_dir))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(())
    }
}
