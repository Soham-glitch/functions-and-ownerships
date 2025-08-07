fn main() {
    println!("Hello, world!");
    // s comes into the scope
    let s: String = String::from("Hii, let's get rusty");
    //s's value moved to the function and after that s goes out of scope. Dropped!!!!
    takes_ownership(s);
    
    //x comes into the scope
    let x: i32 = 5;
    // Because x use copy trait so x is not moved to function.
    makes_copy(x);
    //we can use x afterwords also
    println!("printing after copying {x}");

    
    println!("let's understand return values and scopes");
    let ss: String = gives_ownership();
    println!("{ss}");
    let ss1:String = String::from("heyy this is some new program");
    let ss3: String = gives_and_takes_ownership(ss1);
    println!("{ss3}");

    let tuple_some = String::from("hello");
    
    let (tuple2, len) = calculate_length(tuple_some);
    println!("the length of '{tuple2}' is {len}.");

   
}
pub fn takes_ownership(some_string: String){
    println!("{some_string}");
}
pub fn makes_copy(some_value:i32){
    println!("{some_value}");
}

pub fn gives_ownership () -> String{
    let some_string: String = String::from("Hi from some string");
    some_string
}
pub fn gives_and_takes_ownership(a_string:String)->String{
    a_string
}
pub fn calculate_length(s:String) -> (String, usize) {
    let length: usize = s.len();
    (s,length)
}