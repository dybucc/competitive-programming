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
    // `ExactSizeIterator` bound on the input iterable's iterator. Implement as the
    // method of the `FromIterator` trait, so that that trait impl is documented as
    // subpar in comparison with the (one and only) `new` method from the inherent
    // impl.

    /// Creates a new segment tree from an iterable, assumming there is a well
    /// defined iteration order.
    ///
    /// # Errors
    ///
    /// Fails if some auxiliary allocation fails.
    pub fn new<'a>(
        input: impl IntoIterator<Item = &'a T, IntoIter: ExactSizeIterator>,
    ) -> Result<Self, BuildError> {
        let iter = input.into_iter();
        let init_len = iter.len();
        let target_len = init_len.next_power_of_two();
        let sentinel_padded_input = {
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
                // Recall `iter::from_fn` produces an infinite iterator that ought be capped.
                sentinels.take(sentinel_padding)
            };
            let res = iter.chain(sentinel_values);
            res.collect::<Vec<_>>()
        };
        let mut buf = {
            let res = Box::try_new_uninit_slice(target_len);
            let alloc_error = |_| BuildError::AuxiliaryAlloc;
            let res = res.map_err(alloc_error);
            res?
        };
        let buf_slice = buf.as_mut_slice();
        let starting_idx = 0;
        let range = 0..target_len;
        build(buf_slice, sentinel_padded_input, starting_idx, range);
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
    tree: &mut [MaybeUninit<T>],
    array: Vec<NewIter<T>>,
    index: usize,
    range: Range<usize>,
) {
    let left = range.start;
    let right = range.end;
    if left != right {
        let new_left_range = {
            let new_right = left + right / 2;
            left..new_right
        };
        let new_right_range = {
            let new_left = left + right / 2 + 1;
            new_left..right
        };
        build(tree, array, index, new_left_range);
        build(tree, array, index, new_right_range);
        return;
    }
    let smallest_elem = {
        let mut iter = array.into_iter();
        let res_target = iter.nth(left);
        let wrapped_target = res_target.unwrap();
        wrapped_target.unwrap()
    };
    let target_elem = {
        let res_elem = tree.get_mut(index);
        res_elem.unwrap()
    };
    target_elem.write(smallest_elem);
}
