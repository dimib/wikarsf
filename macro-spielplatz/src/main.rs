mod mymacros;
use demo_derive::DemoDerive;
use demo_trait::DemoTrait;
use mymacros::foo;
use mymacros::bar;

#[derive(DemoDerive)]
struct Foo { }

#[derive(DemoDerive)]
struct Bar { }


#[allow(special_module_name)]

fn main() {
    println!("Hello, world!");

    foo();
    bar();
    print_result!(0xabcd + 0xacab);
    if compare_expressions!(1, 1) {
        println!("1 == 1");
    }
    if compare_expressions!(1, 2) {
        println!("1 == 2");
    }
    if compare_expressions!(1 * 1 + 1, 2 * 1) {
        println!("1 * 1 + 1 == 2 * 1");
    }

    calculate! {
        eval 10 / 2
    }

    let v = vecxx![1, 2, 3];
    v.iter().for_each(|x| println!("{}", x));

    let foo = Foo {};
    foo.demo();

    let bar = Bar {};
    bar.demo();
}
