//exersice 1

/*
// Fix the errors
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
enum Number2 {
    Zero = 0.0,
    One = 1.0,
    Two = 2.0,
}


fn main() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One, Number1::One);
    assert_eq!(Number1::One, Number2::One);

    println!("Success!");
} 
*/

// Fix the errors
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}


fn main() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as u32, Number1::One as u32);
    assert_eq!(Number1::One as u32, Number2::One as u32);

    println!("Success!");
} 