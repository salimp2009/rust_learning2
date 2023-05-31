pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn strlen_m(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

// same as above definition of strlen_m
pub fn strlen_2<S>(s: S) -> usize
where
    S: AsRef<str>,
{
    s.as_ref().len()
}

pub fn foo() {
    println!("{}", strlen_m("hello world"));
    println!("{}", strlen_m(String::from("hei Salitosssss")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strlen_test1() {
        foo();
        assert_eq!(strlen_m("hello"), 5);
    }
}
