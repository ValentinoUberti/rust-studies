#[derive(Debug)]
enum MyEnum {
    MyInt(u32),
    MyString(String),
    MyFloat(f64),
}

fn find_until(v: &Vec<i32>, n: i32, til: usize) -> Option<usize> {
    for i in 0 .. til {
      println!("{} {} ",i,v[i]);
      if v[i] == n {
        return Some(i);
      }
    }
    return None;
  }
fn main() {
    let mut v = vec![100, 32, 57];
    for n_ref in &mut v {
        // n_ref has type &mut i32
        *n_ref += 50;
    }
    println!("{:?}",v);

    let row = vec![MyEnum::MyInt(10),MyEnum::MyString(String::from("Ciao")),MyEnum::MyFloat(1.56)];


    println!("{:#?}",row);

    find_until(&vec![1, 2, 3], 1, 4);

    let mut v = Vec::new();
  let s = String::from("Hello ");
  v.push(s.clone());
  v[0].push_str("world");
  println!("original: {}", s);
  println!("new: {}", v[0]);
}
