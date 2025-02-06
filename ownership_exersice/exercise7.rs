//exersice 7

/*
fn main() {
    let x = Box::new(5);
    
    let ...      // update this line, don't change other lines!
    
    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}
*/
fn main() {
    let x = Box::new(5);
    
    let mut y = Box::new(1);     // update this line, don't change other lines!
    
    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}