pub trait Triplets: Iterator<Item = bool> {
    fn triplets(self) -> TripletIterator;
}

impl<T> Triplets for T
where
    T: Iterator<Item = bool> + 'static,
{
    fn triplets(self) -> TripletIterator {
        TripletIterator::from(self)
    }
}

pub struct TripletIterator {
    bits: Box<dyn Iterator<Item = bool>>,
}

impl<T> From<T> for TripletIterator
where
    T: Iterator<Item = bool> + 'static,
{
    fn from(bits: T) -> Self {
        Self {
            bits: Box::new(bits),
        }
    }
}

impl Iterator for TripletIterator {
    type Item = u8;

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
                    triplet += (bit as u8) << index;
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
