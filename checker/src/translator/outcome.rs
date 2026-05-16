use std::borrow::Cow;

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
}
