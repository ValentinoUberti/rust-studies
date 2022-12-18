# Ownership Rules

- Each value has an owner
- There can only be an owner at a time
- When the owner goes out of scope, the value will be dropped

# String

## String literal

&str is a slice (&[u8]) that always points to a valid UTF-8 sequence, and can be used to view into a String, just like &[T] is a view into Vec<T>.

let a: &str = "Test"; 

## On Heap
let s1 = String::from("hello");
let s2 = s1;


 println!("{}, world!", s1); <-- Compile time error, s1 is not longer valid

Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

- double free error

If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move. 

let s1 = String::from("hello");

let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);

# know size

let x = 5;

let y = x;

println!("x = {}, y = {}", x, y);

This works due to the fact that x has a know size (u32 by default); knows type are stored on the stack which is faster than heap.

Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are

Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait. 

# Reference

Rust has a feature for using a value without transferring ownership, called references

 A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

 ```
 fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

 These ampersands represent references, and they allow you to refer to some value without taking ownership of it.

```
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
```


Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.


Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. 

```
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
```

The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion. It’s something that new Rustaceans struggle with because most languages let you mutate whenever you’d like. The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

Two or more pointers access the same data at the same time.
At least one of the pointers is being used to write to the data.
There’s no mechanism being used to synchronize access to the data.


Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile because the last usage of the immutable references, the println!, occurs before the mutable reference is introduced:

```
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
```

# Slice

