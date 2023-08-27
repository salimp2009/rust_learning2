use smart_pointers::Boxm;

use crate::List::{Cons, Nil};

#[derive(Debug, Clone)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn reference_pointers() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    println!("x: {x}");
    println!("x via y: {}", y);
    println!("x via z: {}", z);
    assert_eq!(x, *y);
    assert_eq!(x, *z);
}

pub fn mybox_type() {
    let x = 5;
    let y = Boxm::new(x);
    assert_eq!(x, *y);
    println!("y: {:?}", *y);
}

pub fn print_name(name: &str) {
    println!("My name is {}", name);
}

pub fn use_print_name() {
    let m = Boxm::new(String::from("Salitos"));
    // automatically to deref's from &Box<String> to &String
    // and &String coerces to &str
    print_name(&m);
    // print_name(&*m);
    // those are not needed; just to understand how it works
    print_name(&m[..3]);
    print_name(&(*m)[..3]);
    print_name("didokis");
}

pub fn drop_early() {
    let data = Boxm::new(5);
    println!("data is created");
    std::mem::drop(data);
    println!("data is dropped");
}

pub fn list_sharedptr() {
    // example for the need of reference counting pointer
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list: {:#?}", list);
    reference_pointers();
    mybox_type();
    use_print_name();
    drop_early();
}
