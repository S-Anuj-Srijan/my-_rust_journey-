//exersice 6

/*
// make the necessary variable mutable
fn main() {
    let s = String::from("Hello ");
    
    let s1 = s;

    s1.push_str("World!");

    println!("Success!");
}
*/
// make the necessary variable mutable
fn main() {
    let s = String::from("Hello ");
    
    let mut s1 = s;

    s1.push_str("World!"); // for push_str() to work the variable s1 must be a String type and mutable (mutable means its changable)

    println!("Success!");
}