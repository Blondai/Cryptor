use crate::key::Key;
use crate::word::Word;

pub fn test_key() {
    let initial_key: [Word; 8] = [
        Word::new([0x00, 0x01, 0x02, 0x03]),
        Word::new([0x04, 0x05, 0x06, 0x07]),
        Word::new([0x08, 0x09, 0x0A, 0x0B]),
        Word::new([0x0C, 0x0D, 0x0E, 0x0F]),
        Word::new([0x10, 0x11, 0x12, 0x13]),
        Word::new([0x14, 0x15, 0x16, 0x17]),
        Word::new([0x18, 0x19, 0x1A, 0x1B]),
        Word::new([0x1C, 0x1D, 0x1E, 0x1F])
    ];
    let key: Key = Key::new(initial_key);
    let expanded_key: [Word; 60] = key.expand();
    for word in expanded_key {
        println!("{}", word.to_string())
    }

}



