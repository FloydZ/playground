pub fn celsuis2fahrenheit(c: i32) -> i32 {
    c*9/5 + 32
}

pub fn fahrenheit2celsius(f: i32) -> i32 {
    (f - 32)*5/32
}

pub fn fib_loop(n: u32) -> u64 {
    if n == 1 {
        return 0;
    } else { 
        let mut f0:u64 = 0;
        let mut f1:u64 = 1;
        for _ in 0..n-1 {
            let f = f1;
            f1 = f1 + f0;
            f0 = f;
        }
        f1
    }
}

pub fn fib_rec(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_rec(n-1) + fib_rec(n-2)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci() {
        let r1 = fib_loop(3);
        let r2 = fib_rec(3);
        assert_eq!(r1, 2);
        assert_eq!(r1, r2);
    }
}
