use std::borrow::Cow;

#[derive(Debug)]
pub(super) struct OutcomeRepr {
    inner: Cow<'static, str>,
}

impl OutcomeRepr {
    pub(super) fn new(s: impl AsRef<str>) -> Self {
        Self {
            inner: s.as_ref().to_owned().into(),
        }
    }
}
