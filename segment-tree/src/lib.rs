#![feature(allocator_api, str_as_str)]

use std::{iter, mem::MaybeUninit, ops::Range};

use itertools::Itertools;

pub(crate) mod errors;
pub(crate) mod utils;

#[doc(inline)]
pub use crate::errors::BuildError;
pub(crate) use crate::utils::NewIter;

#[derive(Debug, Default, Clone)]
pub struct SegmentTree<T: Ord>(pub(crate) Vec<T>);

/// # Errors
///
/// Fails if some auxiliary allocation fails.
impl<T: Ord, A: Into<T>> TryFrom<Vec<A>> for SegmentTree<T> {
    type Error = BuildError;

    fn try_from(value: Vec<A>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// # Errors
///
/// Fails if some auxiliary allocation fails.
impl<T: Ord, A: Into<T>, const N: usize> TryFrom<[A; N]> for SegmentTree<T> {
    type Error = BuildError;

    fn try_from(value: [A; N]) -> Result<Self, Self::Error> {
        Self::new(value)
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
        let iter = input.into_iter();
        let init_len = iter.len();
        let target_len = init_len.next_power_of_two();
        let sentinel_padded_iter = {
            let iter = iter.map_into::<T>();
            // We map onto a different time capable of holding sentinel values that always
            // compare larger than actual items of the iterator. See the `Ord`
            // implementation for `NewIter`.
            let iter = iter.map_into::<NewIter<T>>();
            let sentinel_values = {
                let sentinel_producer = || {
                    let sentinel = NewIter::Sentinel;
                    Some(sentinel)
                };
                let sentinels = iter::from_fn(sentinel_producer);
                // We ought pad with as many sentinel values as necessary to reach the next
                // power of two of the input iterator's initial length (or leave it as a power
                // of two, if it already was; See the documentation on `next_power_of_two()`.)
                let sentinel_padding = target_len - init_len;
                sentinels.take(sentinel_padding)
            };
            iter.chain(sentinel_values)
        };
        let mut buf = {
            let res = Box::try_new_uninit_slice(target_len);
            let alloc_error = |_| BuildError::AuxiliaryAlloc;
            let res = res.map_err(alloc_error);
            res?
        };
        let buf_slice = buf.as_mut_slice();
        let ds_input = (buf_slice, sentinel_padded_iter);
        let starting_idx = 0;
        let range = 0..target_len;
        build(ds_input, starting_idx, range);
        let tree = {
            // SAFETY: `build()` ensures the entire range given, starting from the provided
            // index, is initialized.
            let tree = unsafe { buf.assume_init() };
            let tree = tree.into_vec();
            Self(tree)
        };
        Ok(tree)
    }
}

pub(crate) fn build<T: Ord>(
    (tree, array): (&mut [MaybeUninit<T>], impl IntoIterator<Item = NewIter<T>>),
    p: usize,
    r: Range<usize>,
) {
    todo!()
}
