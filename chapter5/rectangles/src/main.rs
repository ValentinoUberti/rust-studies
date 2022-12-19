fn main() {
    let width1= 30;
    let height1= 50;

    let rect1 = (30,50);

    println!("Area = {} ", area(width1,height1));
    println!("Area2 = {} ", area2(rect1));
}

fn area(w: i32,h:i32) -> i32 {
    w*h
}

fn area2(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}