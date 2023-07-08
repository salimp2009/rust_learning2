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

#[derive(Debug)]
pub struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let my_vec = vec![1, 2, 3, 4];
    largest_any::<i32>(&[1, 2, 3, 4]);
    largest_any(&my_vec);
    let char_vec = vec!["a", "b", "c"];
    largest_any(&char_vec);
    let point1 = Point { x: 5, y: 15.6 };
    println!("point1: {point1:?},\nx: {}, y:{}", point1.x, point1.y);
}
