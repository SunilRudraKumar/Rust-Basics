// OPTION ENUM LETS YO RETURN NONE NULL OR NILL

// fucntion can return either a number or a null
// it has option to return other values, if the fucntion was not able to return value 
fn main() {
    let index = find_letter_a(String::from("SunilKumar"));

    match index{
        Some(value ) => println!("{}", value),
        None => println!("a Not Found")
    }
  
}


fn find_letter_a(s:String) -> Option<i32> {

    for (index, char ) in s.chars().enumerate() {
        if char == 'a'{
            return Some(index as i32)
        } 

    
}
return None
}