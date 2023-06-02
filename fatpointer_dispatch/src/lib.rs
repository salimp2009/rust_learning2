pub trait Hello {
    fn hi(&self);
}

impl Hello for &str {
    fn hi(&self) {
        println!("hi {self}");
    }
}

impl Hello for String {
    fn hi(&self) {
        println!("hi {self}");
    }
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

pub fn strlen_3<S>(s: &dyn AsRef<str>) -> usize {
    s.as_ref().len()
}

pub fn foo() {
    println!("{}", strlen_m("hello world"));
    println!("{}", strlen_m(String::from("hei Salitosssss")));

    println!(
        "str_len3 with &dynAsRef with &str: {}",
        strlen_3::<&str>(&"sssss")
    );
    println!(
        "strlen_3; dynAsRef with String: {}",
        strlen_3::<&String>(&String::from("hei Salitosssss"))
    );

    "J".hi();

    bar(&[&"J", &"Joo"]);
    bar(&[&String::from("Didokis"), &"salitos"]);
}

pub fn bar(s: &[&dyn Hello]) {
    s.iter().for_each(|&elem| elem.hi());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strlen_test1() {
        assert_eq!(strlen_m("hello"), 5);
    }

    #[test]
    fn strlen_test2() {
        foo();
        assert_eq!(strlen_m("hello salim"), 11);
    }
}
