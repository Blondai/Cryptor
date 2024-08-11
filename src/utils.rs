
pub fn print_dash(num: u8) {
    let mut string: String = String::new();
    for _ in 0..num {
        string += "-"
    }
    println!("{}", string);
}