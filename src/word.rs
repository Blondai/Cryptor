use crate::finite_field::FiniteField;
use crate::utils::{s_box, s_box_inv};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::ops::{BitXor, Index, IndexMut};

#[derive(Copy, Clone)]
pub struct Word {
    pub values: [u8; 4],
}

// New
impl Word {
    pub fn new(values: [u8; 4]) -> Word {
        Word { values }
    }
}

// Bitwise Xor
impl BitXor for Word {
    type Output = Word;

    fn bitxor(self, other: Word) -> Word {
        let mut new_values: [u8; 4] = [0u8; 4];
        for index in 0..self.values.len() {
            new_values[index] = self.values[index] ^ other.values[index]
        }
        Word::new(new_values)
    }
}

// Indexing
impl Index<usize> for Word {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

// Mutable Indexing
impl IndexMut<usize> for Word {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

// Substitution
impl Word {
    pub fn substitute_bytes(&mut self) {
        for byte in &mut self.values {
            *byte = s_box(*byte);
        }
    }
}

// Shift Word
impl Word {
    pub fn shift_word(&mut self) {
        let mut new_values: [u8; 4] = [0u8; 4];
        new_values[0] = self.values[1];
        new_values[1] = self.values[2];
        new_values[2] = self.values[3];
        new_values[3] = self.values[0];
        self.values = new_values
    }
}

// Mix Column
impl Word {
    pub fn mix_column(&mut self) {
        const TWO: FiniteField = FiniteField::new_const(2u8);
        const THREE: FiniteField = FiniteField::new_const(3u8);
        let self0: FiniteField = FiniteField::new(self[0]);
        let self1: FiniteField = FiniteField::new(self[1]);
        let self2: FiniteField = FiniteField::new(self[2]);
        let self3: FiniteField = FiniteField::new(self[3]);

        let new_values0: FiniteField = (TWO * self0) ^ (THREE * self1) ^ self2 ^ self3;
        let new_values1: FiniteField = self0 ^ (TWO * self1) ^ (THREE * self2) ^ self3;
        let new_values2: FiniteField = self0 ^ self1 ^ (TWO * self2) ^ (THREE * self3);
        let new_values3: FiniteField = (THREE * self0) ^ self1 ^ self2 ^ (TWO * self3);
        let new_values: [u8; 4] = [
            new_values0.value,
            new_values1.value,
            new_values2.value,
            new_values3.value,
        ];
        self.values = new_values
    }
}

// Inverse Substitution
impl Word {
    pub fn inv_substitute_bytes(&mut self) {
        for byte in &mut self.values {
            *byte = s_box_inv(*byte);
        }
    }
}

// Inverse Mix Column
impl Word {
    pub fn inv_mix_column(&mut self) {
        const ZERO_E: FiniteField = FiniteField::new_const(14u8);
        const ZERO_B: FiniteField = FiniteField::new_const(11u8);
        const ZERO_D: FiniteField = FiniteField::new_const(13u8);
        const NINE: FiniteField = FiniteField::new_const(9u8);
        let self0: FiniteField = FiniteField::new(self[0]);
        let self1: FiniteField = FiniteField::new(self[1]);
        let self2: FiniteField = FiniteField::new(self[2]);
        let self3: FiniteField = FiniteField::new(self[3]);

        let new_values0: FiniteField =
            (ZERO_E * self0) ^ (ZERO_B * self1) ^ (ZERO_D * self2) ^ (NINE * self3);
        let new_values1: FiniteField =
            (NINE * self0) ^ (ZERO_E * self1) ^ (ZERO_B * self2) ^ (ZERO_D * self3);
        let new_values2: FiniteField =
            (ZERO_D * self0) ^ (NINE * self1) ^ (ZERO_E * self2) ^ (ZERO_B * self3);
        let new_values3: FiniteField =
            (ZERO_B * self0) ^ (ZERO_D * self1) ^ (NINE * self2) ^ (ZERO_E * self3);
        let new_values: [u8; 4] = [
            new_values0.value,
            new_values1.value,
            new_values2.value,
            new_values3.value,
        ];
        self.values = new_values
    }
}

// Empty Word
impl Word {
    pub fn empty() -> Word {
        Word::new([0u8, 0u8, 0u8, 0u8])
    }
}

// Random Word
impl Word {
    pub fn random() -> Word {
        let mut rng: ThreadRng = rand::thread_rng();
        let bit_0: u8 = rng.gen();
        let bit_1: u8 = rng.gen();
        let bit_2: u8 = rng.gen();
        let bit_3: u8 = rng.gen();
        Word::new([
            bit_0,
            bit_1,
            bit_2,
            bit_3
        ])
    }
}

// Binary Representation
impl Word {
    pub fn binary_representation(&self) -> String {
        let mut string: String = String::new();
        string += "[";
        for index in 0..self.values.len() {
            string += &*format!("{:08b}", self.values[index]);
            if index < self.values.len() - 1 {
                string += ", "
            } else if index == self.values.len() - 1 {
                string += "]"
            }
        }
        string
    }
}

// Hexadecimal Representation
impl Word {
    pub fn hexadecimal_representation(&self) -> String {
        let mut string: String = String::new();
        for index in 0..self.values.len() {
            string += &*format!("{:02x}", self.values[index]);
            if index < self.values.len() - 1 {
                string += " "
            }
        }
        string
    }
}

// Display
impl Word {
    pub fn to_string(&self) -> String {
        let mut string: String = String::new();
        string += "[";
        for index in 0..self.values.len() {
            string += &*format!("{}", self.values[index]);
            if index < self.values.len() - 1 {
                string += ", "
            } else if index == self.values.len() - 1 {
                string += "]"
            }
        }
        string
    }
}
