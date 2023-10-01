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
}
