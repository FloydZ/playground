use p42::song::{SongIter, song_to_tcp};

fn main() {
    let t = SongIter::new();
    song_to_tcp(t);
}
