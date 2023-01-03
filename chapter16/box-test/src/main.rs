use core::fmt;
use std::ops::Deref;

fn main() {
    let a= Box::new(10);

    let x=5;
    let y=Box::new(x);
    println!("Hello, world! {} {}",*a,y);

    let y = MyBox::new(5);

    println!("{}",*y);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");


}
#[derive(Debug,Default)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; // Associated types 

    fn deref(&self) -> &Self::Target {
        println!("{:p}",&self.0);
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}




