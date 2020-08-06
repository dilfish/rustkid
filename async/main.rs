// rust

use futures::executor::block_on;
use std::fmt;

struct Song {
    score: i32
}

impl fmt::Display for Song {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.score)
    }
}

async fn learn_song() -> Song {
    println!("learn song!");
    Song{score: 1}
}

async fn sing_song(song: Song) {
    println!("sing song {}", song);
}

async fn dance() {
    println!("dance");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
