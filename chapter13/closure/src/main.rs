// https://stackoverflow.com/questions/31012923/what-is-the-difference-between-copy-and-clone
use std::thread;
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor{
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {

        let (mut num_red,mut num_blue) =(0,0);
        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue +=1,
                ShirtColor::Red => num_red +=1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }

}




fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!("The user with preference {:?} gets {:?}",user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );


    // Only borrows
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    //let only_borrows = || println!("From closure: {:?}", list);
    let mut borrow_mutably = || list.push(7);

    //println!("Before calling closure: {:?}", list);
    borrow_mutably();
    println!("After calling closure: {:?}", list);


    // Move ownership
    thread::spawn(move || println!("From thread: {:?}",list)).join().unwrap();

    // dosen't work: list was moved in thread
    //println!("After thread closure: {:?}", list);
}
