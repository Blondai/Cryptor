use crate::key::Key;
use crate::message::Message;
use crate::word::Word;

mod utils;
mod word;
mod finite_field;
mod key;
mod message;


fn main() {
    let initial_message: [Word; 4] = [
        Word::new([0, 1, 2, 3]),
        Word::new([4, 5, 6, 7]),
        Word::new([8, 9, 10, 11]),
        Word::new([12, 13, 14, 15]),
    ];
    let key: Key = Key::random();
    let mut message = Message::new(initial_message, key);
    println!("Original:");
    for value in message.message {
        println!("{}", value.to_string())
    }

    message.encrypt();
    println!("Encrypted:");
    for value in message.message {
        println!("{}", value.to_string())
    }

    println!("Decrypted:");
    message.decrypt();
    for value in message.message {
        println!("{}", value.to_string())
    }

    message.mix_columns();
    println!("Mixed:");
    message.mix_columns();
    for value in message.message {
        println!("{}", value.to_string())
    }

    message.inv_mix_columns();
    println!("Unmixed:");
    message.inv_mix_columns();
    for value in message.message {
        println!("{}", value.to_string())
    }
}
