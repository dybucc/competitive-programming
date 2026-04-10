use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NewIter<T: Ord> {
    Some(T),
    Sentinel,
}

impl<T: Ord> From<T> for NewIter<T> {
    fn from(value: T) -> Self {
        Self::Some(value)
    }
}

#[expect(
    clippy::non_canonical_partial_ord_impl,
    reason = "The current implementation is really just another way of putting the recommended \
              implementation (meaning it still relies on `Ord`'s total order, but the differences \
              are purely cosmetic.)"
)]
impl<T: Ord> PartialOrd for NewIter<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp = self.cmp(other);
        cmp.into()
    }
}

impl<T: Ord> Ord for NewIter<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = (self, other);
        match cmp {
            (Self::Some(value1), Self::Some(value2)) => value1.cmp(value2),
            (Self::Some(_), Self::Sentinel) => Ordering::Less,
            (Self::Sentinel, Self::Some(_)) => Ordering::Greater,
            (Self::Sentinel, Self::Sentinel) => Ordering::Equal,
        }
    }
}
