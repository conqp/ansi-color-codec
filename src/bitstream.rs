pub trait BytesToBits<T>
where
    T: Iterator<Item = bool>,
{
    fn bits(self) -> T;
}

impl<T> BytesToBits<BytesToBitsIterator<T>> for T
where
    T: Iterator<Item = u8>,
{
    fn bits(self) -> BytesToBitsIterator<T> {
        BytesToBitsIterator::from(self)
    }
}

pub struct BytesToBitsIterator<T>
where
    T: Iterator<Item = u8>,
{
    bytes: T,
    current: Option<u8>,
    index: u8,
}

impl<T> From<T> for BytesToBitsIterator<T>
where
    T: Iterator<Item = u8>,
{
    fn from(bytes: T) -> Self {
        Self {
            bytes,
            current: None,
            index: 0,
        }
    }
}

impl<T> Iterator for BytesToBitsIterator<T>
where
    T: Iterator<Item = u8>,
{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 7 {
            self.current = None;
            self.index = 0;
        }

        let current = match self.current {
            None => match self.bytes.next() {
                None => {
                    return None;
                }
                Some(byte) => {
                    self.current = Some(byte);
                    byte
                }
            },
            Some(byte) => byte,
        };

        let bit = current & (1 << self.index) != 0;
        self.index += 1;
        Some(bit)
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
