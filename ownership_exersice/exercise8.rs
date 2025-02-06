//exersice 8

/*
fn main() {
   let t = (String::from("hello"), String::from("world"));

   let _s = t.0;

   // Modify this line only, don't use `_s`
   println!("{:?}", t);
}
*/
fn main() {
    let t = (String::from("hello"), String::from("world"));
    let _s = t.0.clone();
    // Modify this line only, don't use `_s`
    println!("{:?} {:?}",t.0,t.1);
 }