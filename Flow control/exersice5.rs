//exersice 5

/*
fn main() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i,v) in a.__ {
        println!("The {}th element is {}",i+1,v);
    }
}
*/
fn main() {
   let a = [4, 3, 2, 1];

   // Iterate the indexing and value in 'a'
   for (i,v) in a.iter().enumerate() {
       println!("The {}th element is {}",i+1,v);
   }
}