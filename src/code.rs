const NUMBER_MASK: u8 = 0b1111;

pub trait ToCodes: Iterator<Item = u8> {
    fn codes(self) -> CodeIterator;
}

impl<T> ToCodes for T
where
    T: Iterator<Item = u8> + 'static,
{
    fn codes(self) -> CodeIterator {
        CodeIterator::from(self)
    }
}

pub struct Code {
    bytes: [u8; 6],
}

impl Code {
    pub fn new(bytes: [u8; 6]) -> Self {
        Self { bytes }
    }

    pub fn number(&self) -> u8 {
        (self.bytes[2] & NUMBER_MASK) * 10 + (self.bytes[3] & NUMBER_MASK)
    }

    pub fn byte(&self) -> u8 {
        self.number() - 40
    }

    pub fn triplet(&self) -> [bool; 3] {
        let byte = self.byte();
        [byte & 0b001 != 0, byte & 0b010 != 0, byte & 0b100 != 0]
    }

    pub fn triplets(&self) -> Box<dyn Iterator<Item = bool>> {
        Box::new(self.triplet().into_iter())
    }
}

pub struct CodeIterator {
    bytes: Box<dyn Iterator<Item = u8>>,
}

impl<T> From<T> for CodeIterator
where
    T: Iterator<Item = u8> + 'static,
{
    fn from(bytes: T) -> Self {
        Self {
            bytes: Box::new(bytes),
        }
    }
}

impl Iterator for CodeIterator {
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
