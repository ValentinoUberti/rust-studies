
fn change_me(my_string: &mut String) {

my_string.push_str(" hey hey hey");

}

fn main() {
    let a: &str =  "Ciao";
    println!("{}",a.chars().nth(0).unwrap());

    let mut b=String::from("Hello");

    change_me(&mut b);
    println!("{}",b);



}
