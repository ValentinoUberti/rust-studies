A trait defines functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.

Traits are similar to a feature often called interfaces in other languages, although with some differences.


"This restriction is part of a property called coherence, and more specifically the orphan rule,"

lifetime elision rules


The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build