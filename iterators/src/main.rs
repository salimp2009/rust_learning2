use std::{assert_eq, println, vec};

pub struct Counter {
    count: usize,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        Some(self.count)
    }
}

pub fn flatten_m<I>(iter: I) -> FlattenM<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    FlattenM::new(iter.into_iter())
}

pub struct FlattenM<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> FlattenM<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        FlattenM {
            outer: iter,
            front_iter: None,
            back_iter: None,
        }
    }
}

impl<O> Iterator for FlattenM<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut front_iter) = self.front_iter {
                if let Some(i) = front_iter.next() {
                    return Some(i);
                }
                self.front_iter = None;
            }
            if let Some(next_front_inner) = self.outer.next() {
                self.front_iter = Some(next_front_inner.into_iter());
            } else {
                return self.back_iter.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for FlattenM<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut back_iter) = self.back_iter {
                if let Some(i) = back_iter.next_back() {
                    return Some(i);
                }
                self.back_iter = None;
            }
            if let Some(next_back_inner) = self.outer.next_back() {
                self.back_iter = Some(next_back_inner.into_iter());
            } else {
                return self.front_iter.as_mut()?.next_back();
            }
        }
    }
}

pub fn flatten_things() {
    let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
    let flattened = data.into_iter().flatten().collect::<Vec<u8>>();
    assert_eq!(flattened, &[1, 2, 3, 4, 5, 6]);

    let words = ["alpha", "beta", "gamma"];
    // let merged = words
    //     .iter()
    //     .map(|word| word.chars().rev())
    //     .flatten()
    //     .collect::<String>();

    let merged = words
        .iter()
        .flat_map(|word| word.chars().rev())
        .collect::<String>();
    assert_eq!(merged, "ahplaatebammag");
    println!("{merged:?}");
}

pub fn flatten_more() {
    let mut iter = flatten_m(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]]);
    assert_eq!(iter.next(), Some("a1"));
    assert_eq!(iter.next_back(), Some("b3"));
    assert_eq!(iter.next(), Some("a2"));
    assert_eq!(iter.next_back(), Some("b2"));
    assert_eq!(iter.next(), Some("a3"));
    assert_eq!(iter.next_back(), Some("b1"));
}

pub fn infinite_flattened() {
    let mut iter = flatten_m((0..).map(|i| 0..i));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(0));
}

fn main() {
    let mut iter = vec![1, 2, 3, 4, 5].into_iter();
    while let Some(x) = iter.next() {
        println!("{x}");
    }
    let my_array: Vec<_> = ["a", "b", "c"]
        .iter()
        .map(|elem| elem.to_string() + "x")
        .collect();
    // .for_each(|element| println!("{element}"));

    println!("{:?}", my_array);

    // (0..5)
    //     .flat_map(|x| x * 100..x * 110)
    //     .enumerate()
    //     .filter(|&(i, x)| (i + x) % 3 == 0)
    //     .for_each(|(i, x)| println!("{i}:{x}"));

    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));

    let vs = vec![1, 2, 3];
    for v in vs {
        println!("{v}");
    }
    // illegal acces; vs is consumed in for loop
    // println!("{vs:?}");

    // for in and .iter() does not consume; it borrows like &
    let vs2 = vec![1, 2, 3, 4];
    for _v in vs2.iter() {
        //
    }

    // this is OK too; the for loop borrows not consumes
    for _v in &vs2 {
        //
    }

    println!("{vs2:?}");

    let mut values = vec![1, 2, 3, 4, 5];
    // for v in values.iter_mut() {
    //     *v *= 2;
    // }
    values.iter_mut().for_each(|v| *v *= 2);
    println!("{values:?}");
    flatten_things();
    assert_eq!(flatten_m(std::iter::empty::<Vec<()>>()).count(), 0);
    flatten_more();
    infinite_flattened();
}
