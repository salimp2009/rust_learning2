pub trait Draw {
    fn draw(&self) {
        println!("drawing Draw trait!");
    }
}

// #[derive(Debug)]
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        self.components.iter().for_each(|elem| elem.draw());
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("drawing Button:{:?}", self)
    }
}
