# Error Handling

## Unrecoverable Errors

- panic!
  - print a failure message
  - unwind
  - clean up the stack
  - quit

In Cargo.toml

```
[profile.release]
panic = 'abort'
```

- does not clean up the stack
- smaller binary
- OS is responsible to clean up the stack


To display a backtrace:

RUST_BACKTRACE=1

## Recoverable Errors

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## The ? Operator

```

#![allow(unused)]
fn main() {
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
    }
}
```

The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values in Listing 9-6. If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.


```

#![allow(unused)]
fn main() {
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
}
```

```

#![allow(unused)]
fn main() {
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
}
```


