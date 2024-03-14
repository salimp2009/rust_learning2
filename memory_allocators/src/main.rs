use std::{borrow::BorrowMut, mem};

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
}
