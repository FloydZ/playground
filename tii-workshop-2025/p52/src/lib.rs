#![allow(dead_code)]
#![allow(unused_variables)]

use rayon::prelude::*;
use std::{sync::atomic::AtomicU64, thread};
use std::sync::atomic::Ordering::AcqRel;


const N: usize = 2;
static ATOMIC_CTR:AtomicU64 = AtomicU64::new(0);

fn square(i: u32) -> u64 {
    let t: u64 = i as u64;
    return t*t;
}

fn square2(i: &u32) -> u64 {
    let t: u64 = *i as u64;
    return t*t;
}

fn map_sum1<const N: usize>(v: Vec<u32>, f: fn(u32) -> u64) -> u64 {
    let mut children = vec![];
    let split_size = v.len() / N;
    for i in 0..N {
        let start: usize = i*split_size;
        let end: usize = (i+1)*split_size;
        let cc = v.clone();
        let handle = thread::spawn(move || -> u64 {
            let mut sum: u64 = 0;
            for j in start..end {
                sum += f(cc[j]);
            }
            return sum;
        });

        children.push(handle);
    }

    let mut sum: u64 = 0;
    for c in children {
        sum += c.join().unwrap();
    }

    sum
}

fn map_sum2<const N: usize>(v: Vec<u32>, f: fn(u32) -> u64) {
    let mut children = vec![];
    let split_size = v.len() / N;
    for i in 0..N {
        let start: usize = i*split_size;
        let end: usize = (i+1)*split_size;
        let cc = v.clone();
        let handle = thread::spawn(move ||  {
            for j in start..end {
                let t= f(cc[j]);
                ATOMIC_CTR.fetch_add(t, AcqRel);
            }
        });

        children.push(handle);
    }

    for c in children {
        _ = c.join();
    }
}


fn map_sum4<const N: usize>(v: Vec<u32>, f: fn(u32) -> u64) -> u64 {
    let t = v.par_iter()
        .copied()
        .map(f)
        .sum::<u64>();
    t
}

fn map_sum5<const N: usize>(v: &mut [u32], f: fn(u32) -> u64) -> u64 {
    let at: AtomicU64 = AtomicU64::new(0);
    let split_size = v.len() / N;

    thread::scope(|s| {
        for i in 0..N {
            s.spawn(|| {
                let i = rayon::current_thread_index().unwrap();
                let start = i*split_size;
                let end = (i+1)*split_size;
                let mut sum = 0;
                for j in start..split_size {
                    sum += v[j] as u64; 
                }
                at.fetch_add(sum, AcqRel);
            });
        }
    });
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut data =  Vec::<u32>::new();
        for i in 0..N*100 {
            data.push(i as u32);
        }
        let t = map_sum1::<N>(data, square);
        println!("{t}");
    }

    #[test]
    fn it_works2() {
        let mut data =  Vec::<u32>::new();
        for i in 0..N*100 {
            data.push(i as u32);
        }
        let t = map_sum5::<N>(&mut data[..], square);
        println!("{t}");
    }
}
