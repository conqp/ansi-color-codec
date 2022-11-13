pub type Triplet = u8;

pub trait BytesToBits<'a> {
    fn bits(self) -> Box<dyn Iterator<Item = bool> + 'a>;
}

impl<'a, T> BytesToBits<'a> for T
where
    T: Iterator<Item = u8> + 'a,
{
    fn bits(self) -> Box<dyn Iterator<Item = bool> + 'a> {
        Box::new(self.flat_map(|byte| (0..8).map(move |index| byte & (1 << index) != 0)))
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

#[derive(Debug, Eq, PartialEq)]
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

pub trait Triplets<T>
where
    T: Iterator<Item = Triplet>,
{
    fn triplets(self) -> T;
}

impl<T> Triplets<TripletIterator<T>> for T
where
    T: Iterator<Item = bool>,
{
    fn triplets(self) -> TripletIterator<T> {
        TripletIterator::from(self)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct TripletIterator<T>
where
    T: Iterator<Item = bool>,
{
    bits: T,
}

impl<T> From<T> for TripletIterator<T>
where
    T: Iterator<Item = bool>,
{
    fn from(bits: T) -> Self {
        Self { bits }
    }
}

impl<T> Iterator for TripletIterator<T>
where
    T: Iterator<Item = bool>,
{
    type Item = Triplet;

    fn next(&mut self) -> Option<Self::Item> {
        let mut exhausted = false;
        let mut triplet = 0;

        for index in 0..3 {
            match self.bits.next() {
                None => {
                    exhausted = true;
                    break;
                }
                Some(bit) => {
                    triplet += (bit as Triplet) << index;
                }
            }
        }

        if triplet == 0 && exhausted {
            None
        } else {
            Some(triplet)
        }
    }
}
