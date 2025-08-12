use rand::Rng;
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

    let rand_val = use_random();
    println!("{rand_val}");
  //borrowing the string from the function
    let s_cal = String::from("helllllllloooo");
    //only taking the reference of the string. not the actual string is moving to the function. only pointing to the address of the string instance.
    let s_cal2 = calculate_len_of_tupple(&s_cal);
    println!("the size of the tupple is {s_cal2}");

    let mut some_string = String::from("hellow!");
    borrow_some_val(&mut some_string);
    let reference_to_nothing = dangle();
    println!("{reference_to_nothing}");

    //one string with two words
   let mut string = String::from("hi from hello");
    let word = two_string_of_words(&string);
    println!("{word}");
    string.clear();
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
pub fn calculate_length(s: String) -> (String, usize) {
    let length: usize = s.len();
    (s,length)
}
pub fn use_random()->i32{
    let a: i32 = rand::thread_rng().gen_range(1..=100);
    a
}
//understanding borrowing
pub fn calculate_len_of_tupple( s: &String)-> usize {
    s.len()
}
//exmp of borrowing
pub fn borrow_some_val(s: &mut String){
    s.push_str(", let's push it");
    println!("{s}");
    
}
//example of a dangling pointer that reference to noting

pub fn dangle ()-> String {
    let s = String::from("dangle the string");
    s
}
pub fn two_string_of_words(s: &String) -> usize{
    let arr_of_bytes: &[u8] = s.as_bytes();
    for (i, &item) in arr_of_bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }

    s.len()
}