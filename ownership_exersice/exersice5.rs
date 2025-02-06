//exersice 5

/*
// Don't use clone ,use copy instead
fn main() {
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}
*/
// Don't use clone ,use copy instead
fn main() {
   let x = (1, 2, (), "hello");//"hello" is &str type which means it is stored in the stack and is imutable whereas "hello".to_string() is a String type which is stored in the heap memory 
   let y = x;
   println!("{:?}, {:?}", x, y);
}