pub trait BitsToBytes: Iterator<Item = bool> {
    fn bytes(self) -> BitsToBytesIterator;
}

impl<T> BitsToBytes for T
where
    T: Iterator<Item = bool> + 'static,
{
    fn bytes(self) -> BitsToBytesIterator {
        BitsToBytesIterator::from(self)
    }
}

pub struct BitsToBytesIterator {
    bits: Box<dyn Iterator<Item = bool>>,
}

impl<T> From<T> for BitsToBytesIterator
where
    T: Iterator<Item = bool> + 'static,
{
    fn from(bits: T) -> Self {
        Self {
            bits: Box::new(bits),
        }
    }
}

impl Iterator for BitsToBytesIterator {
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
