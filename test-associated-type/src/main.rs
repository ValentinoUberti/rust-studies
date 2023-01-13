
struct test_trait_string;

trait TestTrait {
    type A;
    type B;

    fn write_all(&self,a: Self::A, b: Self::B) -> String;
    
}

impl TestTrait for test_trait_string {
    type A = String;

    type B = String;

    fn write_all(&self,a: Self::A, b: Self::B) -> String {
        format!("{} {}",a,b)
    }
}




fn main() {
    let s = test_trait_string {};
    println!("{}",s.write_all(String::from("Hello"),String::from("World")));
}
