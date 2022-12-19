#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple struct
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);


// Unit like struct, usefull when creating a Trait without properties
struct INeedATrait;

fn build_user(email: String,username: String) -> User {

    User {
        active: true,
        username,
        email,
        sign_in_count: 0
    }

}

fn main() {
    
    let user = build_user(String::from("vuberti@redhat.com"), String::from("valeube"));

    let user2 = User {
        email: String::from("info@valentinouberti.com"),
        username: String::from("valeube2"),
       ..user
    };

    println!("{:#?} \n {:#?} ",user,user2);

    let black=Color(0,0,0);
    let origin= Point(0,0,0);

    println!("{:#?} \n {:#?} ",black,origin);
}
