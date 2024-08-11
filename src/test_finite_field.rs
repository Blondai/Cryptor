use crate::utils::print_dash;
use crate::finite_field::FiniteField;


pub fn test_finite_field() {
    test_xor();
    print_dash(20);
    test_addition();
    print_dash(20);
    test_negation();
    print_dash(20);
    test_subtraction();
    print_dash(20);
    test_multiplication();
    print_dash(20);
    test_inversion();
    print_dash(20);
    test_division();
     print_dash(20);
    test_equation_1();
    print_dash(20);
    test_equation_2();
}

fn test_xor() {
    let a: FiniteField = FiniteField::new(0b10011101);
    println!("a = {}", a.byte_vector_string());
    let b: FiniteField = FiniteField::new(0b01011001);
    println!("b = {}", b.byte_vector_string());
    let sum: FiniteField = a ^ b;
    println!("a ^ b = {}", sum.byte_vector_string())
}

fn test_addition() {
    let a: FiniteField = FiniteField::new(221);
    println!("a = {}", a);
    let b: FiniteField = FiniteField::new(112);
    println!("b = {}", b);
    let sum: FiniteField = a + b;
    println!("a + b = {}", sum)
}

fn test_negation() {
    let a: FiniteField = FiniteField::new(167);
    println!("a = {}", a);
    let negative: FiniteField = - a;
    println!("- a = {}", negative)
}

fn test_subtraction() {
        let a: FiniteField = FiniteField::new(17);
    println!("a = {}", a);
    let b: FiniteField = FiniteField::new(98);
    println!("b = {}", b);
    let difference: FiniteField = a - b;
    println!("a - b = {}", difference)
}

fn test_multiplication() {
    let a: FiniteField = FiniteField::new(18);
    println!("a = {}", a);
    let b: FiniteField = FiniteField::new(93);
    println!("b = {}", b);
    let product: FiniteField = a * b;
    println!("a + b = {}", product)
}

fn test_inversion() {
    let a: FiniteField = FiniteField::new(79);
    println!("a = {}", a);
    let inverse: FiniteField = a.inverse().unwrap();
    println!("1 / a = {}", inverse)
}

fn test_division() {
    let a: FiniteField = FiniteField::new(221);
    println!("a = {}", a);
    let b: FiniteField = FiniteField::new(37);
    println!("b = {}", b);
    let quotient: Option<FiniteField> = a / b;
    println!("a / b = {}", quotient.unwrap())
}


fn test_equation_1() {
    let a: FiniteField = FiniteField::new(221);
    println!("a = {}", a);
    let b: FiniteField = FiniteField::new(112);
    println!("b = {}", b);
    let equality: bool = a == b;
    println!("a == b = {}", equality)
}

fn test_equation_2() {
    let a: FiniteField = FiniteField::new(28);
    println!("a = {}", a);
    let b: FiniteField = FiniteField::new(28);
    println!("b = {}", b);
    let equality: bool = a == b;
    println!("a == b = {}", equality)
}
