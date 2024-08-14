use crate::word::Word;

pub fn test_word() {
    test_xor()
}

fn test_xor() {
    let word_1: Word = Word::new([0b10010110, 0b01011101, 0b10110101, 0b00101110]);
    println!("word_1 = {}", word_1.binary_representation());
    let word_2: Word = Word::new([0b10110100, 0b10110101, 0b11011101, 0b00000111]);
    println!("word_2 = {}", word_2.binary_representation());
    let xor: Word = word_1 ^ word_2;
    println!("word_1 ^ word_2 = {}", xor.binary_representation())
}
