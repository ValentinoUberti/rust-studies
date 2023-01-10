use macros::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello Macro Test")
    }
}

fn main() {
    Pancakes::hello_macro();
}