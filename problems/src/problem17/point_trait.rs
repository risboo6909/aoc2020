pub(crate) trait Point: Sized {
    fn get_vicinity(&self) -> Vec<Self>;
}
