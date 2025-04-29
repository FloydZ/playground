// use arithmetic::traits::Unsigned;
use num::{Integer, FromPrimitive};

#[derive(Debug)]
struct Int<T, const N: usize> 
where 
    T: FromPrimitive + Integer + Copy
{
    pub dp: [T; N],
    pub used: usize,
    pub sign: usize,    // 0 = pos; 1 = neg
}

impl<T, const N: usize> Default for Int<T, N> 
where 
    T: FromPrimitive + Integer + Copy
{
    fn default() -> Int<T, N> {
        let a = FromPrimitive::from_u64(0 as u64).unwrap();
        Int {
            dp: [a ; N],
            used: 0,
            sign: 0,
        }
    }
   
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let t = Int::<u64, 10>::default();

    }
}
