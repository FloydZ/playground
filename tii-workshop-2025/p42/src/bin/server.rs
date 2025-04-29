use p42::song::{SongIter, song_from_tcp};

fn main() {
    let t = song_from_tcp().unwrap();
    for i in t.song.text {
        println!("{}", i);
    }
}
