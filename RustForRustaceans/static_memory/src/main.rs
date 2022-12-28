

fn main() {
    let s: &'static str ="test static string";
    let s1 = String::from("Second test");
    println!("{} {}",s,s1);

    const S2: &str = "constant";

    for i in 1..10 {

        println!("{} = {}",i,S2);
    }

    let mut x = Box::new(42);
    let mut z = &x;
    for i in 0..100 {
        println!("{}",z);
        x=Box::new(i);
        z = &x;
    }

    println!("{}",z);
    
}
