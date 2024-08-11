use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Add, BitXor, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct FiniteField {
    pub value: u8
}

// New
impl FiniteField {
    pub fn new(value: u8) -> Self {
        FiniteField { value }
    }
}

// Bitwise Xor
impl BitXor for FiniteField {
    type Output = FiniteField;

    fn bitxor(self, other: FiniteField) -> FiniteField {
        let value: u8 = self.value ^ other.value;
        let finite_field: FiniteField = FiniteField { value };
        return finite_field
    }
}

// Addition
impl Add for FiniteField {
    type Output = FiniteField;

    fn add(self, other: FiniteField) -> FiniteField {
        let sum: u16 = self.value as u16 + other.value as u16;
        let value: u8 = (sum % 256u16) as u8;
        let finite_field: FiniteField = FiniteField { value };
        return finite_field
    }
}

// Negation
impl Neg for FiniteField {
    type Output = FiniteField;

    fn neg(self) -> FiniteField {
        let negative: u16 = (- (self.value as i16) + 256i16) as u16;
        let value: u8 = (negative % 256u16) as u8;
        FiniteField { value }
    }
}

// Subtraction
impl Sub for FiniteField {
    type Output = FiniteField;

    fn sub(self, other: FiniteField) -> FiniteField {
        let finite_field: FiniteField = self + (- other);
        return finite_field
    }
}

// Multiplication
impl Mul for FiniteField {
    type Output = FiniteField;

    fn mul(self, other: FiniteField) -> FiniteField {
        let product: u16 = self.value as u16 * other.value as u16;
        let value: u8 = (product % 256u16) as u8;
        let finite_field: FiniteField = FiniteField { value };
        return finite_field
    }
}

// Inversion
impl FiniteField {
    pub fn inverse(self: FiniteField) -> Option<FiniteField> {
        for i in 1..=255 {
            let candidate: FiniteField = FiniteField::new(i);
            if candidate * self == FiniteField::new(1) {
                return Some(candidate);
            }
        }
        None
    }
}

// Division
impl Div for FiniteField {
    type Output = Option<FiniteField>;

    fn div(self, other: FiniteField) -> Option<FiniteField> {
        let inverse: FiniteField = other.inverse()?;
        let quotient: FiniteField = self * inverse;
        return Some(quotient)
    }
}

// Equality
impl PartialEq for FiniteField {
    fn eq(&self, other: &FiniteField) -> bool {
        let boolean: bool = self.value == other.value;
        return boolean
    }
}

// Byte vector
impl FiniteField {
    fn to_byte_vector(&self) -> Vec<u8> {
        let vector: Vec<u8> = vec![(self.value >> 7) & 1,
                                   (self.value >> 6) & 1,
                                   (self.value >> 5) & 1,
                                   (self.value >> 4) & 1,
                                   (self.value >> 3) & 1,
                                   (self.value >> 2) & 1,
                                   (self.value >> 1) & 1,
                                   (self.value) & 1];
        return vector
    }

    pub fn byte_vector_string(&self) -> String {
        let byte_vector: Vec<u8> = self.to_byte_vector();
        let byte_string: String = byte_vector
            .iter()
            .map(|&bit| bit.to_string())
            .collect::<Vec<String>>()
            .join("");
        return byte_string
    }
}

// Display
impl fmt::Display for FiniteField {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string: String = self.value.to_string();
        write!(formatter, "{}", string)
    }
}
