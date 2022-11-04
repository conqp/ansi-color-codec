pub type Triplet = u8;

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

pub trait ToColor {
    fn to_color(self) -> String;
}

impl ToColor for Triplet {
    fn to_color(self) -> String {
        format!("\x1b[{}m ", self + 40)
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
