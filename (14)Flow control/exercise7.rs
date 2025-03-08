//exersice 7

/*


// Fill in the blank
fn main() {
    let mut n = 0;
    for i in 0..=100 {
       if n == 66 {
           __
       }
       n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");
}
*/

// Fill in the blank
fn main() {
    let mut n = 0;
    for i in 0..=100 {
       if n == 66 {
           break
       }
       n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");
}