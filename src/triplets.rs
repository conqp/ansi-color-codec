pub type Triplet = u8;

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

pub trait ToColor {
    fn to_color(self) -> String;
}

impl ToColor for Triplet {
    fn to_color(self) -> String {
        format!("\x1b[{}m ", self + 40)
    }
}

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
