use crate::key::Key;
use crate::word::Word;

pub struct Message {
    pub message: [Word; 4],
    key: Key,
    expanded_key: [Word; 60],
}

// New
impl Message {
    pub fn new(message: [Word; 4], key: Key) -> Message {
        let expanded_key: [Word; 60] = key.expand();
        Message {
            message,
            key,
            expanded_key,
        }
    }
}

// Encrypt
impl Message {
    pub fn encrypt(&mut self) {
        self.add_round_key(0u8);

        for round in 1u8..14u8 {
            self.substitute_bytes();
            self.shift_rows();
            // self.mix_columns();
            self.add_round_key(round)
        }

        self.substitute_bytes();
        self.shift_rows();
        self.add_round_key(14u8);
    }
}

// Decryption
impl Message {
    pub fn decrypt(&mut self) {
        self.add_round_key(14u8);

        for round in (1u8..14u8).rev() {
            self.inv_shift_rows();
            self.inv_substitute_bytes();
            self.add_round_key(round);
            // self.inv_mix_columns();
        }

        self.inv_shift_rows();
        self.inv_substitute_bytes();
        self.add_round_key(0u8);
    }
}

// Add Round Key
impl Message {
    pub fn add_round_key(&mut self, round: u8) {
        let round: usize = round as usize;
        let mut new_message: [Word; 4] = [Word::empty(); 4];
        for index in 0..4 {
            let index: usize = index as usize;
            new_message[index] = self.message[index] ^ self.expanded_key[4 * round + index]
        }
        self.message = new_message
    }
}

// Substitute Bytes
impl Message {
    pub fn substitute_bytes(&mut self) {
        for word in &mut self.message {
            word.substitute_bytes()
        }
    }
}

// Shift Rows
impl Message {
    pub fn shift_rows(&mut self) {
        let mut new_message: [Word; 4] = [Word::empty(); 4];

        new_message[0] = self.message[0];

        new_message[1] = self.message[1].shift_word();

        new_message[2] = self.message[2].shift_word().shift_word();

        new_message[3] = self.message[3].shift_word().shift_word().shift_word();

        self.message = new_message
    }
}

// Mix Columns
impl Message {
    pub fn mix_columns(&mut self) {
        let mut new_message: [Word; 4] = [Word::empty(); 4];
        for index in 0..4 {
            let index: usize = index as usize;
            let mut column: Word = Word::new([
                self.message[0][index],
                self.message[1][index],
                self.message[2][index],
                self.message[3][index],
            ]);
            column.mix_column();
            for jndex in 0..4 {
                let jndex: usize = jndex as usize;
                new_message[jndex][index] = column[jndex];
            }
        }
        self.message = new_message;
    }
}

// Inverse Substitute Bytes
impl Message {
    pub fn inv_substitute_bytes(&mut self) {
        for word in &mut self.message {
            word.inv_substitute_bytes()
        }
    }
}

// Inverse Shift Rows
impl Message {
    pub fn inv_shift_rows(&mut self) {
        let mut new_message: [Word; 4] = [Word::empty(); 4];

        new_message[0] = self.message[0];

        new_message[1] = self.message[1].shift_word().shift_word().shift_word();

        new_message[2] = self.message[2].shift_word().shift_word();

        new_message[3] = self.message[3].shift_word();

        self.message = new_message
    }
}

// Inverse Mix Columns
impl Message {
    pub fn inv_mix_columns(&mut self) {
        let mut new_message: [Word; 4] = [Word::empty(); 4];
        for index in 0..4 {
            let index: usize = index as usize;
            let mut column: Word = Word::new([
                self.message[0][index],
                self.message[1][index],
                self.message[2][index],
                self.message[3][index],
            ]);
            column.inv_mix_column();
            for jndex in 0..4 {
                let jndex: usize = jndex as usize;
                new_message[jndex][index] = column[jndex];
            }
        }
        self.message = new_message;
    }
}
