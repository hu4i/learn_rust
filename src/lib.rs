//! ## General tutorials
//!
//! General concepts, practice and exercies.
//! 
//! ### The Rust Programming Language
//! 
//! [The Rust Programming Language](https://doc.rust-lang.org/nightly/book/title-page.html) is a community-contributed, 
//! easy to understand book that gives a gerenal grasp of the rust language.
//! 
//! #### Keywords
//! 
//! All basic concepts and usage of the language.
//! 
//! ### Learn Rust With Entirely Too Many Linked Lists
//! 
//! [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/) is a tutorial book 
//! organized around on refining some data structures (mainly lists) with a non-authentic style. The main idea is that when trying to 
//! construct and refine your code and hearing compiler's screaming, you will (probably) find that it is the smart and knowledgable rust comiler that is teaching you.
//! 
//! #### Keywords
//! 
//! * The following pointer types: `&`, `&mut`, `Box`, `Rc`, `Arc`, `*const`, `*mut`, `NonNull`(?)
//! * Ownership, borrowing, inherited mutability, interior mutability, Copy
//! * All The Keywords: struct, enum, fn, pub, impl, use, ...
//! * Pattern matching, generics, destructors
//! * Testing, installing new toolchains, using `miri`
//! * Unsafe Rust: raw pointers, aliasing, stacked borrows, UnsafeCell, variance
//! 
//! #### Chapters
//! 
//! See [`entirely_too_many_lists`].

pub mod entirely_too_many_lists;
pub mod asynchronous_programming_in_rust; 
pub mod my_redis;
pub mod tokio_tutorial;