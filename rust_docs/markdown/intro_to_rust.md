# Introduction to Rust

Rust is a systems programming language focused on **performance, memory safety, and concurrency**.

It is commonly used for:

- command-line tools
- web servers
- embedded systems
- game engines
- operating systems
- blockchain infrastructure
- performance-critical backend services
- WebAssembly apps

Rust is often compared to C and C++ because it gives you low-level control, but it prevents many common bugs at compile time.

---

## 1. Why Rust Exists

In languages like C and C++, programmers manually manage memory. This can lead to bugs such as:

- use-after-free
- null pointer dereferences
- double frees
- buffer overflows
- data races

Rust aims to give you the same level of performance as C/C++, while making many of those bugs impossible or much harder to write.

Rust does this using its most famous feature:

> **Ownership**

---

## 2. Hello World

A basic Rust program looks like this:

```rust
fn main() {
    println!("Hello, world!");
}
```

Explanation:

```rust
fn main()
```

defines the program entry point.

```rust
println!
```

prints text to the terminal. The `!` means it is a **macro**, not a normal function.

You can run a Rust program using Cargo, Rust’s build tool:

```bash
cargo new hello_rust
cd hello_rust
cargo run
```

---

## 3. Variables

Rust variables are immutable by default.

```rust
fn main() {
    let x = 5;
    println!("{}", x);
}
```

You cannot do this:

```rust
let x = 5;
x = 6; // error
```

If you want a variable to be mutable, use `mut`:

```rust
fn main() {
    let mut x = 5;
    x = 6;
    println!("{}", x);
}
```

This design encourages safer code by making mutation explicit.

---

## 4. Basic Types

Rust is statically typed, meaning variable types are known at compile time.

Common types include:

```rust
let age: i32 = 30;        // signed integer
let count: u32 = 10;      // unsigned integer
let price: f64 = 19.99;   // floating-point number
let active: bool = true;  // boolean
let letter: char = 'A';   // Unicode character
```

Rust can often infer types:

```rust
let age = 30;
let active = true;
```

---

## 5. Functions

Functions use `fn`.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(2, 3);
    println!("{}", result);
}
```

Notice there is no semicolon after `a + b`.

In Rust, the last expression in a function can be returned automatically.

This:

```rust
a + b
```

returns a value.

But this:

```rust
a + b;
```

does not return a value because the semicolon turns it into a statement.

You can also use `return`, but it is less common:

```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```

---

## 6. Control Flow

### `if`

```rust
fn main() {
    let number = 7;

    if number > 5 {
        println!("large");
    } else {
        println!("small");
    }
}
```

`if` can also produce a value:

```rust
let status = if number > 5 {
    "large"
} else {
    "small"
};
```

Both branches must return the same type.

### Loops

Rust has several loop types.

```rust
loop {
    println!("forever");
}
```

```rust
while number > 0 {
    number -= 1;
}
```

```rust
for i in 0..5 {
    println!("{}", i);
}
```

`0..5` means 0 through 4.

`0..=5` means 0 through 5.

---

## 7. Ownership

Ownership is the core concept that makes Rust different.

Rust has three basic ownership rules:

1. Each value has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped.

Example:

```rust
fn main() {
    let name = String::from("Rust");
    println!("{}", name);
}
```

The variable `name` owns the string.

When `main` ends, the string is automatically freed.

---

## 8. Moving Values

When you assign some values to another variable, ownership may move.

```rust
fn main() {
    let a = String::from("hello");
    let b = a;

    println!("{}", b);
}
```

After `let b = a;`, `a` is no longer valid.

This will fail:

```rust
fn main() {
    let a = String::from("hello");
    let b = a;

    println!("{}", a); // error
}
```

Why?

Because `String` stores data on the heap. Rust avoids double-free errors by allowing only one owner.

If you want a deep copy, use `clone`:

```rust
let a = String::from("hello");
let b = a.clone();

println!("{}", a);
println!("{}", b);
```

---

## 9. Borrowing

Instead of moving ownership, you can borrow a value using a reference.

```rust
fn print_name(name: &String) {
    println!("{}", name);
}

fn main() {
    let name = String::from("Ferris");

    print_name(&name);

    println!("{}", name);
}
```

`&name` means “borrow `name`.”

The function can read the value without taking ownership.

Usually, you would write this as:

```rust
fn print_name(name: &str) {
    println!("{}", name);
}
```

`&str` is a string slice and is more flexible than `&String`.

---

## 10. Mutable Borrowing

You can borrow a value mutably:

```rust
fn add_exclamation(text: &mut String) {
    text.push('!');
}

fn main() {
    let mut message = String::from("hello");

    add_exclamation(&mut message);

    println!("{}", message);
}
```

Rust has strict borrowing rules:

- You can have many immutable references, or
- One mutable reference,
- But not both at the same time.

This prevents data races and many bugs.

---

## 11. Structs

Structs let you create custom data types.

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let user = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
    };

    println!("{}", user.username);
}
```

You can add methods with `impl`:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    println!("{}", rect.area());
}
```

`&self` means the method borrows the struct.

---

## 12. Enums

Enums define a type that can be one of several variants.

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
```

Enums can also hold data:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
```

Enums are very powerful in Rust and are often used with `match`.

---

## 13. Pattern Matching

`match` lets you handle different cases safely.

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let direction = Direction::Up;

    match direction {
        Direction::Up => println!("up"),
        Direction::Down => println!("down"),
        Direction::Left => println!("left"),
        Direction::Right => println!("right"),
    }
}
```

Rust makes sure your `match` handles every possible case.

---

## 14. `Option`

Rust does not use `null` in the same way many languages do.

Instead, it uses `Option`.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Example:

```rust
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}
```

Using it:

```rust
fn main() {
    let user = find_user(1);

    match user {
        Some(name) => println!("Found {}", name),
        None => println!("No user found"),
    }
}
```

This forces you to handle missing values explicitly.

---

## 15. `Result`

Rust uses `Result` for recoverable errors.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Example:

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}
```

Using it:

```rust
fn main() {
    match divide(10, 2) {
        Ok(value) => println!("{}", value),
        Err(error) => println!("Error: {}", error),
    }
}
```

Rust also has the `?` operator for propagating errors:

```rust
fn read_username() -> Result<String, std::io::Error> {
    let text = std::fs::read_to_string("username.txt")?;
    Ok(text)
}
```

---

## 16. Cargo

Cargo is Rust’s package manager and build tool.

Common commands:

```bash
cargo new my_project
cargo build
cargo run
cargo test
cargo check
cargo add serde
```

Typical project structure:

```text
my_project/
├── Cargo.toml
└── src/
    └── main.rs
```

`Cargo.toml` stores project metadata and dependencies.

Example:

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
```

---

## 17. Traits

Traits are similar to interfaces.

```rust
trait Speak {
    fn speak(&self);
}

struct Dog;

impl Speak for Dog {
    fn speak(&self) {
        println!("woof");
    }
}
```

Using a trait:

```rust
fn make_it_speak<T: Speak>(animal: T) {
    animal.speak();
}
```

Traits are central to Rust’s type system.

---

## 18. Generics

Generics let you write code that works with many types.

```rust
fn identity<T>(value: T) -> T {
    value
}
```

Example with structs:

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

---

## 19. Lifetimes

Lifetimes describe how long references are valid.

Most of the time, Rust infers them.

Sometimes you write them explicitly:

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
```

This says the returned reference will be valid as long as both input references are valid.

Lifetimes do not change how long values live. They help the compiler verify references are safe.

---

## 20. Concurrency

Rust is designed for safe concurrency.

Example with threads:

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("hello from another thread");
    });

    handle.join().unwrap();
}
```

Rust’s ownership and borrowing rules help prevent data races at compile time.

---

## 21. What Makes Rust Hard?

Rust can feel difficult at first because you must understand:

- ownership
- borrowing
- lifetimes
- strict type checking
- compiler errors

But Rust’s compiler is very helpful. Many Rust developers say:

> If it compiles, it often works.

That is not always literally true, but Rust catches a lot of mistakes before your program runs.

---

## 22. A Small Complete Example

```rust
struct Todo {
    title: String,
    done: bool,
}

impl Todo {
    fn new(title: &str) -> Todo {
        Todo {
            title: title.to_string(),
            done: false,
        }
    }

    fn complete(&mut self) {
        self.done = true;
    }

    fn display(&self) {
        let status = if self.done { "done" } else { "not done" };
        println!("{}: {}", self.title, status);
    }
}

fn main() {
    let mut task = Todo::new("Learn Rust");

    task.display();

    task.complete();

    task.display();
}
```

This example shows:

- structs
- methods
- mutable borrowing
- strings
- conditionals

---

## Suggested Learning Path

1. Install Rust with `rustup`
2. Learn Cargo basics
3. Practice variables, functions, structs, and enums
4. Study ownership and borrowing carefully
5. Learn `Option`, `Result`, and `match`
6. Build a command-line tool
7. Learn traits and generics
8. Explore async Rust, web servers, or embedded Rust depending on your goals

The official Rust book is excellent:

<https://doc.rust-lang.org/book/>

If you want a simple first project, build a command-line todo list or number guessing game.
