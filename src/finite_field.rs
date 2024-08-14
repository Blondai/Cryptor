use std::ops::{BitXor, Mul};

#[derive(Copy, Clone)]
pub struct FiniteField {
    pub value: u8
}

// New
impl FiniteField {
    pub fn new(value: u8) -> Self {
        FiniteField { value }
    }

    pub const fn new_const(value: u8) -> Self {
        FiniteField {value}
    }
}

// Bitwise Xor
impl BitXor for FiniteField {
    type Output = FiniteField;

    fn bitxor(self, other: FiniteField) -> FiniteField {
        let value: u8 = self.value ^ other.value;
        FiniteField { value }
    }
}

// Multiplication
impl Mul for FiniteField {
    type Output = FiniteField;

    fn mul(self, other: FiniteField) -> FiniteField {
        const POLYNOMIAL: u16 = 0x11B;
        let mut product: u16 = 0u16;
        let mut self_value: u16 = self.value as u16;
        let mut other_value: u16 = other.value as u16;
        while other_value > 0 {
            if other_value & 1 != 0 {
                product ^= self_value;
            }
            self_value <<= 1;
            other_value >>= 1;
        }
        while product & 0x100 != 0 {
            product ^= POLYNOMIAL;
        }

        FiniteField::new(product as u8)
    }
}
