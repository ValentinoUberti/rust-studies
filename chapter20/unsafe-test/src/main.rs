fn main() {
    let mut num = 5;
    let r1 = &num as *const i32; // Raw pointer
    let r2 = &mut num as *mut i32; // Raw pointer

    unsafe {
        println!("{}", *r1);
        *r2 +=1;
        println!("{}", *r2);
        println!("{}", *r1);
        
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];
    let r3 = &v[..];
    
}
