pub trait BytesToBits: Iterator<Item = u8> {
    fn bits(self) -> Box<dyn Iterator<Item = bool>>;
}

impl<T> BytesToBits for T
where
    T: Iterator<Item = u8> + 'static,
{
    fn bits(self) -> Box<dyn Iterator<Item = bool>> {
        Box::new(self.flat_map(|byte| (0..8).map(move |offset| byte & (1 << offset) != 0)))
    }
}
