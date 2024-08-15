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
            self.mix_columns();
            self.add_round_key(round)
        }

        self.substitute_bytes();
        self.shift_rows();
        self.add_round_key(14u8);
    }
}

// Decrypt
impl Message {
    pub fn decrypt(&mut self) {
        self.add_round_key(14u8);
        self.inv_substitute_bytes();
        self.inv_shift_rows();

        for round in (1u8..14u8).rev() {
            self.add_round_key(round);
            self.inv_mix_columns();
            self.inv_substitute_bytes();
            self.inv_shift_rows();
        }

        self.add_round_key(0u8);
    }

    /*  pub fn decrypt_old(&mut self) {
        self.add_round_key(14u8);

        for round in (1u8..14u8).rev() {
            self.inv_shift_rows();
            self.inv_substitute_bytes();
            self.add_round_key(round);
            self.inv_mix_columns();
        }

        self.inv_shift_rows();
        self.inv_substitute_bytes();
        self.add_round_key(0u8);
    }*/
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
        let mut row_0: Word = Word::empty();
        let mut row_1: Word = Word::empty();
        let mut row_2: Word = Word::empty();
        let mut row_3: Word = Word::empty();
        for index in 0..4usize {
            row_0[index] = self.message[index][0];
            row_1[index] = self.message[index][1];
            row_2[index] = self.message[index][2];
            row_3[index] = self.message[index][3];
        }
        row_1.shift_word();
        row_2.shift_word();
        row_2.shift_word();
        row_3.shift_word();
        row_3.shift_word();
        row_3.shift_word();
        for index in 0..4usize {
            self.message[index][0] = row_0[index];
            self.message[index][1] = row_1[index];
            self.message[index][2] = row_2[index];
            self.message[index][3] = row_3[index]
        }
    }
}

// Mix Columns
impl Message {
    pub fn mix_columns(&mut self) {
        self.message[0].mix_column();
        self.message[1].mix_column();
        self.message[2].mix_column();
        self.message[3].mix_column();
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
        let mut row_0: Word = Word::empty();
        let mut row_1: Word = Word::empty();
        let mut row_2: Word = Word::empty();
        let mut row_3: Word = Word::empty();
        for index in 0..4usize {
            row_0[index] = self.message[index][0];
            row_1[index] = self.message[index][1];
            row_2[index] = self.message[index][2];
            row_3[index] = self.message[index][3];
        }
        row_1.shift_word();
        row_1.shift_word();
        row_1.shift_word();
        row_2.shift_word();
        row_2.shift_word();
        row_3.shift_word();
        for index in 0..4usize {
            self.message[index][0] = row_0[index];
            self.message[index][1] = row_1[index];
            self.message[index][2] = row_2[index];
            self.message[index][3] = row_3[index]
        }
    }
}

// Inverse Mix Columns
impl Message {
    pub fn inv_mix_columns(&mut self) {
        self.message[0].inv_mix_column();
        self.message[1].inv_mix_column();
        self.message[2].inv_mix_column();
        self.message[3].inv_mix_column();
    }
}

// Random Message
impl Message {
    pub fn random() -> Message {
        let key: Key = Key::random();
        let text: [Word; 4] = [
            Word::random(),
            Word::random(),
            Word::random(),
            Word::random(),
        ];
        Message::new(text, key)
    }
}
