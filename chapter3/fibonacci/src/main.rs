use std::io;
use std::io::Write;

fn fibonacci(n: u32) -> u32 {
    if n<=1 {
        n
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}

fn main() {
    
    loop {
    println!("-----------");
    print!("Input a number: ");
    
    io::stdout().flush().expect("Error flushing stdout");
    


    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Stdin error");
    let number: u32 = match number.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Not a number");
            continue;
        }
    };

    println!("Fibonacci: {} ",fibonacci(number));

    }
}
