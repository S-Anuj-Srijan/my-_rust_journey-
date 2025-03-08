//exersice 
/*
fn main() {
    assert!(0.1+0.2==0.3);

    println!("Success!");
}
*/

fn main() {
    assert!(0.1 as f32+0.2 as f32==0.3 as f32);// happens deu to floating point precsision when 0.1 and 0.2 get added it is not equal to 0.3 its equal to 0.300000000000 

    println!("Success!");
}