// pub fn collect_m(s: &dyn FromIterator<bool>) -> Vec<bool> {
//     s.collect()
// }

pub fn extend_example() {
    let mut messages = String::from("the first three letters are: ");
    messages.extend(['a', 'b', 'c'].iter());
    println!("messages extended: {messages:?} ");
}

#[cfg(test)]
mod collect_tests {
    use super::*;
    #[test]
    fn extend_test1() {
        extend_example();
        let x = 5;
        let y = 5;
        println!("x == y: {}", x == y);
    }
}
