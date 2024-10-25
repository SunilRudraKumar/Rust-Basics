---

# Rust Strings and Slices

## 1. Strings

Rust has two main types of strings:
- **`String`**: A growable, heap-allocated string type.
- **`&str`** (String Slice): An immutable view into a string data, either part of a `String` or a string literal.

### Creating Strings

- **Empty String**:
  ```rust
  let s = String::new();
  ```

- **From a String Literal**:
  ```rust
  let s = "Hello".to_string();  // Converts &str to String
  ```

- **Using `String::from`**:
  ```rust
  let s = String::from("Hello, Rust!");
  ```

### String Methods

1. **Appending to a String**
   - **`push_str()`**: Appends a string slice (`&str`) to a `String`.
     ```rust
     let mut s = String::from("Hello");
     s.push_str(", World!");  // s becomes "Hello, World!"
     ```

   - **`push()`**: Appends a single character to a `String`.
     ```rust
     let mut s = String::from("Hello");
     s.push('!');  // s becomes "Hello!"
     ```

2. **Concatenation**
   - **`+` Operator**: Concatenates strings, consuming the first operand.
     ```rust
     let s1 = String::from("Hello, ");
     let s2 = String::from("Rust!");
     let s3 = s1 + &s2;  // s1 is moved and cannot be used
     println!("{}", s3);  // Outputs: "Hello, Rust!"
     ```

   - **`format!` Macro**: Useful for concatenating multiple strings without consuming any.
     ```rust
     let s1 = String::from("Hello");
     let s2 = String::from("Rust");
     let s3 = format!("{}, {}!", s1, s2);  // Outputs: "Hello, Rust!"
     ```

3. **Accessing Characters**
   - Rust doesn’t allow direct indexing (`s[0]`) due to variable-width UTF-8 encoding. Use these methods instead:
     - **`chars()`**: Returns an iterator over characters.
       ```rust
       let s = String::from("Hello");
       for c in s.chars() {
           println!("{}", c);  // Prints: H e l l o (each on a new line)
       }
       ```

     - **`bytes()`**: Returns an iterator over bytes.
       ```rust
       let s = String::from("Hello");
       for b in s.bytes() {
           println!("{}", b);  // Prints each byte as an integer
       }
       ```

4. **Slicing Strings**
   - String slices (`&str`) can be created using the slice syntax `&s[start..end]`.
   - **Note**: Slicing must respect UTF-8 character boundaries.

   ```rust
   let s = String::from("Hello, Rust!");
   let hello = &s[0..5];  // "Hello"
   println!("{}", hello);
   ```

5. **Checking String Properties**
   - **`is_empty()`**: Checks if the string is empty.
     ```rust
     let s = String::new();
     println!("{}", s.is_empty());  // Outputs: true
     ```

   - **`len()`**: Returns the length in bytes.
     ```rust
     let s = String::from("Hello");
     println!("{}", s.len());  // Outputs: 5
     ```

---

## 2. Slices

A **slice** is a reference to a contiguous sequence in a collection (such as arrays, vectors, and strings) without owning the data.

### Slice Types

1. **String Slices (`&str`)**:

   - Represent an immutable view into a string, allowing partial access without ownership.
   - Useful for functions that only need a read-only reference to a part of a string.

   ```rust
   let s = String::from("Hello, Rust!");
   let hello = &s[0..5];  // Slices from index 0 to 5 (exclusive)
   println!("{}", hello);  // Outputs: "Hello"
   ```

2. **Array/Vectors Slices (`&[T]`)**:

   - Used to reference a subset of an array or vector.
   - Similar syntax to string slices, but applied to arrays or vectors.

   ```rust
   let arr = [1, 2, 3, 4, 5];
   let slice = &arr[1..4];
   println!("{:?}", slice);  // Outputs: [2, 3, 4]
   ```

### Working with Slices

Slices allow functions to accept parts of data structures without requiring full ownership or copying.

```rust
fn print_slice(slice: &[i32]) {
    for val in slice {
        println!("{}", val);
    }
}

let arr = [10, 20, 30, 40, 50];
print_slice(&arr[1..4]);  // Prints: 20 30 40
```

### Slice Methods

1. **Length of a Slice**: `len()`

   ```rust
   let v = vec![1, 2, 3, 4];
   let slice = &v[1..3];
   println!("{}", slice.len());  // Outputs: 2
   ```

2. **Checking if a Slice is Empty**: `is_empty()`
   ```rust
   let v: Vec<i32> = Vec::new();
   println!("{}", v.is_empty());  // Outputs: true
   ```

---

## Summary

### Strings

- **`String`**: Growable, heap-allocated string.
- **Common Methods**:
  - `push_str()`, `push()`, `chars()`, `bytes()`, `len()`, `is_empty()`.
- **Concatenation**:
  - Use `+` operator or `format!`.
- **Accessing Characters**:
  - Use `chars()` for character-by-character access.
- **Slicing**:
  - Use `&str[start..end]`, must follow UTF-8 boundaries.

### Slices

- **Types**: `&str` (String Slices) and `&[T]` (Array/Vector Slices).
- **Usage**:
  - Allow partial access to data structures without ownership transfer.
- **Common Methods**:
  - `len()`, `is_empty()`.

---
