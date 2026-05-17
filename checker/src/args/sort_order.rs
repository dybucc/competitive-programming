use crate::repr::Repr;

mod internal;

use crate::args::sort_order::internal::SortOrderRepr;

#[derive(Debug, Clone, Copy)]
pub(crate) struct SortOrder {
    repr: SortOrderRepr,
}

impl SortOrder {
    pub(super) fn new(order: SortOrderKind) -> Self {
        match order {
            SortOrderKind::Ascendingly => Self {
                repr: SortOrderRepr::Ascendingly,
            },
            SortOrderKind::Descendingly => Self {
                repr: SortOrderRepr::Descendingly,
            },
        }
    }

    pub(crate) fn order(self) -> SortOrderKind {
        let Self { repr } = self;

        repr.map_public()
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum SortOrderKind {
    Ascendingly,
    Descendingly,
}
