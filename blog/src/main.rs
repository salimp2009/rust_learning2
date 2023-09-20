use blog::post::Post;
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
}
