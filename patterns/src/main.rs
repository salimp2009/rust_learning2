pub fn iflet_pattern() {
    let favorite_color: Option<&str> = None;
    let is_tuesday: bool = false;
    let age = "34".parse::<u8>();

    if let Some(color) = favorite_color {
        println!("Using favorite color, {} as background", color);
    } else if is_tuesday {
        println!("Tuesday is cool day!");
    }
    if let Ok(age) = age {
        if age > 30 {
            println!("Using blue as background color");
        } else {
            println!("Using orange as background color");
        }
    } else {
        println!("Using purple as background color");
    }
}

pub fn while_let_conditionals() {
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("top: {}", top);
    }
    assert_eq!(stack.len(), 0);
    println!("stack after pop {:?}", stack);
}

pub fn iterators_ranges() {
    ('a'..'d').enumerate().for_each(|(i, val)| {
        println!("index: {i}, value: {val}");
    });
}
pub fn print_2d_coordinates(&(x, y): &(i32, i32)) {
    println!("x: {}, y: {}", x, y);
}

pub fn match_with_or() {
    let x: u32 = 7;
    let y = 'b';
    match x {
        1 | 2 => println!("1 or 2"),
        5 | 6 => println!("5 or 6"),
        7..=9 => println!("7 thru 9"),
        _ => println!(" greater than 6 or smaller than 1"),
    }
    match y {
        'a' => println!("a"),
        'b'..='d' => println!("b thru d"),
        _ => println!("anything other than a thru d"),
    }
}

pub fn ignore_values_partially() {
    let mut settings = Some(5);
    let new_settings = Some(10);

    match (settings, new_settings) {
        (Some(_), Some(_)) => println!("cant overwrite settings"),
        _ => {
            settings = new_settings;
        }
    }
    println!("setting is {:?}", settings);
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn destructuring() {
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("x: {}", x),
        Point { x: 0, y } => println!("y: {}", y),
        Point { x, y } => println!("x: {}, y: {}", x, y),
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
pub fn destructuring_enums() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!("r: {r}, g: {g}, b: {b}"),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!("h:{h}, s:{s}, v:{v} "),
        _ => (),
    }
}

struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}
pub fn ignoring_multiple_values() {
    let p = Point2 { x: 0, y: 0, z: 5 };
    let Point2 { x, .. } = p;
    println!("Point2 x: {x}");
}

pub fn matchguard_with_conditionals() {
    let num = Some(4);
    let y = true;
    match num {
        Some(x) if x % 2 == 0 => println!("The number is {} even", x),
        Some(x) if y => println!("The number is {} odd", x),
        None => (),
        _ => (),
    }
}
enum Message2 {
    Hello { id: i32 },
}
pub fn match_bindings() {
    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello { id: id_var @ 3..=7 } => println!("Found id in range {}", id_var),
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message2::Hello { id } => println!("Found some other id {}", id),
    }
}
fn main() {
    iflet_pattern();
    while_let_conditionals();
    iterators_ranges();
    let point = (2, 3);
    print_2d_coordinates(&point);
    match_with_or();
    ignore_values_partially();
    destructuring();
    destructuring_enums();
    ignoring_multiple_values();
    matchguard_with_conditionals();
    match_bindings();
}
