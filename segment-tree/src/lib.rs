#![feature(try_with_capacity)]

pub(crate) mod build_error;

pub(crate) use build_error::BuildError;

#[derive(Debug, Default, Clone)]
pub(crate) struct SegmentTree<T>(Vec<T>);

/// # Panics
///
/// Panics if some internal allocation fails.
impl<T, A: Into<T>> From<Vec<A>> for SegmentTree<T> {
  fn from(value: Vec<A>) -> Self { Self::new(value).unwrap() }
}

/// # Panics
///
/// Panics if some internal allocation fails.
impl<T, A: Into<T>> FromIterator<A> for SegmentTree<T> {
  fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
    Self::new(iter).unwrap()
  }
}

impl<T> SegmentTree<T> {
  /// Creates a new segment tree.
  ///
  /// # Errors
  ///
  /// If some internal allocation fails, this returns `Err`.
  fn new(input: impl IntoIterator<Item: Into<T>>) -> Result<Self, BuildError> {
    let mut out =
      Vec::try_with_capacity(input.into_iter().count().next_power_of_two())?;

    Ok(Self(out))
  }
}
