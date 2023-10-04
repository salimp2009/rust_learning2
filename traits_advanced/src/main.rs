use core::fmt;
use std::{
    fmt::Display,
    mem::size_of_val,
    ops::{Add, Deref, DerefMut},
};
use traits_advanced::point::Point;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

trait Pilot {
    fn fly(&self);
    fn name() -> String;
}

trait Wizard {
    fn fly(&self);
    fn name() -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("captain speaking: lets flyy!");
    }
    fn name() -> String {
        String::from("Pilot")
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("the best thing in is live life !");
    }
    fn name() -> String {
        "Wizard".to_string()
    }
}
impl Human {
    fn fly(&self) {
        println!("whats up yooww yoww !");
    }
    fn name() -> String {
        "Human".to_string()
    }
}
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("{}", "*".repeat(len + 2));
        println!("*{}*", " ".repeat(len + 4));
    }
}

struct Point4 {
    x: i32,
    y: i32,
}

impl fmt::Display for Point4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point4 {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

#[derive(Debug)]
struct VecString(Vec<String>);

impl Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}
impl Deref for VecString {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
    // add code here
}

impl DerefMut for VecString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
// type aliases
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

fn long_type_input(f: Thunk) {
    println!("long type input: {:?}", f());
}

fn long_type_return() -> Thunk {
    Box::new(|| println!(" long type return"))
}

// Never type !
// Dont use this since it is a never ending loop; just for showing only
#[allow(dead_code)]
fn bar() -> ! {
    println!("testing Never type");
    #[allow(clippy::empty_loop)]
    loop {}
}

#[derive(Debug)]
#[allow(dead_code)]
enum Status {
    Value(u32),
    Stop,
}

fn closures_function_pointers() {
    let list_of_statuses = (0u32..20).map(&Status::Value).collect::<Vec<_>>();
    println!("list_of_statuses: {:?}", list_of_statuses);
}

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|val: i32| val * 3)
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    let size = size_of_val(&(Human {}));
    println!("size of empty Struct: {:?}", size);
    Human {}.fly();
    Pilot::fly(&(Human {}));
    Wizard::fly(&(Human {}));

    println!("{}", Human::name());
    println!("{}", <Human as Pilot>::name());
    println!("{}", <Human as Wizard>::name());
    Point4 { x: 4, y: 5 }.outline_print();
    let w = VecString(vec!["yoow".to_string(), "cool".to_string()]);
    println!("VecString w= {}", w);
    w.iter().for_each(|elem| println!("elem: {}", elem));

    let x: i32 = 5;
    let y: Kilometers = 10;
    println!("x + y : {}", x + y);
    long_type_input(long_type_return());
    closures_function_pointers();
    let final_value = return_closure()(5);
    println!("final value: {}", final_value);
}
