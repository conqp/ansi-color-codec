const NUMBER_MASK: u8 = 0b1111;

pub trait ToColorCodes {
    fn codes(self) -> ColorCodeIterator;
}

impl<T> ToColorCodes for T
where
    T: Iterator<Item = u8> + 'static,
{
    fn codes(self) -> ColorCodeIterator {
        ColorCodeIterator::from(self)
    }
}

pub struct ColorCode {
    number: u8,
}

impl ColorCode {
    pub fn new(number: u8) -> Self {
        Self { number }
    }

    pub fn byte(&self) -> u8 {
        self.number - 40
    }

    pub fn triplet(&self) -> [bool; 3] {
        let byte = self.byte();
        [byte & 0b001 != 0, byte & 0b010 != 0, byte & 0b100 != 0]
    }

    pub fn triplets(&self) -> Box<dyn Iterator<Item = bool>> {
        Box::new(self.triplet().into_iter())
    }
}

pub struct ColorCodeIterator {
    bytes: Box<dyn Iterator<Item = u8>>,
}

impl<T> From<T> for ColorCodeIterator
where
    T: Iterator<Item = u8> + 'static,
{
    fn from(bytes: T) -> Self {
        Self {
            bytes: Box::new(bytes),
        }
    }
}

impl Iterator for ColorCodeIterator {
    type Item = ColorCode;

    fn next(&mut self) -> Option<Self::Item> {
        let mut digits = Vec::new();

        if self.bytes.next().unwrap_or(0) != 0x1b {
            return None;
        }

        if self.bytes.next().unwrap_or(0) as char != '[' {
            return None;
        }

        loop {
            match self.bytes.next() {
                Some(byte) => {
                    if byte.is_ascii_digit() {
                        digits.push(byte);
                    } else if byte as char == 'm' {
                        break;
                    } else {
                        return None;
                    }
                }
                None => {
                    return None;
                }
            }
        }

        if self.bytes.next().unwrap_or(0) != 32 {
            return None;
        }

        match digits
            .iter()
            .enumerate()
            .map(|(index, digit)| {
                (digit & NUMBER_MASK) * (10_u8.pow((digits.len() - index - 1) as u32))
            })
            .sum()
        {
            0 => None,
            sum => Some(ColorCode::new(sum)),
        }
    }
}
