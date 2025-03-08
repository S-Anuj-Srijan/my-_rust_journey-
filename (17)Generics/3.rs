//exersice 3

/*
// Implement struct Point to make it work.


fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("Success!");
}
*/
// Implement struct Point to make it work.
struct Point<T,U>{
    x:T,
    y:U,

}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("Success!");
}