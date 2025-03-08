//exersice 3

/*
// Fix error
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(s);

    println!("Success!");
}

fn borrow_object(s: &String) {}
*/
// Fix error
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);//s is String data type but &s is &String 

    println!("Success!");
}

fn borrow_object(s: &String) {}