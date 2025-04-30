//! Basic vector structures for LLL
use super::number::{BigInteger, BigNumber};

use std::{
    fmt,
    ops::{Index, IndexMut, Add, Sub},
    vec::IntoIter,
};


/// Implementation of a vector without generic coefficients
#[derive(Clone, PartialEq)]
pub struct Vector<T: BigNumber> {
    /// Internal representation as a list of coefficients
    coefficients: Vec<T>,
}

impl<T: BigNumber> Vector<T> {
    //pub fn basis_vector(dimension: usize, position: usize) -> Self {
    //    assert!(position < dimension);
    //    let coefficients = (0..dimension)
    //        .map(|i| {
    //            if i == position {
    //                T::from(1)
    //            } else {
    //                T::from(0)
    //            }
    //        })
    //        .collect();
    //    Self { coefficients }
    //}
    /// Create an instance from a `Vec`
    
    #[inline]
    pub fn from_vector(coefficients: Vec<T>) -> Self {
        Self { coefficients }
    }

    pub fn init(dimension: usize) -> Self {
        Self {
            coefficients: vec![Default::default(); dimension],
        }
    }
    
    pub fn zero(dimension: usize) -> Self {
        Vector::<T>::init(dimension)
    }

    pub fn dimension(&self) -> usize {
        self.coefficients.len()
    }

    pub fn add(&self, other: &Self) -> Self {
        let n = self.dimension();
        assert_eq!(n, other.dimension());

        Self::from_vector(
            (0..n)
                .map(|i| self.coefficients[i].clone() + &other.coefficients[i])
                .collect(),
        )
    }

    pub fn sub(&self, other: &Self) -> Self {
        let n = self.dimension();

        assert_eq!(n, other.dimension());

        Self::from_vector(
            (0..n)
                .map(|i| self.coefficients[i].clone() - &other.coefficients[i])
                .collect(),
        )
    }

    /// Multiplication by a scalar
    pub fn mulf(&self, other: T) -> Self {
        let n = self.dimension();

        Self::from_vector(
            (0..n)
                .map(|i| self.coefficients[i].clone() * &other)
                .collect(),
        )
    }

    pub fn is_zero(&self) -> bool {
        self == &Vector::zero(self.dimension())
    }

    pub fn dot(&self, other: &Self) -> T {
        self.coefficients
            .iter()
            .zip(&other.coefficients)
            .map(|(coeff_r, coeff_l)| coeff_r.clone() * coeff_l)
            .sum()
    }
}


impl<T: BigNumber> Add for Vector<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector::<T>::add(&self, &rhs)
    }
}

impl<T: BigNumber> Sub for Vector<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vector::<T>::sub(&self, &rhs)
    }
}

impl<T: BigNumber> IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.coefficients.into_iter()
    }
}

impl<T: BigNumber> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.coefficients[index]
    }
}

impl<T: BigNumber> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.coefficients[index]
    }
}

impl<T: BigNumber> fmt::Debug for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?}", self.coefficients)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = Vector::<BigInteger>::init(100);
        let b = Vector::<BigInteger>::init(100);
        let c = a + b;
        for a in c {
            assert!(a.is_zero());
        }
    }
}
