use std::{assert_eq, vec};

pub trait IteratorExt: Iterator {
    fn flatten_ext(self) -> FlattenM<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator,
    {
        flatten_m(self)
    }
}

impl<T> IteratorExt for T
where
    T: Iterator,
{
    fn flatten_ext(self) -> FlattenM<Self>
    where
        Self: Sized,
        Self::Item: IntoIterator,
    {
        flatten_m(self)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infinite_flattened() {
        let mut iter = flatten_m((0..).map(|i| 0..i));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(0));
    }

    #[test]
    fn flatten_more() {
        let mut iter = flatten_m(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]]);
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b3"));
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next(), Some("a3"));
        assert_eq!(iter.next_back(), Some("b1"));
    }
    #[test]
    fn flatten_data() {
        let data2 = vec![vec![1, 2, 3, 4], vec![5, 6]];
        let flattened2 = flatten_m(data2).collect::<Vec<u8>>();
        println!("flattened2: {flattened2:?}");

        let data3 = vec![vec![1, 2, 3, 4], vec![5, 6]];
        let flattened3 = flatten_m(data3).rev().collect::<Vec<u8>>();
        println!("flattened3: {flattened3:?}");
    }

    #[test]
    fn deep_flatten() {
        let iter = flatten_m(flatten_m(vec![vec![vec![0, 1]]])).count();
        assert_eq!(iter, 2);
    }

    #[test]
    fn ext_flatten_ext() {
        let iter = (vec![vec![0, 1]]).into_iter().flatten_ext().count();
        assert_eq!(iter, 2);
    }

    #[test]
    fn ext_flatten() {
        let iter = (vec![vec![0, 1]]).into_iter().flatten().count();
        assert_eq!(iter, 2);
    }
}
