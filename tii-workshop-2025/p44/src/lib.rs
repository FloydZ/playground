use std::collections::HashMap;

macro_rules! hash_map {
    ($( $x:expr => $y:expr, )* ) => {
        {
            let mut t = HashMap::<i32, bool>::new();
            $(
                t.insert($x, $y);
            )*
            t
        } 
    };
}

fn test2(s: &mut String) {

}

fn test() {
    let mut s = String::new();
    test2(&mut s);
    println!("{s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let map = hash_map!(
            42 => true,
            64 => false,
            128 => true,
        );

        for (key, value) in &map {
            println!("{}: {}", key, value);
        }
    }
}
