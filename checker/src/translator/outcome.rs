use std::borrow::Cow;

mod kind;

pub(crate) use self::kind::OutcomeKind;

#[derive(Debug)]
pub(super) struct Outcome {
    inner: Cow<'static, str>,
}

impl Outcome {
    pub(super) fn from_str(slice: impl AsRef<str>) -> Self {
        Self {
            inner: slice.as_ref().to_string().into(),
        }
    }

    pub(crate) fn with<T>(self, f: impl FnOnce(OutcomeKind) -> T) -> T {
        f(OutcomeKind::new(self.inner))
    }
}
