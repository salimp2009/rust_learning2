use std::iter::Extend;

#[derive(Debug)]
struct MyVec<T>(Vec<T>);

impl<T> MyVec<T> {
    fn new() -> MyVec<T> {
        // MyVec::<T>(Vec::<T>::new())
        // the type T is interfered since the return type have type T
        MyVec(Vec::new())
    }

    fn add(&mut self, elem: T) {
        self.0.push(elem);
    }
}

impl<T> Extend<T> for MyVec<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        iter.into_iter().for_each(|elem| self.add(elem));
        // for elem in iter {
        //     self.add(elem);
        // }
    }
}

pub fn extend_myvec() {
    let mut myvec: MyVec<i32> = MyVec::new();
    myvec.extend([1, 2, 3, 4, 6].into_iter());
    println!("myvec: {myvec:#?}");
}

pub fn extend_example() {
    let mut messages = String::from("the first three letters are: ");
    messages.extend(['a', 'b', 'c'].iter());
    println!("messages extended: {messages:?} ");
}

// pub fn extend_example2(v: &mut dyn Extend<bool>) {
//     v.extend(std::iter::once(true));
// }

#[cfg(test)]
mod extend_tests {
    use super::*;
    #[test]
    fn extend_test1() {
        extend_example();
        let x = 5;
        let y = 5;
        println!("x == y: {}", x == y);
    }

    #[test]
    fn test_myvec() {
        extend_myvec();
    }
}
