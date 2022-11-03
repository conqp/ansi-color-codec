pub struct Code {
    bytes: [u8; 6],
}

impl Code {
    pub fn new(bytes: [u8; 6]) -> Self {
        Self { bytes }
    }

    pub fn number(&self) -> u8 {
        (self.bytes[2] & 0b1111) * 10 + (self.bytes[3] & 0b1111)
    }

    pub fn byte(&self) -> u8 {
        self.number() - 40
    }

    pub fn triplet(&self) -> [bool; 3] {
        let byte = self.byte();
        [byte & 0b001 != 0, byte & 0b010 != 0, byte & 0b100 != 0]
    }
}

pub struct Codes {
    bytes: Box<dyn Iterator<Item = u8>>,
}

impl Codes {
    pub fn from(bytes: impl Iterator<Item = u8> + 'static) -> Self {
        Self {
            bytes: Box::new(bytes),
        }
    }
}

impl Iterator for Codes {
    type Item = Code;

    fn next(&mut self) -> Option<Self::Item> {
        let mut block = [0, 0, 0, 0, 0, 0];

        for num in block.iter_mut() {
            match self.bytes.next() {
                Some(byte) => {
                    *num = byte;
                }
                None => {
                    return None;
                }
            }
        }

        Some(Code::new(block))
    }
}
