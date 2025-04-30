use rug::{Float, Integer};
use std::{
    cmp::PartialOrd,
    fmt::{Debug, Display},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}
};

pub trait BigNumber:
    From<i32>
    + PartialEq
    + PartialOrd<Self>
    + Clone
    + Debug
    + Default
    + Display
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'a> MulAssign<&'a Self>
    + for<'a> Div<&'a Self, Output = Self>
    + for<'a> DivAssign<&'a Self>
    + Sum<Self>
{
    //pub fn is_zero(&self) -> bool;
}

impl<T> BigNumber for T where T:
    From<i32>
    + PartialEq
    + PartialOrd<Self>
    + Clone
    + Debug
    + Default
    + Display
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'a> MulAssign<&'a Self>
    + for<'a> Div<&'a Self, Output = Self>
    + for<'a> DivAssign<&'a Self>
    + Sum<Self>
{}

pub type BigInteger = Integer;
pub type BigFloat = Float;

//pub trait BigNumber<'a>
//where
//    Self: Sized,
//    Self: Default,
//    Self: Add<Self>,
//    &'a Self:Add<&'a Self>,
//{
//    // fn rand_normal(n: usize, exp_from: i32, exp_to: i32, gs: G, sign_positive: bool) -> Vec<Self>;
//    fn new() -> Self;
//    fn add(&self, rhs: &Self) -> Self;
//    fn sub(&self, rhs: &Self) -> Self;
//    fn mul(&self, rhs: &Self) -> Self;
//    fn div(&self, rhs: &Self) -> Self;
//}
//
//
//#[derive(Clone, Default)]
//pub struct BigInteger(Integer);
//
//
//impl BigNumber for BigInteger {
//    fn new() -> Self {
//        Self(Integer::new())
//    }
//
//    fn add(&self, rhs: &Self) -> Self {
//        Self((self.0.borrow() + rhs.0.borrow()).complete())
//    }
//
//    fn sub(&self, rhs: &Self) -> Self {
//        Self((self.0.borrow() - rhs.0.borrow()).complete())
//    }
//
//    fn mul(&self, rhs: &Self) -> Self {
//        Self((self.0.borrow() * rhs.0.borrow()).complete())
//    }
//
//    fn div(&self, rhs: &Self) -> Self {
//        Self((self.0.borrow() / rhs.0.borrow()).complete())
//    }
//}
//
//impl Add for BigInteger {
//    type Output = Self;
//
//    fn add(self, rhs: Self) -> Self {
//        BigNumber::add(&self, &rhs)
//    }
//}
//
//impl Add for &BigInteger {
//    type Output = Self;
//
//    fn add(self, rhs: Self) -> Self {
//        BigNumber::add(self, rhs);
//        &self
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = BigInteger::new();
        let b = BigInteger::new();
        let c= a+b;
        assert!(c.is_zero());
    }
}

