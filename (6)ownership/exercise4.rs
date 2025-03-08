//exersice 4

/*
// Fix the error without removing any code
fn main() {
    let s = String::from("Hello World");

    print_str(s);

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}
*/
// Fix the error without removing any code
fn main() {
    let s = String::from("Hello World");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}