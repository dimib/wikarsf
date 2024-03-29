# What I know about Rust... so far!

### What is Rust?

Rust is a _Multipurpose System Programming Language_ with target on System Programming.
It can be seen as a replacement for the programming language "C" and maybe "C++". The
biggest advantange of Rust towards "C" is its robust and secure memory management.
Rust comes with a number of strange concepts, that increases the learning curve.

![](rust-logo-gray.png)

__Invented__: at Mozilla

__Year__: 2010, first stable Version in 2015

__Current Version__: 1.73.0

__Pardigms__: Concurrent, functional, generic, imperative, structured

__Replaces__: C/C++ in Windows, Android, Linux and others

__Compiler__: based on LLVM compiler framework

__Web__: https://www.rust-lang.org

### Why Rust?

I was searching for a programming language to create my own programming language. I was
thinking about using "C++", but the language I want to create has unicode expressions,
so I was looking for something that can handle unicode strings by default.

But it should not be _Swift_.

I heard about Rust some years ago on a CocoaHeads talk about [Hyperdeck](https://hyperdeck.io) which uses Rust for parsing and some other stuff. So I wanted to give it a try.

During my investigations I found out some more about Rust:

Rust has become the no 1 choice of Microsoft for rebuilding Windows kernel code.

Rust is the second language after "C" that is allowed in Linux kernel code.

Figma replaces the code for the collaboration platform with Rust and notices a big performance increase by using less resources.

Dropbox rewrites a lot of code for file sharing with Rust.

And more...

# What makes Rust better than other programming languages

### Rust is safe

Rust is a very safe / secure language. Programms written in Rust don't have

Dangeling References

*-* Data races

*-* Buffer overflows

*-* Iterator invalidation

*-* Optimized for speed

### But why should I not use Rust

Safety and security has its price. Rust is not easy to learn and it is nearly impossible to quickly write programm / app / web service.

The compiler is quite communicative and gives a lot of information what you made wrong. This could be overwhealming in the beginning and sometimes the errors are not very clear. We will see this later, I'd guess :-) 

# Installation (very easy)

### See Rust website for installation instructions

https://www.rust-lang.org/tools/install

```
$ rustup help
rustup 1.26.0 (5af9b9484 2023-04-05)
The Rust toolchain installer
```

Will be installed in `$HOME/.rustup` (1.2GB)

### Visual Studio Code Extensions for Rust

Visual Studio Code is a very good IDE for Rust development. There are good plugins available that help programming. Also, the workspace concept works good for larger projects.

[rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

[rust extension](https://marketplace.visualstudio.com/items?itemName=1YiB.rust-bundle)

[github copilot](https://marketplace.visualstudio.com/items?itemName=GitHub.copilot)

### Package manager __Cargo__

#### Create a new Rust project

```
$ cargo init hello_world
```

#### Build a Rust program (crate)

```
$ cargo build
```
#### Run a Rust program

```
$ cargo run
```


#### The Rust community's crate registry

It's not necessary to program everything from scatch. Explore crates at [crates.io](https://crates.io)

Crates can be added as dependencies in the program's `Cargo.toml`. 

```
[dependencies]
wry = {version = "0.23"}
tokio = { version = "1", features = ["full"] }
warp = {version = "0.3"}
pulldown-cmark = { version = "0.9", default-features = false }
notify = {version = "5.0"}
rust-embed = {version = "6.4.0"}
rand = "0.8"
gray_matter = "0.2"
```

Complared with the Swift development environment for MacOS and iOS, which brings most of the features and frameworks for the daily use, Rust has a very small standard library. Everything else must be installed separatelly.


# Look, listen and learn

Offical documeintation on `doc.rust-lang.org`

[read the book](https://doc.rust-lang.org/book/)

This is also installed with rustup. Read the offline book with `rustup docs --book`.

[rust-by-example](https://doc.rust-lang.org/rust-by-example/)

Other sources

[Let's get rusty - YouTube](https://www.youtube.com/@letsgetrusty)

# Let's code!


```
// Hello Rust!

fn main() {
    println!("Hello, world!");
}
```

_Yes! We need to finish lines with semicolons!!!!_

### Data types

#### Numeric types and boolean
```
let a: i8 = 2;
let b: i32 = 1;
let c: i64 = 1;
let d: f32 = 1.0;
let e: f64 = 1.0;
let f: bool = true;
```

#### String, string slice, chars, Vectors

```
let s1: &str = "hello";
let s2: String = String::from("hello");
let c1: char = 'a';
let v: Vec<char> = Vec::new();
```

`&str` is called a string slice and is allocated on stack memory.

`Strings` are allocated on heap memory.


#### Enums

```
enum Profession {
    ProductOwner,
    ScrumMaster,
    Developer(String),
    UxDesigner,
    UiDesgner,
}
```

#### Structures

```
struct Person {
    name: String,
    email: String,
}
```

Initializing a structure

```
let p = Person {
    name: String::from("John Doe"),
    email: String::from("john.doe@foo.com"),
};
```

Structures can have functions, that operate with the attributes of a structure. Functions can be added with `Implementations`.

```
// Implementation of Person
impl Person {
    fn send_email(&self) {
        send_hello_mail(self.email, format!("Hello, {}", self.email));
    }
}

let person = Person(...);
person.send_hello_mail();

```
### Functions

```
fn foo(number: i32) -> i32 {
    let new_number = number * 42;
    new_number // <= no semicolon here
}

// Calling foo

let bar = foo(42);
```


# Traits

Rust does not have classes, but structures can implement _traits_. Traits are like protocols in Swift.

```
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
```

# Macros

Rust makes intensive use of macros. 

A good introduction to Rust macros is [The Little Book Of Rust Macros](https://veykril.github.io/tlborm/)

There are 3 different types of macros:

*-* Declaretive macros

*-* Procedural macros

*-* Derive macros

### Declerative macros

```
// Defining a simple macro
macro_rules! hello {
    () => {
        println!("Hello, World!");
    };
}

// Using a the macro

hello!();
```

Macros can take parameters that will be used inside the code. As far as I could see,
the ownership / borrowing mechanism does not apply on macros.


```
// Defining a marcro with expressions
macro_rules! hello_user {
    ($user:expr) => {
        println!("Hello, {}", $user);
    };
}

// Using the macro

hello!("Paul");
```

The expression list can contain more than one expression or even an unlimited number of expressions. The `vec!()` macro is a good example for macros with unlimited expressions.

Declerative macros must be defined or declared in the source file before they can be used.



# Strange Rust concepts

## Module system

Some help with modules can be found here
[Cargo Modules](https://crates.io/crates/cargo-modules)

This is a cargo extension that prints a module tree.

`cargo install cargo-modules`

`cargo modules generate tree`

## Ownership and borrowing

Rust implements a system of ownership and borrowing mechanism that prevents using variables at differnt places. Example:

```
fn ownership_explained() {
    let mut s1 = String::from("Hello");
    let mut s2 = s1; // s1 is moved to s2

    s1.push_str(" world");
    println!("{:?}", s1);
    println!("{:?}", s2);
}
```

## Lifetime declaration

What is a dangling reference?

A link or pointer to an instruction, table element, index item, etc. that no longer contains the same content. If the reference is not a currently valid address, or if it is valid but there is no content in that location, it may cause the computer to crash if the software is not programmed carefully.

`fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str { }`

https://www.youtube.com/watch?v=juIINGuZyBc

## Optionals

Rust has Optionals but they are not integrated into the language as seamlessly as in Swift.

```
fn main() {

    let my_optional = optional_value(true);
    if let Some(value) = my_optional {
        println!("{}", value);
    }

    match my_optional {
        Some(value) => println!("{}", value),
        None => println!("No value"),
    }

    let my_other_optional = optional_value(true).expect("No value");
    println!("{}", my_other_optional);

}

fn optional_value(yes: bool) -> Option<&'static str> {
    if yes {
        Some("Yes")
    } else {
        None
    }
}

```


# Multiplatform

Rust can compile binaries for may different targets. The supported targets can be listed with `rustup target list`.

```
user@mac ~ % rustup target list
aarch64-apple-darwin (installed)
aarch64-apple-ios (installed)
aarch64-apple-ios-sim
aarch64-linux-android
...

user@mac ~ % rustup target add aarch64-apple-ios-sim

```

That make Rust a good opportunity for cross- or multiplatform development.

There is a good tutorial on [github](https://github.com/thombles/dw2019rust)

### How does it work

Create a new rust library, for example

`cargo init rs-xcode --lib`

Add the following lines to the `Cargo.toml` file:

```
[lib]
crate-type = ["lib", "staticlib"]
```

Now create static lib

```
cargo rustc -- --print native-static-libs

note: Link against the following native artifacts when linking against this static library. The order and any duplication can be significant on some platforms.

note: native-static-libs: -lSystem -lc -lm

```

Now lets test it with a simple "C" program `hello-rust.c`:

```
// Decleare reference to hello_from_rust()

extern void hello_from_rust();

int main(int argc, char **argv) {
    hello_from_rust();
    return 0;
}
```

Build with

`cc hello-rust.c ../rs-xcode/target/debug/librs_xcode.a -lSystem -lresolv -lc -lm -o hello-rust`

### How does it work with iOS

First we need to build our static lib for iOS simulator and / or iPhone.

`cargo build --target aarch64-apple-ios-sim`

Now we find our library here `target/aarch64-apple-ios-sim/debug/librs_xcode.a`.

This library can be added to our Xcode project.

Important to know is that the library has a size of about 17MB.