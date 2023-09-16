use object_oriented_rust::gui::{Button, Draw, Screen};

#[derive(Debug)]
struct SelectBox<'a> {
    width: u32,
    height: u32,
    options: Vec<&'a str>,
}

impl<'a> Draw for SelectBox<'a> {
    fn draw(&self) {
        println!("drawing: {:?}", self);
    }
}

fn main() {}
