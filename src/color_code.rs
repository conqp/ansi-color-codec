use std::array::IntoIter;

const NUMBER_MASK: u8 = 0b1111;
const COLOR_OFFSET: u8 = 40;

pub trait BytesToColorCodes<T>
where
    T: Iterator<Item = ColorCode>,
{
    fn codes(self) -> T;
}

impl<T> BytesToColorCodes<ColorCodeIterator<T>> for T
where
    T: Iterator<Item = u8>,
{
    fn codes(self) -> ColorCodeIterator<T> {
        ColorCodeIterator::from(self)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ColorCode {
    number: u8,
}

impl ColorCode {
    pub fn new(number: u8) -> Self {
        Self { number }
    }

    pub fn byte(&self) -> u8 {
        self.number - COLOR_OFFSET
    }

    pub fn triplet(&self) -> [bool; 3] {
        (0..3)
            .map(|index| self.byte() & (1 << index) != 0)
            .collect::<Vec<bool>>()
            .try_into()
            .unwrap()
    }

    pub fn triplets(&self) -> IntoIter<bool, 3> {
        self.triplet().into_iter()
    }
}

pub trait ToColor {
    fn to_color(self) -> String;
}

impl ToColor for u8 {
    fn to_color(self) -> String {
        format!("\x1b[{}m ", self + COLOR_OFFSET)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ColorCodeIterator<T>
where
    T: Iterator<Item = u8>,
{
    bytes: T,
}

impl<T> From<T> for ColorCodeIterator<T>
where
    T: Iterator<Item = u8>,
{
    fn from(bytes: T) -> Self {
        Self { bytes }
    }
}

impl<T> Iterator for ColorCodeIterator<T>
where
    T: Iterator<Item = u8>,
{
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

        if self.bytes.next().unwrap_or(0) as char != ' ' {
            return None;
        }

        match digits
            .iter()
            .rev()
            .enumerate()
            .map(|(index, digit)| (digit & NUMBER_MASK) * 10_u8.pow(index as u32))
            .sum()
        {
            0 => None,
            sum => Some(ColorCode::new(sum)),
        }
    }
}
