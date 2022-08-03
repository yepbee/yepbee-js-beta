pub trait AsSelf {
    #[inline]
    fn as_borrowed_self(&self) -> &Self {
        self
    }
}
