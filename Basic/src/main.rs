struct User {
    first_name: String,
    last_name: String,
    age: u32,
}

fn main() {
    println!("Hello, world!");

    // Call isEven function
    println!("{}", isEven(2));

    // Call fib function
    println!("{}", fib(10));

    // Get string length
    let name = String::from("Monica S");
    let len = get_st_length(name);
    println!("{}", len);

    // Create User struct instance
    let user1 = User {
        first_name: String::from("Sunil"),
        last_name: String::from("Kumar"),
        age: 23,
    };

    // Access first_name from user1
    println!("{}", user1.first_name);
}

// Check if a number is even
fn isEven(num: i32) -> bool {
    num % 2 == 0
}

// Get the nth Fibonacci number
fn fib(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if num == first {
        return first;
    }
    if num == second {
        return second;
    }

    for _ in 1..num - 1 {
        let temp = second;
        second = second + first;
        first = temp;
    }

    second
}

// Get the length of a string
fn get_st_length(str: String) -> usize {
    str.chars().count()
}
