//exersice 2
// Make it work

/*
fn main() {
    let c1 = "中";
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}
*/
// Make it work

fn main() {
    let c1 = "中";//"" means a str '' means a char 
    print_char(c1);
} 

fn print_char(c : &str) {
    println!("{}", c);
}