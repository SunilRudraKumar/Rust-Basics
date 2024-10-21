use std::collections::HashMap;

fn main() {
    // Creating a HashMap
    let mut marks = HashMap::new();

    // Inserting key-value pairs into the HashMap
    marks.insert(String::from("Math"), 85);
    marks.insert(String::from("Science"), 90);
    marks.insert(String::from("English"), 78);

    // Printing the HashMap
    println!("Marks HashMap: {:?}", marks); // Prints the HashMap

    // Accessing a value using a key
    let subject = String::from("Math");
    if let Some(mark) = marks.get(&subject) {
        println!("The mark for {} is {}", subject, mark);
    } else {
        println!("No marks found for {}", subject);
    }

    // Iterating over the HashMap
    for (subject, mark) in &marks {
        println!("Subject: {}, Mark: {}", subject, mark);
    }

    // Modifying a value
    marks.insert(String::from("Math"), 95); // Updating the mark for Math
    println!("Updated Marks HashMap: {:?}", marks);

    // Removing an entry
    marks.remove(&String::from("English"));
    println!("After removing English: {:?}", marks);

    // Passing the HashMap to a function that filters subjects with marks above 80
    let high_marks = filter_high_marks(&marks);
    println!("Subjects with high marks: {:?}", high_marks);
}

// Function to filter subjects with marks greater than 80
fn filter_high_marks(map: &HashMap<String, i32>) -> HashMap<String, i32> {
    let mut filtered_map = HashMap::new();
    for (subject, &mark) in map {
        if mark > 80 {
            filtered_map.insert(subject.clone(), mark);
        }
    }
    filtered_map
}
