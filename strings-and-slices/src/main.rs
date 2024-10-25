fn main() {
    // Creating a String using `String::new()`
    let mut s = String::new();
    s.push_str("Hello");
    s.push(' ');
    s.push_str("World!");

    // Printing the String
    println!("{}", s); // Outputs: Hello World!

    // Strings initialized with a value using the `to_string()` method
    let s = "Hello, Rust!".to_string();
    println!("{}", s); // Outputs: Hello, Rust!

    // Accessing Characters in a String by Iterating
    let s = "Hello";
    for c in s.chars() {
        println!("{}", c); // Outputs each character in a new line: H e l l o
    }

    // Using bytes() method to access raw bytes of the string
    for b in s.bytes() {
        println!("{}", b); // Outputs each byte of the string
    }

    // Slicing a String
    let s = "Hello, Rust!";
    let hello = &s[0..5];
    println!("{}", hello); // Outputs: Hello

    // Note: Slicing works only on valid UTF-8 character boundaries.
    // Attempting to slice in the middle of a multi-byte character will cause a panic.
}
