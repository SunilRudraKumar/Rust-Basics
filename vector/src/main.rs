fn main() {
    // Creating a Vector

    // 1. Using Vec::new() and pushing elements
    let mut v: Vec<i32> = Vec::new(); // Empty vector of type i32
    v.push(10);
    v.push(25);
    v.push(54);
    println!("{:?}", v); // Prints [10, 25, 54]

    // 2. Using vec! Macro
    let v = vec![1, 2, 3, 4, 5]; // Vector initialized with values
    println!("{:?}", v); // Prints [1, 2, 3, 4, 5]

    // Accessing Elements

    // 1. Indexing
    let v = vec![1, 2, 3, 4];
    let third = v[2]; // Accessing the third element (index starts at 0)
    println!("The value at index 2 is {}", third); // Prints "The value at index 2 is 3"

    // 2. Using get(): Safely access with Option
    let third_option = v.get(2); // Returns Some(&3) or None if out of bounds
    match third_option {
        Some(&value) => println!("The value at index 2 is {}", value), // Prints 3
        None => println!("There is no element at index 2"),
    }

    // 3. Handling out-of-bounds access with get()
    let out_of_bounds = v.get(10); // Returns None safely
    match out_of_bounds {
        Some(&value) => println!("The value at index 10 is {}", value),
        None => println!("There is no element at index 10"),
    }

    // Iterating over a Vector

    // 1. Immutable Iteration
    let v = vec![1, 2, 3, 4];
    for i in &v {
        println!("Immutable iteration: {}", i);
    }

    // 2. Mutable Iteration (modifying the elements)
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 10; // Modify elements by dereferencing the mutable reference
    }
    println!("After modifying: {:?}", v); // Prints [11, 12, 13]

    // 3. Consuming the Vector (moves the values out)
    let v = vec![1, 2, 3];
    for i in v.into_iter() {
        println!("Consuming iteration: {}", i);
    }
    // After this, 'v' cannot be used because it's moved.

    // Adding and Removing Elements

    // 1. Push - Adds an element to the end
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("After push: {:?}", v); // Prints [1, 2, 3, 4]

    // 2. Pop - Removes and returns the last element
    v.pop(); // Removes the last element (4)
    println!("After pop: {:?}", v); // Prints [1, 2, 3]

    // 3. Removing by Index
    let mut v = vec![1, 2, 3, 4];
    v.remove(2); // Removes the element at index 2 (the third element), vector becomes [1, 2, 4]
    println!("After remove: {:?}", v); // Prints [1, 2, 4]

    // 4. Retaining Elements
    let mut v = vec![1, 2, 3, 4];
    v.retain(|&x| x > 2); // Only retains elements greater than 2
    println!("After retain: {:?}", v); // Prints [3, 4]

    // Slicing a Vector

    let v = vec![1, 2, 3, 4, 5];
    let slice = &v[1..3]; // Slice contains elements from index 1 to 3 (not inclusive)
    println!("Slice: {:?}", slice); // Prints [2, 3]

    // Common Methods

    // 1. Length of a vector
    let v = vec![1, 2, 3, 4];
    println!("Length: {}", v.len()); // Prints "Length: 4"

    // 2. Check if the vector is empty
    let v: Vec<i32> = Vec::new();
    println!("Is empty: {}", v.is_empty()); // Prints "Is empty: true"

    // 3. Clear the vector (removes all elements)
    let mut v = vec![1, 2, 3, 4];
    v.clear(); // Now the vector is empty
    println!("After clear: {:?}", v); // Prints "After clear: []"

   // Passing a new vector to is_even
   let v = vec![1, 2, 3, 4, 5, 6];
   let ans = is_even(&v);  // Pass the vector by reference
   println!("The even vec is {:?}", ans); // Prints "The even vec is [2, 4, 6]"

}


// Function that returns even numbers
fn is_even(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();  // Start with an empty vector
    for &val in vec {  // Dereference the value
        if val % 2 == 0 {
            new_vec.push(val);
        }
    }
    return new_vec;
}