use std::collections::HashMap;

fn reverse(v: &mut Vec<String>) {
    let n = v.len();
    for i in 0 .. n / 2 {
        std::mem::swap(&mut v[i], &mut v[n - i - 1]);
    }
}

fn main() {
    let mut myhash = HashMap::new();

    myhash.insert(String::from("Hello"), "world");
    myhash.insert(String::from("Hello2"), "world2");
    let mykey = String::from("Hello");

    let myvalue = match myhash.get(&mykey).copied() {
        Some(v) => v,
        None => "",
    };

    println!("{}", myvalue);

    myhash.entry(String::from("Hello")).or_insert("Madignano");
    myhash.entry(String::from("Hello3")).or_insert("Madignano");

    for (k, v) in myhash.iter() {
        println!("{} = {}", k, v);
    }

    let mut h: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, c) in "hello!".chars().enumerate() {
        h.entry(c).or_insert(Vec::new()).push(i);
    }

    println!("{:#}",h.get(&'l').unwrap()[1]);
    let mut sum = 0;
    for i in h.get(&'l').unwrap() {
        println!("{}",i);
        sum += *i;
    }
    println!("{}", sum);
}
