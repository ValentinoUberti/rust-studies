use std::thread;
use std::time::Duration;

fn main() {

    let v = Box::new(vec![1, 2, 3]);


    let t = thread::spawn(move || {
        for i in 1..10 {
            println!("Number {i} from thread 1 - {:#?}",v);
            thread::sleep(Duration::from_millis(1));
        }

    }).join().unwrap();


    let t1 = thread::spawn(|| {
        for i in 1..10 {
            println!("Number {i} from thread 2 ");
            thread::sleep(Duration::from_millis(1));
        }

    });

   /*
    if let Ok(_) = t.join() {
        println!("Ok");
    } else {
        println!("Error joining threads");
    }
    */

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

  
}