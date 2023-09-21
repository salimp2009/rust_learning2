use blog::post::Post;
use blog::post2::Post2;

fn main() {
    let mut post = Post::new();
    println!("default post: {:?}", post.content());
    post.add_text("I did a horrible mistake today :(");
    assert_eq!(post.content(), "");
    println!("draft post: {:?}", post.content());
    post.request_review();
    assert_eq!(post.content(), "");
    println!("reviewed post: {:?}", post.content());
    post.approve();
    assert_eq!(post.content(), "I did a horrible mistake today :(");
    println!("approved post: {:?}", post.content());

    let mut post2 = Post2::new();
    post2.add_text("I ate a banana today and ice cream");
    let post2 = post2.request_review();
    let post2 = post2.approve();
    assert_eq!(post2.content(), "I ate a banana today and ice cream");
    println!("post2 : {:?}", post2.content());
}
