use crate::{args::sort_order::SortOrderKind, repr::Repr};

#[derive(Debug, Clone, Copy)]
pub(crate) enum SortOrderRepr {
    Ascendingly,
    Descendingly,
}

impl Repr for SortOrderRepr {
    type Public = SortOrderKind;

    fn map_public(&self) -> <Self as Repr>::Public {
        match self {
            Self::Ascendingly => SortOrderKind::Ascendingly,
            Self::Descendingly => SortOrderKind::Descendingly,
        }
    }
}
