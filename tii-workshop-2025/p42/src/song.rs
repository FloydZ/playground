use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::io::Error;
use std::net::{TcpListener, TcpStream, Shutdown};

pub struct Song {
    pub text: Vec<String>,
}

impl Song {
    fn new() -> Song {
        let t1: &Vec<String> = &vec![
            String::from("A partridge in a pear tree\n"),
            String::from("Two turtle doves,\n"),
            String::from("Three French hens,\n"),
        ];
        let t2: Vec<&str> = vec![
            "first",
            "second",
            "third",
        ];
        let mut r = Vec::<String>::new();
        for i in 0..3 {
 let s = t2[i];
            r.push(format!("On the {s} day of Christmas my true love sent to me\n"));
            for j in t1.into_iter().take(i+1).rev() {
                r.push(j.clone());
            }
        };

        return Song {text: r};
    }
}

pub struct SongIter {
    pub song: Song,
    ctr: usize,
}

impl SongIter {
    pub fn new() -> SongIter {
        SongIter {
            song: Song::new(),
            ctr: 0,
        }
    }
}

impl Iterator for SongIter {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ctr >= self.song.text.len() {
            return None;
        } else {
            let c = self.ctr;
            self.ctr += 1;
            Some(self.song.text[c].clone())
        }
    }
}

fn song_to_string(iter: SongIter) -> String {
    iter.collect::<Vec<String>>().join("\n")
}

fn song_to_file(iter: SongIter, filePath: String) -> Result<(), Error> {
    let f = File::create(filePath)?;
    let s = song_to_string(iter);
    {
        let mut writer = BufWriter::new(f);
        writer.write(s.as_bytes())?;
    } 
    Ok(())
}

pub fn song_to_tcp(iter: SongIter) -> Result<(), Error> {
    let mut stream = TcpStream::connect("127.0.0.1:3333")?;
    let s = song_to_string(iter);
    {
        stream.write(s.as_bytes())?;
    } 
    Ok(())
}


pub fn song_from_tcp() -> Result<SongIter, Error> {
    let listener = TcpListener::bind("127.0.0.1:3333")?;
    let mut t = SongIter {
        ctr: 0,
        song: Song {
            text: Vec::<String>::new(),
        },
    };
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let mut data = [0 as u8; 10000];
                match stream.read(&mut data) {
                    Ok(size) => {
                        // echo everything!
                        // stream.write(&data[0..size]).unwrap();
                        let s = match String::from_utf8(data.to_vec()) {
                            Ok(v) => v,
                            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                        };
                        let ss = s.split("\n");
                        t.song.text = ss.map(|x| String::from(x)).collect::<Vec<String>>();
                        println!("kek");
                        stream.shutdown(Shutdown::Both).unwrap();
                        true
                    },
                    Err(_) => {
                        println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                        stream.shutdown(Shutdown::Both).unwrap();
                        false
                    }
                };
            }
            Err(e) => {
                println!("Listening Error: {}", e);
            }
        }

        break;
    }

    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let si =  SongIter::new();
        for s in si {
            println!("{}", s);
        }
    }

    #[test]
    fn test_song_to_string() {
        let it =  SongIter::new();
        let t = song_to_string(it);
        println!("{}", t);
    }

    #[test]
    fn test_song_to_file() {
        let it =  SongIter::new();
        let t = song_to_file(it, "du.kek".to_string());
        t.unwrap()
    }
}
