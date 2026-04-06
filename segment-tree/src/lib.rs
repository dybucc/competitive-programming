#![feature(try_with_capacity, iterator_try_reduce, box_vec_non_null)]

pub(crate) mod errors;

use std::{cmp::Ordering, iter, mem::MaybeUninit};

#[doc(inline)]
pub use crate::errors::BuildError;

#[derive(Debug, Default, Clone)]
pub struct SegmentTree<T>(pub(crate) Vec<T>);

/// # Errors
///
/// Fails if some auxiliary allocation fails.
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
  // `ExactSizeIterator` bound on the input iterable's iterator. Implement as
  // the method of the `FromIterator` trait, so that that trait impl is
  // documented as subpar in comparison with the (one and only) `new` method
  // from the inherent impl.

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
            (out.resize_with(iter.len(), MaybeUninit::<T>::uninit), out).1
          }),
          iter,
        )
      })
      .next()
    {
      | Some((Ok(tree), array)) => {
        iter::once(tree.into_parts()).map(|(ptr, len, cap)| {
          // casting to a slice may not be feasible because the methods in std
          // don't seem to consider it sound to mutate the slice
        });
        todo!()
      },
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
