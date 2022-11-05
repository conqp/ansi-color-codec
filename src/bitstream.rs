pub trait BytesToBits<'a> {
    fn bits(self) -> Box<dyn Iterator<Item = bool> + 'a>;
}

impl<'a, T> BytesToBits<'a> for T
where
    T: Iterator<Item = u8> + 'a,
{
    fn bits(self) -> Box<dyn Iterator<Item = bool> + 'a> {
        Box::new(self.flat_map(|byte| (0..8).map(move |offset| byte & (1 << offset) != 0)))
    }
}

pub trait BitsToBytes<T>
where
    T: Iterator<Item = bool>,
{
    fn bytes(self) -> BitsToBytesIterator<T>;
}

impl<T> BitsToBytes<T> for T
where
    T: Iterator<Item = bool>,
{
    fn bytes(self) -> BitsToBytesIterator<T> {
        BitsToBytesIterator::from(self)
    }
}

pub struct BitsToBytesIterator<T>
where
    T: Iterator<Item = bool>,
{
    bits: T,
}

impl<T> From<T> for BitsToBytesIterator<T>
where
    T: Iterator<Item = bool>,
{
    fn from(bits: T) -> Self {
        Self { bits }
    }
}

impl<T> Iterator for BitsToBytesIterator<T>
where
    T: Iterator<Item = bool>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut byte = 0;
        let mut exhausted = false;

        for index in 0..8 {
            match self.bits.next() {
                Some(bit) => {
                    byte += (bit as u8) << index;
                }
                None => {
                    exhausted = true;
                    break;
                }
            }
        }

        if byte == 0 && exhausted {
            None
        } else {
            Some(byte)
        }
    }
}
