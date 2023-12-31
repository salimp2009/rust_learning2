use futures::executor::block_on;

#[derive(Debug)]
struct Song {
    title: String,
    id: i32,
}

async fn learn_song() -> Song {
    Song {
        title: "dabadabadoo".to_string(),
        id: 5,
    }
}

async fn sing_song(song: Song) {
    println!("Singing: {}, {}", song.title, song.id);
}

pub async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}
pub async fn using_futures() {
    println!("hello async");
}

pub async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = using_futures();
    futures::join!(f1, f2);
}

fn main() {
    let _future = using_futures();
    block_on(using_futures());
    block_on(async_main());
}
