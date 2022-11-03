pub struct BitsToBytes {
    bits: Box<dyn Iterator<Item = bool>>,
}

impl BitsToBytes {
    pub fn from(bits: impl Iterator<Item = bool> + 'static) -> Self {
        Self {
            bits: Box::new(bits),
        }
    }
}

impl Iterator for BitsToBytes {
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
