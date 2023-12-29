use futures::executor::block_on;

pub async fn using_futures() {
    println!("hello async");
}

fn main() {
    let future = using_futures();
    block_on(future);
}
