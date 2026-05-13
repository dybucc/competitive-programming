pub(crate) trait Repr {
    type Public;

    fn map_public(&self) -> <Self as Repr>::Public;
}
