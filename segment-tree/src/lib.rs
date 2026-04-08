#![feature(allocator_api)]

use std::{cmp::Ordering, iter, mem::MaybeUninit, ops::{Not, Range}};

use itertools::Itertools;

pub(crate) mod errors;

#[doc(inline)]
pub use crate::errors::BuildError;

#[derive(Debug, Default, Clone)]
pub struct SegmentTree<T: Ord>(pub(crate) Vec<T>);

/// # Errors
///
/// Fails if some auxiliary allocation fails.
impl<T: Ord, A: Into<T>> TryFrom<Vec<A>> for SegmentTree<T> {
  type Error = BuildError;

  fn try_from(value: Vec<A>) -> Result<Self, Self::Error> { Self::new(value) }
}

/// # Errors
///
/// Fails if some auxiliary allocation fails.
impl<T: Ord, A: Into<T>, const N: usize> TryFrom<[A; N]> for SegmentTree<T> {
  type Error = BuildError;

  fn try_from(value: [A; N]) -> Result<Self, Self::Error> { Self::new(value) }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NewIter<T: Ord> {
  Some(T),
  None,
}

impl<T: Ord> From<T> for NewIter<T> {
  fn from(value: T) -> Self { Self::Some(value) }
}

impl<T: Ord> PartialOrd for NewIter<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.cmp(other).into()
  }
}

impl<T: Ord> Ord for NewIter<T> {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      | (Self::Some(value1), Self::Some(value2)) => value1.cmp(value2),
      | (Self::Some(_), Self::None) => Ordering::Less,
      | (Self::None, Self::Some(_)) => Ordering::Greater,
      | (Self::None, Self::None) => Ordering::Equal,
    }
  }
}

impl<T: Ord> SegmentTree<T> {
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
        (Box::try_new_uninit_slice(iter.len().next_power_of_two()), unsafe {
          iter::once(iter.map_into::<T>().map_into())
            .map(|iter| (
              iter.len().is_power_of_two().not().then(|| iter.len().next_power_of_two()).unwrap_or(iter.len()),
              iter::once(iter.len()).map(|len| iter.chain(iter::repeat_with(|| NewIter::None).take(len.next_power_of_two() - len))).next().unwrap_unchecked()
            ))
      })
      .next()
    {
      | Some((Ok(mut tree), (len, array))) => Ok(Self(
        (build((&mut tree, array), 0, 0..len), unsafe {
          tree.assume_init().into_vec()
        })
          .1,
      )),
      | Some((Err(_), ..)) => Err(BuildError::AuxiliaryAlloc),
      | _ => unreachable!(),
    }
  }
}

pub(crate) fn build<T: Ord>(
  (tree, array): (&mut [MaybeUninit<T>], impl IntoIterator<Item = NewIter<T>>),
  p: usize,
  r: Range<usize>,
) {
  todo!()
}
