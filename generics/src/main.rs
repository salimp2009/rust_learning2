use std::fmt::Debug;
pub fn largest_num<T>(collection: &[T])
where
    T: Debug,
{
    println!("collection received: {:#?}", collection);
}
fn main() {
    let my_vec = vec![1, 2, 3, 4];
    largest_num(&[1, 2, 3, 4]);
    largest_num(&my_vec);
}
