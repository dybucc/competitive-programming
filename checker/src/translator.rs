use self::perm::Perm;
use crate::args::SortOrder;

mod outcome;
mod perm;
mod translated;

use self::outcome::Outcome;
pub(crate) use self::{outcome::OutcomeKind, translated::Translated};

#[derive(Debug)]
pub(crate) struct Translator {
    inner: Vec<Perm>,
    order: SortOrder,
}

impl Translator {
    pub(crate) fn new(order: SortOrder) -> Self {
        Self {
            inner: Vec::new(),
            order,
        }
    }

    pub(crate) fn translate_all(&self) -> Translated<'_> {
        self.translate_n(self.inner.len())
    }

    pub(crate) fn translate_n(&self, n: usize) -> Translated<'_> {
        let Self { inner, order } = self;

        Translated::new(&inner[..n], *order)
    }

    pub(crate) fn add(&mut self, perm: Vec<usize>) -> &mut Self {
        let Self { inner, .. } = self;

        inner.push(Perm::new(perm));

        self
    }
}

impl Extend<Vec<usize>> for Translator {
    fn extend<T: IntoIterator<Item = Vec<usize>>>(&mut self, iter: T) {
        iter.into_iter().for_each(|perm| _ = self.add(perm));
    }
}
