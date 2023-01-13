// Rust code
/*
#![allow(unused)]
fn main() {
    trait Number {
        fn from_i32(num: i32) -> Self;
    }

    impl Number for f64 {
        fn from_i32(num: i32) -> f64 {
            num as f64
        }
    }

    let var: f64 = Number::from_i32(42);
    println!("{var}");

    let var: f64 = <_ as Number>::from_i32(42);
    println!("{var}");
}
*/

// Rust code
struct GfgContainer(i32, i32);

trait HasItems {
	
	type X;
	type Y;

	fn items(&self, _: &Self::X, _: &Self::Y) -> bool;
	fn first_func(&self) -> i32;
	fn second_func(&self) -> i32;
}

impl HasItems for GfgContainer {

	type X = i32;
	type Y = i32;

	fn items(&self, num_one: &i32, num_two: &i32) -> bool {
		(&self.0 == num_one) && (&self.1 == num_two)
	}
	
	fn first_func(&self) -> i32 { self.0 }


	fn second_func(&self) -> i32 { self.1 }
}

fn multiply<C: HasItems>(item: &C) -> i32 {
	item.second_func() * item.first_func()
}

fn main() {
	let num_one = 50;
	let num_two = 20;

	let item = GfgContainer(num_one, num_two);

	println!("1st number: {}", item.first_func());
	println!("2nd number: {}", item.second_func());
	
	println!("Multiplied value: {}", multiply(&item));
}
