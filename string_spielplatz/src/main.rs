use std::collections::LinkedList;

fn main() {
    let some_content = "#Headline\n## Headline 2\nHier ist etwas text";
    
    println!("{:?}", &some_content[10..].starts_with("## "));

    let mut some_string = String::new();

    for i in 0..10 {
        some_string.push_str(&i.to_string());
    }

    println!("{:?}", some_string);

    some_string = String::new();

    for i in 0..10 {
        some_string.push_str(&(10 - i).to_string());
    }    

    println!("{:?}", some_string);

    some_string.push_str("Hallo");

    let c = some_string.pop();
    println!("{:?}, {:?}", c, some_string);

    let mut some_vec = LinkedList::new();
    for i in 0..10 {
        some_vec.push_back(i);
    }

    while let Some(i) = some_vec.pop_back() {
        println!("{:?}", i);
    }
}
