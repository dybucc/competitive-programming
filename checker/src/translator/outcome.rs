use self::internal::OutcomeRepr;

mod internal;

#[derive(Debug)]
pub(super) struct Outcome {
    repr: OutcomeRepr,
}

impl Outcome {
    pub(super) fn from_str(slice: impl AsRef<str>) -> Self {
        Self {
            repr: OutcomeRepr::new(slice),
        }
    }
}
