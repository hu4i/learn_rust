# Qusestions

- [Qusestions](#qusestions)
  - [Ownership](#ownership)
    - [[x] Ownership in matching](#x-ownership-in-matching)
    - [Difference between `as_ref` and `as_dref`](#difference-between-as_ref-and-as_dref)
    - [[x] What does `Option::map` do here?](#x-what-does-optionmap-do-here)
    - [[x] What will happen if we use `Rc` rather than `&` in `iter`?](#x-what-will-happen-if-we-use-rc-rather-than--in-iter)
  - [Pattern mathing](#pattern-mathing)
    - [[x] Pattern matching in function parameters](#x-pattern-matching-in-function-parameters)
  - [Lifetimes](#lifetimes)
  - [Scope](#scope)
    - [[x] Why method `next` in impl for `Iterator` is not public?](#x-why-method-next-in-impl-for-iterator-is-not-public)
  - [Type](#type)
    - [[x] Is `'static` a type?](#x-is-static-a-type)
    - [[x] Difference between `&str` and `&Stirng`](#x-difference-between-str-and-stirng)
    - [Difference between `T: 'static` and `&'static T`, and what do they mean?](#difference-between-t-static-and-static-t-and-what-do-they-mean)
  - [Asynchronous Programming](#asynchronous-programming)
    - [[x] How does `poll` and `await` work?](#x-how-does-poll-and-await-work)
    - [[x] Why use the Mutex in `futures::lock` rather than the one from `std::sync`](#x-why-use-the-mutex-in-futureslock-rather-than-the-one-from-stdsync)
  - [Misc](#misc)
    - [Key word `const`](#key-word-const)

## Ownership

### [x] Ownership in matching

In file [first.rs](./src/entirely_too_many_lists/first.rs):

```rust

    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Nil);
        while let Link::More(boxed_node) = cur_link { // why "mut" is needed here? Doesn't boxed_node have the onwnership of cur_link's node?
            cur_link = mem::replace(&mut boxed_node.next, Link::Nil);
        }
    }

    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Nil);
        while let Link::More(boxed_node) = cur_link {
            cur_link = boxed_node.next; // Boxed_node have the onwnership of cur_link's node.
        }

    }
```

### Difference between `as_ref` and `as_dref`

See [Rust's as_ref vs as_dref](https://www.fpcomplete.com/blog/rust-asref-asderef/) and [too-many-lists: Iter](https://rust-unofficial.github.io/too-many-lists/second-iter.html#iter).

- Example

    In file [second.rs](./src/entirely_too_many_lists/second.rs), two definitions of `iter` works the same:

    ```rust
    impl<T> List<T> {
        pub fn iter1<'a>(&'a self) -> Iter<'a, T> {
            Iter { 
                next: self.head.as_deref(),
            } 
        }

        pub fn iter2<'a>(&'a self) -> Iter<'a, T> {
            Iter { 
                next: self.head.as_ref()
                        .map(|node| &**node) 
            } 
        }
    }
    ```

- Hints

    1. It seems that `String` is a pointer for `str`

        ```rust
        let str_a = &*"hello world".to_string();
        let some_boxed_string:Option<Box<String>> = Some(Box::new("hello world".to_string()));
        let reference_to_str:Option<&String> = some_boxed_string.as_deref();
        ```

### [x] What does `Option::map` do here?

In file [second.rs](./src/entirely_too_many_lists/second.rs), method `iter`:

```rust
// code piece 1
fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
        self.next = node.next.as_deref();
        &node.elem
    })
}

// code piece 2
fn next(&mut self) -> Option<Self::Item> {
    self.next.take().map(|node| {
        self.next = node.next.as_deref();
        &node.elem
    })
}
```

- That's my explain:

The two kinds of `next` both compile and work fine. But in [too-many-lists](https://rust-unofficial.github.io/too-many-lists/second-iter-mut.html#itermut), it says the first kind works because `self.next` copies the `Option<&>` to `map`, because only a "pointer with read access" is needed to be copied. But the code below can't work because `Option<&mut>` can't be copied, you can't just copy the "pointer with mutable access" otherwise there would be more than one mutable pointer pointing to the same address.

```rust
fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
        self.next = node.next.as_deref();
        &node.elem
    })
```

### [x] What will happen if we use `Rc` rather than `&` in `iter`?

In file [third.rs](./src/entirely_too_many_lists/third.rs):

```rust
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>, //if we use: `next: Option<Rc<Node<T>>>`
}
```

Will this change prevent `Node<t>` from being cleaned up?

## Pattern mathing

### [x] Pattern matching in function parameters

In file [second.rs](./src/entirely_too_many_lists/second.rs), method `peek`:

The `value` has the type `&mut i32`:

```rust
    list.peek_mut().map(|value| {
        *value = 7
    });
```

The `value` has the type `i32`:

```rust
    list.peek_mut().map(|&mut value| {
        *value = 7 // error occurs
    });
```

This is because of destructuring references with pattern matching occurs in declaration of function parameters, see [Function Parameters](https://doc.rust-lang.org/book/ch18-01-all-the-places-for-patterns.html#function-parameters) and [destructuring references](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch18-03-pattern-syntax.html#destructuring-referenceshttps://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch18-03-pattern-syntax.html#destructuring-references).

- Notes

    1. It seems that it has different meaning of the declaration of the keyword `self` in the definition of a method, see [Defining Methods](https://doc.rust-lang.org/nightly/book/ch05-03-method-syntax.html?highlight=self#defining-methods), it says:

        - `self` means `self: Self`
        - `&self` means `self: &Self`
        - `&mut self` means `self: &mut Self`

    2. Keyword `ref` also doesn't involve in pattern matching, see [Keyword ref](https://doc.rust-lang.org/std/keyword.ref.html).
    3. What about `fn foo(mut value: mut T)`?
    4. What about `|&mut value| {...}` and value doesn't implement `copy` trait?

## Lifetimes

See [too-many-lists: Iter](https://rust-unofficial.github.io/too-many-lists/second-iter.html#iter).

## Scope

### [x] Why method `next` in impl for `Iterator` is not public?

In file [third.rs](./src/entirely_too_many_lists/third.rs):

```rust
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> { // why not pub fn next(...)?
        // ToDo
    }
```

## Type

### [x] Is `'static` a type?

In file [executor.rs](./src/asynchronous_programming_in_rust/executor.rs):

```rust
struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
}
```

### [x] Difference between `&str` and `&Stirng`

In file [pining.rs](src/asynchronous_programming_in_rust/pinning.rs):

```rust
impl PinToStack {

    ...

    fn a(self: Pin<& Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<& Self>) -> &String {
        assert!(!self.b.is_null(), "no no no!");
        unsafe { &*self.b }
    }
}
```

### Difference between `T: 'static` and `&'static T`, and what do they mean?

see rust blog of [common-rust-lifetime-misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md#2-if-t-static-then-t-must-be-valid-for-the-entire-program).

- `&'static T`is an immutable reference to some `T` that can be safely held indefinitely long, including up until the end of the program.
- `T: 'static` is some T that can be safely held indefinitely long, including up until the end of the program.
  - Including `&'static T` and owned type, because they can be, but not must be.

and in [tutorial of tokio](https://tokio.rs/tokio/tutorial/spawning)

> When we say that a value is `'static`, all that means is that it would not be incorrect to keep that value around forever. This is important because the compiler is unable to reason about how long a newly spawned task stays around. We have to make sure that the task is allowed to live forever, so that Tokio can make the task run as long as it needs to.  
> The article that the info-box earlier links to uses the terminology "bounded by `'static`" rather than "its type outlives `'static`" or "the value is `'static`" to refer to `T: 'static`. These all mean the same thing, but are different from "annotated with `'static`" as in `&'static T`.

## Asynchronous Programming

### [x] How does `poll` and `await` work?

In file [executor.rs](./src/asynchronous_programming_in_rust/executor.rs):

```rust
spawner.spawn(async {
    dbgprint("Anonymous async block","started");
    println!("howdy!");
    TimerFuture::new(Duration::new(2, 0)).await;
    dbgprint("Anonymous async block","going to sleep");
    thread::sleep(Duration::new(2,0));
    dbgprint("Anonymous async block","get up!");
    println!("down!");
    dbgprint("Anonymous async block","finished");
});


impl Executor {
    pub fn run(&self) {
        ...
                dbgprint("Executor::run", "started to poll a future");

                let output = future_temp.poll(context);

                dbgprint("Executor::run", "ended in polling a future");

                if output.is_pending() {
                    dbgprint("Executor::run", "future is pending");
                    *future_slot = Some(future);
                } else {
                    dbgprint("Executor::run", "future is finished");
                }
            }
        }
    }
}
```

The output shows that at first the async block executed before `future_temp.poll(context)`, and the async block executed fater `future_temp.poll(context)`. But who call the codes in async block to go on and to "yield"?

```plain
[*] [14:28:46] [ThreadId(2)] [Executor::run] received a task
[*] [14:28:46] [ThreadId(2)] [Executor::run] task has a future
[*] [14:28:46] [ThreadId(2)] [Executor::run] started to poll a future
[*] [14:28:46] [ThreadId(2)] [Anonymous async block] started
howdy!
[*] [14:28:46] [ThreadId(2)] [TimerFuture::new] started
[*] [14:28:46] [ThreadId(2)] [TimerFuture::new] spawned a new thread
[*] [14:28:46] [ThreadId(2)] [TimerFuture::new] finished
[*] [14:28:46] [ThreadId(2)] [TimerFuture::poll] started
[*] [14:28:46] [ThreadId(3)] [New Thread] created
[*] [14:28:46] [ThreadId(2)] [TimerFuture::poll] future pending
[*] [14:28:46] [ThreadId(3)] [New Thread] is going to sleep
[*] [14:28:46] [ThreadId(2)] [Executor::run] ended in polling a future
[*] [14:28:46] [ThreadId(2)] [Executor::run] future is pending
[*] [14:28:48] [ThreadId(3)] [New Thread] get up!
[*] [14:28:48] [ThreadId(3)] [New Thread] is going to call wake
[*] [14:28:48] [ThreadId(3)] [New Thread] destoryed
[*] [14:28:48] [ThreadId(2)] [Executor::run] received a task
[*] [14:28:48] [ThreadId(2)] [Executor::run] task has a future
[*] [14:28:48] [ThreadId(2)] [Executor::run] started to poll a future
[*] [14:28:48] [ThreadId(2)] [TimerFuture::poll] started
[*] [14:28:48] [ThreadId(2)] [TimerFuture::poll] future ready
[*] [14:28:48] [ThreadId(2)] [Anonymous async block] going to sleep
[*] [14:28:50] [ThreadId(2)] [Anonymous async block] get up!
down!
[*] [14:28:50] [ThreadId(2)] [Anonymous async block] finished
[*] [14:28:50] [ThreadId(2)] [Executor::run] ended in polling a future
[*] [14:28:50] [ThreadId(2)] [Executor::run] future is finished
```

### [x] Why use the Mutex in `futures::lock` rather than the one from `std::sync`

In [Async-book: async/await](https://rust-lang.github.io/async-book/03_async_await/01_chapter.html#awaiting-on-a-multithreaded-executor), it says:

> Similarly, it isn't a good idea to hold a traditional non-futures-aware lock across an .await, as it can cause the threadpool to lock up: one task could take out a lock, .await and yield to the executor, allowing another task to attempt to take the lock and cause a deadlock. To avoid this, use the Mutex in futures::lock rather than the one from std::sync.

## Misc

### Key word `const`

In file [pining.rs](src/asynchronous_programming_in_rust/pinning.rs), I see a use of `const`:

```const
pub struct Test {
    a: String,
    b: *const String,
}
```

From Rust documentation: [Keyword const](https://doc.rust-lang.org/std/keyword.const.html), there are three main use of keyowrd `cosnt`:

- Compile-time constants
- Compile-time evaluable functions
- Use of [pointer primitive](https://doc.rust-lang.org/std/primitive.pointer.html), as seen in `*const T` and `*mut T`, these types can roughly be treated like pointers pointing to constant or non-constant variable in C:

    ```rust
    let my_num: i32 = 10;
    let my_num_ptr: *const i32 = &my_num;
    let mut my_speed: i32 = 88;
    let my_speed_ptr: *mut i32 = &mut my_speed;
    ```
