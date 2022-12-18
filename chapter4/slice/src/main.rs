
fn first_word(s: &String) -> usize {

let first_space = match s.find(" ") {
    Some(x) => x,
    None => s.len()
    
};

first_space

}

fn main() {
    let s = String::from("Helloworld! How are you?");
    let second_index= first_word(&s);
    println!("{} -> {}",s,&s[0..second_index]);

}
