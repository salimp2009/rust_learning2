use std::fmt::Debug;
pub fn largest_num(collection: &(impl Iterator + Debug)) {
    println!("collection received: {:#?}", collection);
}
fn main() {
    largest_num(&vec![1, 2, 3, 4].iter());
    largest_num(&String::from("hello").chars());
}
