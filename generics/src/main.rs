use std::fmt::Debug;
pub fn largest_num<T>(list: &[T]) -> T
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
    *largest
}
fn main() {
    let my_vec = vec![1, 2, 3, 4];
    largest_num(&[1, 2, 3, 4]);
    largest_num(&my_vec);
}
