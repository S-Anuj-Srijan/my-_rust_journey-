//exersice 6

/*
// Fix errors without removing any line
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + s2; 
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}
*/
// Fix errors without removing any line
fn main() {
    let mut s1 = String::from("hello,");
    let mut s2 = String::from("world!");
    let mut s3 = s1.clone() + &s2; 
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}
