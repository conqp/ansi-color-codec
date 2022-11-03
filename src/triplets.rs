pub struct Triplets {
    bits: Box<dyn Iterator<Item = bool>>,
    exhausted: bool,
}

impl Triplets {
    pub fn from(bits: impl Iterator<Item = bool> + 'static) -> Self {
        Self {
            bits: Box::new(bits),
            exhausted: false,
        }
    }
}

impl Iterator for Triplets {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut triplet = 0;

        for index in 0..3 {
            match self.bits.next() {
                None => {
                    self.exhausted = true;
                    break;
                }
                Some(bit) => {
                    triplet += (bit as u8) << index;
                }
            }
        }

        if triplet == 0 && self.exhausted {
            None
        } else {
            Some(triplet)
        }
    }
}
