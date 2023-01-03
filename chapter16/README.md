# Box -> Ok

# RC -> When a value is owned by multiple owners (like in Graph)

# RefCell<T> -> mutate data even when there are immutable refereces to that data
 
- normally, this action is disallowed by the borrowing rules
- interior mutability pattern
- implemented using unsafe code
- The compiler can not check the borrow and ownership rules at compile time
- Single Thread