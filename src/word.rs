use std::ops::{BitXor, Index};

#[derive(Copy, Clone, Debug)]
pub struct Word {
    values: [u8; 4]
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

// Empty Word
impl Word {
    pub fn empty() -> Word {
        Word::new([0u8, 0u8, 0u8, 0u8])
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
            }
            else if index == self.values.len() - 1 {
                string += "]"
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
            }
            else if index == self.values.len() - 1 {
                string += "]"
            }
        }
        string
    }
}
