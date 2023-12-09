#[allow(unused_variables)]
#[allow(dead_code)]

trait Printable {
    fn print(&self);
}

struct Person {
    name: String,
    age: u8,
}

impl Printable for Person {
    fn print(&self) {
        println!("{} {}", self.name, self.age);
    }
}

#[derive(Debug)]
enum Profession {
    ProductOwner,
    ScrumMaster,
    Developer(String),
    UxDesigner,
    UiDesgner,
}

impl Printable for Profession {
    fn print(&self) {
        println!("{:?}", self)
    }
}
macro_rules! hello {
    () => {
        println!("Hello, World!");
    };
}


fn main() {
    hello!();

    let a: i8 = 2;
    let b: i32 = 1;
    let c: i64 = 1;
    let d: f32 = 1.0;
    let e: f64 = 1.0;
    let f: bool = true;

    let s1: &str = "Hello";
    let s2: String = String::from("World");
    let c1: char = 'a';
    let c2: char = '😊';

    let p = Person {
        name: String::from("John"),
        age: 20,
    };

    p.print();
    println!("{}, {}", s1, s2);

    let v: Vec<char> = Vec::new();

    let mut profession = Profession::Developer("Android".to_string());
    profession.print();
    profession = Profession::Developer(String::from("iOS"));
    profession.print();
}

fn foo(number: i32) -> i32 {
    let new_number = number * 42;
    new_number
}