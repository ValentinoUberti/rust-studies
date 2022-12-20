#[derive(Debug)]
enum Family {
    Mother,
    Father,
    Child,
}

#[derive(Debug)]
enum Family2 {
    Mother(String),
    Father(String),
    Child(String),
}

#[derive(Debug)]
struct Person {
    name: String,
    info: Family,
}

fn role(role: Family) {
    match role {

    Family::Mother => println!("Hello Mother"),
    Family::Father => println!("Hello Father"),
    Family::Child => println!("Hello Child"),

        
    }
}



fn main() {
  //let (member1,member2,member3) = (Family::Father,Family::Mother,Family::Child);

  let member1 = Person {
    name: String::from("Clara"),
    info: Family::Mother,
  };
 
  let member2 = Person {
    name: String::from("Vale"),
    info: Family::Father,
  };
 
  let member3 = Person {
    name: String::from("Giorgia"),
    info: Family::Child,
  };
 

 let newmember1 = Family2::Father(String::from("Vale"));

 println!("{:#?}",newmember1);
 println!("{:#?} \n {:#?} \n {:#?}", member1,member2,member3);
 
  


}


fn get_or_default(arg: Option<String>) -> String {
    if arg.is_none() {
        return String::new();
    }
    let s = arg.unwrap();
    s.clone()
}


