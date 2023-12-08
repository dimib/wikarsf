# What I know about Rust... so far!


### Multipurpose System Programming Language

![](rust-logo-blk.svg)

__Invented__: at Mozilla

__Year__: 2010, first stable Version in 2015

__Current Version__: 1.73.0

__Pardigms__: Concurrent, functional, generic, imperative, structured

__Replaces__: C/C++ in Windows, Android, Linux and others

__Compiler__: based on LLVM compiler framework

__Web__: https://www.rust-lang.org



# Installation (very easy)

### See Rust website for installation instructions

https://www.rust-lang.org/tools/install


    $ rustup help
    rustup 1.26.0 (5af9b9484 2023-04-05)
    The Rust toolchain installer


Will be installed in `$HOME/.rustup` (1.2GB)

### Visual Studio Code Extensions for Rust

[rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

[rust extension](https://marketplace.visualstudio.com/items?itemName=1YiB.rust-bundle)

[github copilot](https://marketplace.visualstudio.com/items?itemName=GitHub.copilot)

### Package manager __Cargo__

#### Create a new Rust project

    $ cargo init hello_world

#### Build a Rust program (crate)

    $ cargo build

#### Run a Rust program

    $ cargo run

#### The Rust community’s crate registry

It's not necessary to program everything from scatch. Explore crates at [crates.io](https://crates.io)

Crates can be added as dependencies in the program's `Cargo.toml`. 

    [dependencies]
    wry = {version = "0.23"}
    tokio = { version = "1", features = ["full"] }
    warp = {version = "0.3"}
    pulldown-cmark = { version = "0.9", default-features = false }
    notify = {version = "5.0"}
    rust-embed = {version = "6.4.0"}
    rand = "0.8"
    gray_matter = "0.2"

# Look, listen and learn

Offical documeintation on `doc.rust-lang.org`

[read the book](https://doc.rust-lang.org/book/)

[rust-by-example](https://doc.rust-lang.org/rust-by-example/)

Other sources

[Let's get rusty - YouTube](https://www.youtube.com/@letsgetrusty)

# Let's code!



    // Hello Rust!

    fn main() {
        println!("Hello, world!");
    }

__Yes! We need to finish lines with semicolons!!!! 😏__

### Data types

#### Numeric types and boolean
    let a: i8 = 2;
    let b: i32 = 1;
    let c: i64 = 1;
    let d: f32 = 1.0;
    let e: f64 = 1.0;
    let f: bool = true;

#### String, string slice, chars, Vectors

    let s1: &str = "hello";
    let s2: String = String::from("hello");
    let c1: char = 'a';
    let c2: char = '😊';
    let v: Vec<char> = Vec::new();

#### Structures
    struct Person {
        name: String,
        age: u8,
    }

Initializing a structure

    let p = Person {
        name: String::from("John"),
        age: 20,
    };

#### Enums

    enum Profession {
        ProductOwner,
        ScrumMaster,
        Developer(String),
        UxDesigner,
        UiDesgner,
    }

#### Traits

Rust does not have classes, but structures can implement _traits_. Traits are like protocols in Swift.

    // Sample Printable trait
    trait Printable {
        fn print(&self);
    }

    // Implementation of Printable for Person
    impl Printable for Person {
        fn print(&self) {
            println!("{} {}", self.name, self.age);
        }
    }

    // Using Person and print internal properties.
    fn main() {
        let p = Person {
            name: String::from("John"),
            age: 20,
        };

        p.print();
    }

### Using Macros

    // Defining a macro
    macro_rules! hello {
        () => {
            println!("Hello, World!");
        };
    }

    // Using a macro
    hello!();


### Functions

    fn foo(number: i32) -> i32 {
        let new_number = number * 42;
        new_number // <= no semicolon here
    }

### Built-in features

#### Lint
    #[allow(unused_variables)]
    #[allow(dead_code)]

# Strange Rust concepts

### Module system
### Lifetime declaration

What is dangling reference?


A link or pointer to an instruction, table element, index item, etc. that no longer contains the same content. If the reference is not a currently valid address, or if it is valid but there is no content in that location, it may cause the computer to crash if the software is not programmed carefully.

`fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str { }`

https://www.youtube.com/watch?v=juIINGuZyBc

### Optionals
