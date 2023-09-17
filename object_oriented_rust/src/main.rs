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

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec!["yes", "Maybe", "No"],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
