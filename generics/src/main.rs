use std::fmt::Debug;
pub fn largest_any<T>(list: &[T]) -> &T
where
    T: Copy + Debug + PartialOrd + PartialEq,
{
    let mut largest = &list[0];
    list.iter().for_each(|value| {
        if value > largest {
            largest = value;
        }
    });
    println!("collection received: {:#?}", largest);
    largest
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    pub fn new(x: T, y: U) -> Self {
        Point { x, y }
    }

    pub fn get_x(&self) -> &T {
        &self.x
    }

    pub fn get_y(&self) -> &U {
        &self.y
    }
}

impl Point<f32, f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let my_vec = vec![1, 2, 3, 4];
    largest_any::<i32>(&[1, 2, 3, 4]);
    largest_any(&my_vec);
    let char_vec = vec!["a", "b", "c"];
    largest_any(&char_vec);
    let point1 = Point { x: 5, y: 15.6 };
    println!("point1: {point1:?},\nx: {}, y:{}", point1.x, point1.y);
    println!("point.x :{}", point1.get_x());
}
