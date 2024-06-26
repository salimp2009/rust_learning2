#![feature(allocator_api)]

use memory_allocators::basicallocator::BasicAllocator;
use memory_allocators::custom_alloc::{self, PassThruAllocator};
use memory_allocators::doublylinkedlist::{self, DoublyLinkedList};
use memory_allocators::linkedlist::{self, SinglyLinkedList};
use memory_allocators::linkedlistCow::SinglyLinkedListC;
use std::mem;

fn deep_copy_examples() {
    let mut most_populous_us_cities = vec!["LA", "New York City", "Chicago"];
    let most_pop_us_cities_cloned = most_populous_us_cities.clone();
    most_populous_us_cities.push("Phoenix");
    assert_ne!(
        most_pop_us_cities_cloned.len(),
        most_populous_us_cities.len()
    );
    println!(
        "most_populous_us_cities_cloned: {:?}",
        most_pop_us_cities_cloned
    );

    println!("most_populous_us_cities: {:?}", most_populous_us_cities);
}

struct Buf<T> {
    buf: Vec<T>,
}

#[allow(dead_code)]
impl<T> Buf<T> {
    fn get_reset(&mut self) -> Vec<T> {
        mem::take(&mut self.buf)
    }
}

#[cfg(target_family = "unix")]
fn get_platform() -> String {
    "UNIX".into()
}

#[cfg(target_family = "windows")]
fn get_platform() -> String {
    "Windows".into()
}

fn main() {
    // examples for borrow checker
    let mut top_movies = vec!["Avatar", "Avengers:EndGame", "Iron Man"];
    let top_movies_mut_ref = &mut top_movies;
    top_movies_mut_ref.push("Star Wars: Rogue One");
    let top_movies_ref = &top_movies;
    println!("top movies: {:?}", top_movies_ref);
    let mut top_movies_moved = top_movies;
    top_movies_moved.push("Star Wars: Force Awakens");
    println!("top movies: {:?}", top_movies_moved);
    deep_copy_examples();
    let v: Vec<_> = "abcXXXabcYYYabc".match_indices("abc").collect();
    assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);
    println!("match indices: {:?}", v);
    let v: Vec<_> = "abcXXXabcYYY0abc".rmatch_indices("abc").collect();
    assert_eq!(v, [(13, "abc"), (6, "abc"), (0, "abc")]);
    println!("rmatch indices: {:?}", v);
    let mut list = SinglyLinkedList::new("head");
    list.append("middle");
    list.append("tail");
    println!("list: {:?}", list);
    println!("list: {:?}", list.head());
    let mut item = list.head();
    loop {
        println!("item: {}", item.data());
        if let Some(next_item) = item.next() {
            item = next_item;
        } else {
            break;
        }
    }

    let mut list = DoublyLinkedList::new("head");
    list.append("first");
    list.append("second");
    list.append("third");
    list.append("tail");
    println!("doublylinkedlist head: {:?}", list.head().borrow().data());
    let mut item = list.head();
    loop {
        println!("item: {}", item.borrow().data());
        if item.borrow().next.is_none() {
            break;
        } else {
            let next_item = item.borrow().next.as_ref().unwrap().clone();
            item = next_item;
        }
    }

    let list = SinglyLinkedListC::new("head");
    let list = list.append("middle");
    let list = list.append("tail");
    println!("SinglyLinkedListC list: {:?}", list);
    println!(" SinglyLinkedListC list: {:?}", list.head());
    let mut item = list.head();
    loop {
        println!("SinglyLinkedListC item: {}", item.data());
        if let Some(next_item) = item.next() {
            item = next_item;
        } else {
            break;
        }
    }
    let mut custom_alloc_vec: Vec<i32, _> = Vec::with_capacity_in(10, PassThruAllocator);
    (1..=10).for_each(|elem| custom_alloc_vec.push(elem));
    println!("custom_alloc_vec: {:?}", custom_alloc_vec);

    let mut custom_alloc_vec: Vec<i32, _> = Vec::with_capacity_in(10, BasicAllocator);
    (1..=10).for_each(|elem| custom_alloc_vec.push(elem));
    println!("custom_alloc_vec: {:?}", custom_alloc_vec);

    // example cfg macro usage
    println!("running on: {}", get_platform());
    if cfg!(target_feature = "avx2") {
        println!("avx2 enabled");
    } else {
        println!("avx2 not enabled");
    }
    if cfg!(not(any(target_arch = "x86", target_arch = "x86_64"))) {
        println!("code running on a non Intel CPU");
    }
}
