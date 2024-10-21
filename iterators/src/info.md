
### Key Points on Rust Iterators

---

#### 1. **Types of Iterators**
- **`iter()`**:
  - Creates an iterator that yields **immutable references** to the elements of a collection.
  - Use for reading elements without modifying them.
  ```rust
  let v = vec![1, 2, 3];
  for val in v.iter() {
      println!("{}", val);  // Immutable references
  }
  ```

- **`iter_mut()`**:
  - Creates an iterator that yields **mutable references**, allowing you to modify elements in place.
  ```rust
  let mut v = vec![1, 2, 3];
  for val in v.iter_mut() {
      *val += 10;  // Modify values in place
  }
  println!("{:?}", v);  // [11, 12, 13]
  ```

- **`into_iter()`**:
  - **Consumes** the collection, taking ownership of its elements. The original collection is no longer usable after calling `into_iter()`.
  - Use when you want to move elements out of the collection.
  ```rust
  let v = vec![1, 2, 3];
  for val in v.into_iter() {
      println!("{}", val);  // Consumes v
  }
  ```

---

#### 2. **Iterator Adaptors** (Non-Consuming)

- **`map()`**:
  - Transforms each element using a closure.
  ```rust
  let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
  println!("{:?}", doubled);  // [2, 4, 6]
  ```

- **`filter()`**:
  - Filters elements based on a condition.
  ```rust
  let evens: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).cloned().collect();
  println!("{:?}", evens);  // [2, 4]
  ```

- **`enumerate()`**:
  - Returns an iterator of pairs where the first element is the index and the second is the value.
  ```rust
  let enumerated: Vec<_> = v.iter().enumerate().collect();
  println!("{:?}", enumerated);  // [(0, 1), (1, 2), (2, 3)]
  ```

- **`take(n)`**:
  - Limits the iterator to the first `n` elements.
  ```rust
  let first_two: Vec<_> = v.iter().take(2).collect();
  println!("{:?}", first_two);  // [1, 2]
  ```

---

#### 3. **Consuming Adaptors** (Consumes the Iterator)

- **`sum()`**:
  - Consumes the iterator and returns the sum of all elements.
  ```rust
  let total: i32 = v.iter().sum();
  println!("{}", total);  // 6
  ```

- **`count()`**:
  - Consumes the iterator and counts the number of elements.
  ```rust
  let count = v.iter().count();
  println!("{}", count);  // 3
  ```

- **`collect()`**:
  - Consumes the iterator and collects the results into a new collection (like `Vec`, `HashMap`, etc.).
  ```rust
  let collected: Vec<i32> = v.iter().map(|x| x + 1).collect();
  println!("{:?}", collected);  // [2, 3, 4]
  ```

---

#### 4. **Common Use Cases**

- **Modify elements in place**: Use `iter_mut()` to modify elements.
- **Immutable iteration**: Use `iter()` to read elements without modifying them.
- **Move and consume**: Use `into_iter()` when you need to take ownership of the elements.
- **Chaining adaptors**: Combine `map()`, `filter()`, and others for powerful transformations.
- **Final results**: Use `collect()`, `sum()`, or `count()` to aggregate or collect the results of an iterator.

---

#### 5. **Performance and Safety**

- **Laziness**: Iterators in Rust are **lazy**—they don’t do any work until they are **consumed** (e.g., by `collect()`, `sum()`, etc.).
- **No Ownership Issues**: Rust’s iterators handle ownership safely by borrowing or moving elements, so there are no dangling references or data races.

---

### Example Summary of Iterator Usage:

```rust
let mut v = vec![1, 2, 3];

// Immutable iteration (iter)
for val in v.iter() {
    println!("Value: {}", val);
}

// Mutable iteration (iter_mut)
for val in v.iter_mut() {
    *val += 1;
}

// Consuming iteration (into_iter)
let sum: i32 = v.into_iter().sum();  // v is now consumed
println!("Sum: {}", sum);
```

---
