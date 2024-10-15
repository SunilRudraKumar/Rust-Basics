// result is used for error handling 
// -> lets you return either return Ok or Err Value


use std::fs::read_to_string;


fn main() {
    let result = read_to_string_custom( String::from("a.txt"));
    println!("{:?}", result)
} 

fn read_to_string_custom(file_path: String) -> Result<String, String>{
    let result = read_to_string(file_path);
    match result{
        Ok(data) => Ok(data),
        Err(err) => Err(String::from("File not Found"))       
    }
}