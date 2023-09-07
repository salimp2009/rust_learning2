use smart_pointers::Boxm;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::List::{Cons, Nil};
use crate::List2::{Cons2, Nil2};
use crate::List3::{Cons3, Nil3};
use crate::List4::{Cons4, Nil4};

#[derive(Debug, Clone)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
#[derive(Debug, Clone)]
pub enum List2 {
    Cons2(i32, Rc<List2>),
    Nil2,
}

#[derive(Debug, Clone)]
pub enum List3 {
    Cons3(Rc<RefCell<i32>>, Rc<List3>),
    Nil3,
}

#[derive(Debug, Clone)]
pub enum List4 {
    Cons4(i32, RefCell<Rc<List4>>),
    Nil4,
}

impl List4 {
    fn tail(&self) -> Option<&RefCell<Rc<List4>>> {
        match self {
            Cons4(_, item) => Some(item),
            Nil4 => None,
        }
    }
}

pub fn memory_leak() {
    let a = Rc::new(Cons4(5, RefCell::new(Rc::new(Nil4))));
    println!("a initial rc count {}", Rc::strong_count(&a));
    println!("a next item(tail)  {:?}", &a.tail());
    println!("the list a:  {:?}", a);

    let b = Rc::new(Cons4(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b created: {}", Rc::strong_count(&a));
    println!("b initial rc count {}", Rc::strong_count(&b));
    println!("b next item(tail)  {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count  after changing a {}", Rc::strong_count(&b));
    println!("a rc count  after changing a {}", Rc::strong_count(&a));
    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
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
    println!("m (Boxm) : {:?}", *m);
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
    let a1 = Box::new(Cons(5, Box::new(Cons(10, Box::new(Nil)))));
    let _b1 = Cons(3, Box::clone(&a1));
    let _c1 = Cons(4, Box::clone(&a1));

    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    let b = Cons2(3, Rc::clone(&a));
    let c = Cons2(4, Rc::clone(&a));
    println!("a: {:?}", a);
    println!("count of shared ref of a: {:?}", Rc::strong_count(&a));
    println!("b: {:?}", b);
    println!("c: {:?}", c);
}

pub fn refcount_refcel() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons3(Rc::clone(&value), Rc::new(Nil3)));

    let b = Cons3(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons3(Rc::new(RefCell::new(4)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("value: {:?}", *value.borrow());
    println!("a : {:?}", *a);
    println!("b : {:?}", b);
    println!("c : {:?}", c);
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn weak_pointers() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf strong :{:?}", Rc::strong_count(&leaf));
    println!("leaf weak :{:?}", Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "branch strong :{:?}, branch weak: {:?}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        println!("leaf strong :{:?}", Rc::strong_count(&leaf));
        println!("leaf weak :{:?}", Rc::weak_count(&leaf));

        println!("leaf count: {}", Rc::strong_count(&leaf));
        println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
    }

    println!("after inner leaf strong :{:?}", Rc::strong_count(&leaf));
    println!("after inner leaf weak :{:?}", Rc::weak_count(&leaf));
    println!(
        "after inner leaf parent: {:?}",
        leaf.parent.borrow().upgrade()
    );
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list: {:#?}", list);
    reference_pointers();
    mybox_type();
    use_print_name();
    drop_early();
    list_sharedptr();
    refcount_refcel();
    memory_leak();
    weak_pointers();
}
