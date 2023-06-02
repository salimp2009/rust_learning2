#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        self > other
        // self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// free standing function
pub fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };
    let rect2 = Rectangle {
        width: 60,
        height: 40,
    };

    println!("rect2 can hold rect1 : {}", rect2.can_hold(&rect1));
    println!("square : {:#?}", Rectangle::square(10));

    println!("area: {}", area(&rect1));
    println!("rectangle1: {:#?}", rect1);
    dbg!(&rect1);
    println!("rect1 > rect2 :{:?}", rect1 > rect2);
    println!("rect1 area: {}", rect1.area());
}
