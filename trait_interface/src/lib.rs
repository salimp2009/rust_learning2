use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        String::from("(Read more ...)")
    }
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

#[derive(Debug)]
pub struct Twitter {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Twitter {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("user '{}' says '{}' ", self.username, self.content)
    }
}
//using &imp Trait is static dispatch
// using &dyn Trait is dynamic dispatch
pub fn notify_summaries(item: &impl Summary) {
    println!("Breaking news {}", item.summarize());
}

// this is same as above. The above is syntatic sugar for this
pub fn notify_summaries2<T: Summary + Debug>(item: &T) {
    println!("Breaking news {}", item.summarize());
}

pub fn _some_function<T, U>(_item: &T, _otheritem: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

pub fn factory_summarizable() -> impl Summary + Debug {
    Twitter {
        username: "suumer".to_string(),
        content: "".to_string(),
        reply: false,
        retweet: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let twitter1 = Twitter {
            username: "salitos".to_string(),
            content: "heyyo friends".to_string(),
            reply: true,
            retweet: false,
        };
        println!("twitter1: {}", twitter1.summarize());
    }

    #[test]
    fn test_2() {
        let article = NewsArticle {
            headline: "".to_string(),
            location: "".to_string(),
            author: "".to_string(),
            content: "".to_string(),
        };
        println!("article: {}", article.summarize());
    }
    #[test]
    fn test_3() {
        let twitter1 = Twitter {
            username: "salitos".to_string(),
            content: "heyyo friends".to_string(),
            reply: true,
            retweet: false,
        };
        println!("twitted by: {}", twitter1.summarize_author());
        notify_summaries2(&twitter1);
        notify_summaries(&twitter1);
        println!("factory_summarizable: {:?}", factory_summarizable());
    }
}
