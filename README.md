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

# Learn more

[read the book](https://doc.rust-lang.org/book/)

[rust-by-example](https://doc.rust-lang.org/rust-by-example/)

[Let's get rusty - YouTube](https://www.youtube.com/@letsgetrusty)

# Let's code!

### Hello Rust

    fn main() {
        println!("Hello, world!");
    }

