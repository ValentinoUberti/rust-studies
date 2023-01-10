use std::fmt::Display;

 struct MutStr<'a, 'b> {
        s: &'a mut &'b str
    }


fn main() {
    //println!("Hello, world!");

    //Covariant
    let x: &'static str; //More usefull
    //let x: &'a str;

    //Invariant
    let x: &mut String;

    //Contravariant
    fn take_func1(s: &'static str) {} // Less useful (stricter)
    fn take_func2<'a>(s: &'a str) {} 

   

    let mut s ="hello";
    *MutStr {s: &mut s }.s = "world";
    // Same as let mut x =

    let mut s1 ="hello-1";
    let x = *MutStr {
        s: &mut s1
    }.s="world-1";

    println!("{s}");
    println!("{:?}",x);
}
