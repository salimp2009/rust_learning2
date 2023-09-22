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
fn main() {
    iflet_pattern();
}
