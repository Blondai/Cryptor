use crate::utils;
use crate::word::Word;

pub struct Key {
    pub initial_key: [Word; 8],
}

// New
impl Key {
    pub fn new(initial_key: [Word; 8]) -> Key {
        Key { initial_key }
    }
}

// Key Expansion
impl Key {
    pub fn expand(&self) -> [Word; 60] {
        const N: usize = 8usize;
        const R: usize = 15usize;
        let initial_key: [Word; N] = self.initial_key;
        let mut expanded_words: [Word; 4 * R] = [Word::empty(); 4 * R];
        for index in 0..4 * R {
            if index < N {
                expanded_words[index] = initial_key[index]
            } else if index >= N && index % N == 0 {
                expanded_words[index] = expanded_words[index - N]
                    ^ substitute_word(rotate_word(expanded_words[index - 1]))
                    ^ get_round_constant(index / N)
            } else if index >= N && N > 6 && index % N == 4 {
                expanded_words[index] =
                    expanded_words[index - N] ^ substitute_word(expanded_words[index - 1])
            } else {
                expanded_words[index] = expanded_words[index - N] ^ expanded_words[index - 1]
            }
        }
        expanded_words
    }
}

// Auxiliary Functions
fn substitute_word(word: Word) -> Word {
    Word::new([
        utils::s_box(word[0]),
        utils::s_box(word[1]),
        utils::s_box(word[2]),
        utils::s_box(word[3]),
    ])
}

fn rotate_word(word: Word) -> Word {
    Word::new([word[1], word[2], word[3], word[0]])
}

fn get_round_constant(index: usize) -> Word {
    const ROUND_CONSTANT: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];
    Word::new([ROUND_CONSTANT[index - 1], 0x00, 0x00, 0x00])
}

// Empty Key
impl Key {
    pub fn empty() -> Key {
        Key::new([
            Word::new([0x00, 0x01, 0x02, 0x03]),
            Word::new([0x04, 0x05, 0x06, 0x07]),
            Word::new([0x08, 0x09, 0x0A, 0x0B]),
            Word::new([0x0C, 0x0D, 0x0E, 0x0F]),
            Word::new([0x10, 0x11, 0x12, 0x13]),
            Word::new([0x14, 0x15, 0x16, 0x17]),
            Word::new([0x18, 0x19, 0x1A, 0x1B]),
            Word::new([0x1C, 0x1D, 0x1E, 0x1F]),
        ])
    }
}

// Random Key
impl Key {
    pub fn random() -> Key {
        Key::new([
            Word::random(),
            Word::random(),
            Word::random(),
            Word::random(),
            Word::random(),
            Word::random(),
            Word::random(),
            Word::random(),
        ])
    }
}
