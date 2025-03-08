//exersice 6

/*

fn main() {
    let o = Some(7);

    // Remove the whole `match` block, using `if let` instead 
    match o {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);

            println!("Success!");
        }
        _ => {}
    };
}
*/

fn main() {
    let o = Some(7);
    if let Some(i) = o{
    println!("This is a really long string and `{:?}`", i);
    println!("Success!");}
    
}