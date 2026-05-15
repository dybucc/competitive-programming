use self::perm::Perm;

mod outcome;
mod perm;
mod translated;

use self::outcome::Outcome;
pub(crate) use self::translated::Translated;

#[derive(Debug)]
pub(crate) struct Translator {
    inner: Vec<Perm>,
}

impl Translator {
    pub(crate) fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub(crate) fn translate_all(&self) -> Translated<'_> {
        todo!()
    }

    pub(crate) fn translate_n(&self, n: usize) -> Translated<'_> {
        let Self { inner } = self;

        inner.iter().take(n).collect()
    }

    pub(crate) fn add(&mut self, perm: Vec<usize>) -> &mut Self {
        let Self { inner } = self;

        inner.push(Perm::new(perm));

        self
    }
}
