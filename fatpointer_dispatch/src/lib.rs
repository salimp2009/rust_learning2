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

pub fn strlen_3(s: Box<dyn AsRef<str>>) -> usize {
    s.as_ref().as_ref().len()
}

pub fn strlen_4(s: &dyn AsRef<str>) -> usize {
    s.as_ref().len()
}

pub fn foo() {
    println!("{}", strlen_m("hello world"));
    println!("{}", strlen_m(String::from("hei Salitosssss")));

    println!(
        "str_len3 with dyn AsRef with &str: {}",
        strlen_3(Box::new("sssss"))
    );

    println!(
        "strlen_3; dyn AsRef with String: {}",
        strlen_3(Box::new(String::from("hei Salitosssss")))
    );

    println!(
        "strlen_3; dyn AsRef with String: {}",
        strlen_3(Box::new(String::from("hei Salitosssss")))
    );

    "J".hi();

    bar(&[&"J", &"Joo"]);
    bar(&[&String::from("Didokis"), &"salitos"]);
    bar_box(&[Box::new(String::from("Didokis")), Box::new("salitos")]);
}

pub fn bar(s: &[&dyn Hello]) {
    s.iter().for_each(|&elem| elem.hi());
}

pub fn bar_box(s: &[Box<dyn Hello>]) {
    s.iter().for_each(|elem| elem.hi());
}

pub fn baz(s: &(dyn Hello + Send + Sync)) {
    s.hi();
    let s = s;
    s.hi();
}

pub trait HelloAsRef: Hello + AsRef<str> {}

impl HelloAsRef for &str {}

pub fn baz_hi(s: &dyn HelloAsRef) {
    s.hi();
    let s = s.as_ref();
    println!("s length: {}", s.len());
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
    #[test]
    fn strlen_test3() {
        baz(&("semooos"));
        baz(&(String::from("salitosss")));
        baz_hi(&("trait objects cool!"));
        assert_eq!(strlen_m("hello salim"), 11);
    }
}
