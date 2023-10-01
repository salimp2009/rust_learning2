use core::fmt;
use std::{mem::size_of_val, ops::Add};
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
}
