fn main() {
    // Example vector for iteration
    let mut v = vec![1, 2, 3, 4, 5];

    // 1. `iter()` - Immutable Iteration
    // Creates an iterator that yields immutable references to the elements of the vector.
    println!("Immutable iteration using iter():");
    for val in v.iter() {
        println!("{}", val);  // val is an immutable reference to the elements
    }

    // 2. `iter_mut()` - Mutable Iteration
    // Creates an iterator that yields mutable references to the elements of the vector.
    println!("\nMutable iteration using iter_mut():");
    for val in v.iter_mut() {
        *val += 10;  // Modify elements in place
    }
    println!("Vector after mutating: {:?}", v);  // [11, 12, 13, 14, 15]

    // 3. `into_iter()` - Consuming the Vector
    // Consumes the collection, taking ownership of each element and yielding the elements themselves.
    println!("\nConsuming iteration using into_iter():");
    let v2 = vec![1, 2, 3, 4, 5];
    for val in v2.into_iter() {
        println!("{}", val);  // val is owned, v2 is moved and cannot be used again
    }

    // 4. Using iterator adaptors (map, filter, etc.)

    // `map()` transforms each element
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("\nDoubled values using map: {:?}", doubled);  // Prints [22, 24, 26, 28, 30]

    // `filter()` retains only elements that satisfy a condition
    let evens: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("\nFiltered even numbers using filter: {:?}", evens);  // Prints [12, 14]

    // `enumerate()` adds an index to each element
    let indexed: Vec<_> = v.iter().enumerate().collect();
    println!("\nEnumerated values using enumerate: {:?}", indexed);  // Prints [(0, 11), (1, 12), (2, 13), (3, 14), (4, 15)]

    // 5. Consuming Adaptors (sum, count, collect)

    // `sum()` consumes the iterator and adds all elements
    let total: i32 = v.iter().sum();
    println!("\nSum of the elements using sum: {}", total);  // Prints 65

    // `count()` consumes the iterator and counts the number of elements
    let count = v.iter().count();
    println!("Count of elements using count: {}", count);  // Prints 5

    // `collect()` converts the iterator back into a collection
    let collected: Vec<i32> = v.iter().map(|x| x + 1).collect();
    println!("Collected values after map: {:?}", collected);  // Prints [12, 13, 14, 15, 16]
}

// Helper function to show usage of iter, iter_mut, and into_iter with return types
fn process_vector(v: &mut Vec<i32>) {
    // Immutable iteration (iter)
    for val in v.iter() {
        println!("Value (immutable): {}", val);
    }

    // Mutable iteration (iter_mut)
    for val in v.iter_mut() {
        *val += 5;  // Modify values in place
    }

    // Consuming iteration (into_iter)
    let sum: i32 = v.clone().into_iter().sum();  // Consumes v and sums values
    println!("Sum after consuming: {}", sum);
}
