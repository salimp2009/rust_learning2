pub trait Summary {
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
    fn summarize(&self) -> String {
        format!("user '{}' says '{}' ", self.username, self.content)
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
}
