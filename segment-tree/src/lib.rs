#![feature(try_with_capacity, iterator_try_reduce)]

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

impl<T> SegmentTree<T> {
  // TODO: implement a less efficient `new` that does not require the
  // `ExactSizeIterator` bound on the input iterable's iterator.

  /// Creates a new segment tree from an iterable, assumming there is a well
  /// defined iteration order.
  ///
  /// # Errors
  ///
  /// Fails if some auxiliary allocation fails.
  pub fn new(
    input: impl IntoIterator<Item: Into<T>, IntoIter: ExactSizeIterator>,
  ) -> Result<Self, BuildError> {
    match iter::once(input.into_iter())
      .map(|iter| {
        (
          Vec::try_with_capacity(iter.len()).map(|mut out| {
            (out.resize_with(iter.len(), MaybeUninit::uninit), out).1
          }),
          iter,
        )
      })
      .next()
    {
      | Some((Ok(mut out), mut iter)) => todo!(
        "destructure `out` into raw components and perform `build` logic with \
         the raw pointer and length"
      ),
      | Some((Err(_), _)) => Err(BuildError::AuxiliaryAlloc),
      | _ => unreachable!(),
    }
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
