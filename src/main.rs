use crate::key::Key;
use crate::message::Message;
use crate::word::Word;

mod finite_field;
mod key;
mod message;
mod utils;
mod word;

// Simple Example
/*fn main() {
    let text: [Word; 4] = [
        Word::new([0, 1, 2, 3]),
        Word::new([4, 5, 6, 7]),
        Word::new([8, 9, 10, 11]),
        Word::new([12, 13, 14, 15])
    ];
    let key: Key = Key::new([
        Word::new([0, 1, 2, 3]),
        Word::new([4, 5, 6, 7]),
        Word::new([8, 9, 10, 11]),
        Word::new([12, 13, 14, 15]),
        Word::new([16, 17, 18, 19]),
        Word::new([20, 21, 22, 23]),
        Word::new([24, 25, 26, 27]),
        Word::new([28, 29, 30, 31]),
    ]);
    let mut message: Message = Message::new(text, key);
    message.encrypt();
    message.decrypt();
    for index in 0..4usize {
        println!("Word: {}", message.message[index].hexadecimal_representation());
    }
    for index in 0..4 {
        for jndex in 0..4 {
            if message.message[index][jndex] != text[index][jndex] {
                println!("Wrong {} != {}", message.message[index][jndex], text[index][jndex])
            }
        }
    }
}*/

// Completely Random Example
/*fn main() {
    let mut message: Message = Message::random();
    println!("Original Message:");
     for index in 0..4usize {
        println!("Word: {}", message.message[index].hexadecimal_representation());
    }
    message.encrypt();
    println!("Encrypted Message:");
    for index in 0..4usize {
        println!("Word: {}", message.message[index].hexadecimal_representation());
    }
    message.decrypt();
    println!("Decrypted Message:");
    for index in 0..4usize {
        println!("Word: {}", message.message[index].hexadecimal_representation());
    }
}*/

// Example from https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Standards-and-Guidelines/documents/examples/AES_Core256.pdf
fn main() {
    let text: [Word; 4] = [
        Word::new([0x6B, 0xC1, 0xBE, 0xE2]),
        Word::new([0x2E, 0x40, 0x9F, 0x96]),
        Word::new([0xE9, 0x3D, 0x7E, 0x11]),
        Word::new([0x73, 0x93, 0x17, 0x2A])
    ];
    // let text: [Word; 4] = [
    //     Word::new([0xAE, 0x2D, 0x8A, 0x57]),
    //     Word::new([0x1E, 0x03, 0xAC, 0x9C]),
    //     Word::new([0x9E, 0xB7, 0x6F, 0xAC]),
    //     Word::new([0x45, 0xAF, 0x8E, 0x51])
    // ];
    // let text: [Word; 4] = [
    //     Word::new([0x30, 0xC8, 0x1C, 0x46]),
    //     Word::new([0xA3, 0x5C, 0xE4, 0x11]),
    //     Word::new([0xE5, 0xFB, 0xC1, 0x19]),
    //     Word::new([0x1A, 0x0A, 0x52, 0xEF])
    // ];
    // let text: [Word; 4] = [
    //     Word::new([0xF6, 0x9F, 0x24, 0x45]),
    //     Word::new([0xDF, 0x4F, 0x9B, 0x17]),
    //     Word::new([0xAD, 0x2B, 0x41, 0x7B]),
    //     Word::new([0xE6, 0x6C, 0x37, 0x10])
    // ];
    let key: Key = Key::new([
        Word::new([0x60, 0x3D, 0xEB, 0x10]),
        Word::new([0x15, 0xCA, 0x71, 0xBE]),
        Word::new([0x2B, 0x73, 0xAE, 0xF0]),
        Word::new([0x85, 0x7D, 0x77, 0x81]),
        Word::new([0x1F, 0x35, 0x2C, 0x07]),
        Word::new([0x3B, 0x61, 0x08, 0xD7]),
        Word::new([0x2D, 0x98, 0x10, 0xA3]),
        Word::new([0x09, 0x14, 0xDF, 0xF4])
    ]);
    let mut message: Message = Message::new(text, key);
    message.encrypt();
    message.decrypt();
    for index in 0..4usize {
        println!("Word: {}", message.message[index].hexadecimal_representation());
    }
    for index in 0..4 {
        for jndex in 0..4 {
            if message.message[index][jndex] != text[index][jndex] {
                println!("Wrong {} != {}", message.message[index][jndex], text[index][jndex])
            }
        }
    }
}
