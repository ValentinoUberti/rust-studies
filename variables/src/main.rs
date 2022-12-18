fn main() {
    const LINO: u128 = 127*127;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("Constant value: {}",LINO);
    // Shadowing
    {
        let x= x * 2;
        println!("The value of shadowed x is: {x}");
    }

    println!("The value of x is: {x}");

    /*

    let mut spaces = "   ";
    spaces = spaces.len();

    Error: mutate a variable's type is not allowed

    */

    /*
    Rust’s char type is four bytes in size and represents a Unicode Scalar Value

    Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    */

    let tup: (i32, f64, String) = (500, 6.4, String::from("blaaaa"));
    println!("Tuples : {:#?}",tup);
    println!("Tuple's first element : {}",tup.0);


    /*
    Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.
    */

    let _a = [1, 2, 3, 4, 5];
    

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("{}",number);


    /*
    fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

    
    */

}


// &str vs String vs &String
// https://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html