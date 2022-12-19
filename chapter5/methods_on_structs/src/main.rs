#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // The &self is actually short for self: &Self.
        self.width * self.height
    }    
}

fn main() {
    let rect = Rectangle {
        width: 50,
        height: 50,
    };
    println!("Area: {}",rect.area());
}
