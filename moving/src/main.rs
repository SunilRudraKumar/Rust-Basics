fn main() {
    println!("Hello, world!");
    let a1 = String::from("sunil");
    let a2 = a1;

    println! {"the value of a2 {}", a1} 
    //  Cannot do this as the variable can have only one owner ,
    //  once transfered to a2 the a1 dies
    // this is called moving 
    // can do something like cloning which creates two values in heap and two owners respectively


    // this applies to when you pass a value to fucntion and there also the ownership will be transfered
    let s1 = String::from("sunil");
    // calling fucntion to print 
    println_custom(s1);

    // cannot print s1 as the ownership is moved to another fucntion
    println!("this cannot print the value of s1 {}",s1);
}



fn println_custom(s2:String) {
    println!("The value of s2 {}", s2);
}