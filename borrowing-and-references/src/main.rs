fn main() {
    let s1 = String::from("sunil");
    let s2 = &s1; // just borrowing the value of s1 owner still remains as s1 , this is just giving reference of s1 to s2 but cannot mutate the value
    println!("the s1 value is just lented to s2 , s1 is still the owner, 
    this is s1 {} and this is s2 {}" , s1, s2);

    let mut a1  = String::from("Hello");
    let mut a2 = &mut a1; // now a2 can mutate the value of a1


}

// Points to keep in mind 
// 1) & this kind of reference allows the value to used by other variables but cannot mutate
// 2) &mut this kind of refernce allows the value to used and also mutate them 
// 3) if a variable is $mut refernced then it cannot be & referenced to the other variables
// 4) if 