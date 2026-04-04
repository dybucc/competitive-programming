#![feature(try_with_capacity)]

pub(crate) mod errors;

use std::{cmp::Ordering, iter, mem::MaybeUninit};

#[doc(inline)]
pub use crate::errors::BuildError;

#[derive(Debug, Default, Clone)]
pub struct SegmentTree<T>(pub(crate) Vec<T>);

/// # Errors
///
/// Fails if some auxiilary allocation fails.
impl<T, A: Into<T>> TryFrom<Vec<A>> for SegmentTree<T> {
  type Error = BuildError;

  fn try_from(value: Vec<A>) -> Result<Self, Self::Error> { Self::new(value) }
}

/// # Errors
///
/// Fails if some auxiliary allocation fails.
impl<T, A: Into<T>, const N: usize> TryFrom<[A; N]> for SegmentTree<T> {
  type Error = BuildError;

  fn try_from(value: [A; N]) -> Result<Self, Self::Error> { Self::new(value) }
}

/// # Panics
///
/// Panics if some auxiliary allocation fails.
impl<T, A: Into<T>> FromIterator<A> for SegmentTree<T> {
  fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
    Self::new(iter).unwrap()
  }
}

impl<T> SegmentTree<T> {
  /// Creates a new segment tree from an iterable, assumming there is a well
  /// defined iteration order.
  ///
  /// # Errors
  ///
  /// Fails if some auxiliary allocation fails.
  pub fn new(
    input: impl IntoIterator<Item: Into<T>>,
  ) -> Result<Self, BuildError> {
    iter::once(2 * input.into_iter().count().next_power_of_two())
      .map(|len| {
        (
          len,
          Vec::try_with_capacity(len).map(|mut out| {
            (out.resize_with(len, || MaybeUninit::uninit()), out).1
          }),
        )
      })
      .try_fold(Vec::new(), |mut output, (len, mut out)| out)
      .map(|output| Self(output))
      .map_err(|_| BuildError::AuxiliaryAlloc)
  }
}

pub(crate) fn build<T>(
  (tree, array): (&mut [MaybeUninit<T>], impl IntoIterator<Item: Into<T>>),
  p: usize,
  (l, r): (usize, usize),
) {
  match l.cmp(&r) {
    | Ordering::Equal => tree.get_mut(p).map(|p| p.write(val)),
    | Ordering::Less => todo!(),
    | Ordering::Greater => todo!(),
  }
}
