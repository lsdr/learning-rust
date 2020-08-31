fn main() {
    struct Structure(i32);
    let test = Structure(3);

    let Structure(encapsulated_value) = test;
    println!("test: {}", encapsulated_value);
    // expects 3
}
