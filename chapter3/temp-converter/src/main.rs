fn converter(celsius: f32) -> f32 {
    (celsius * 9.0 / 5.0 ) + 32.0
}

fn main() {
    println!("{}",converter(10.0));
}
