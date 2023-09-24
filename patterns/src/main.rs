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

fn main() {
    iflet_pattern();
    while_let_conditionals();
    iterators_ranges();
    let point = (2, 3);
    print_2d_coordinates(&point);
}
