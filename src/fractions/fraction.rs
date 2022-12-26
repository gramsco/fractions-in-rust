use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

use super::{convert_to_same_den::convert_to_same_denominator, factors::pgcd};
pub struct Fraction<T> {
    pub numerator: T,
    pub denominator: T,
}

impl<T: PartialEq> PartialEq for Fraction<T> {
    fn eq(&self, other: &Self) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }
}

impl Debug for Fraction<u64> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_simplified() {
            print!("{}/{}", self.numerator, self.denominator);
            return Ok(());
        } else {
            let simpl = self.simplify();
            print!("{}/{} (<==> {simpl:?})", self.numerator, self.denominator,);
            return Ok(());
        }
    }
}

impl Copy for Fraction<u64> {}

impl Clone for Fraction<u64> {
    fn clone(&self) -> Self {
        Fraction::from(self.numerator, self.denominator)
    }
}

#[derive(Debug)]
struct DivideByZero;

impl Mul for Fraction<u64> {
    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply_by(rhs)
    }
    type Output = Fraction<u64>;
}

impl Add<f64> for Fraction<u64> {
    type Output = f64;
    fn add(self, rhs: f64) -> Self::Output {
        self.evaluate() + rhs
    }
}

impl Add<Self> for Fraction<u64> {
    fn add(self, rhs: Self) -> Self::Output {
        let (fraction1, fraction2) =
            convert_to_same_denominator(Fraction::from(self.numerator, self.denominator), rhs);

        Fraction::from(
            fraction1.numerator + fraction2.numerator,
            fraction1.denominator,
        )
    }

    type Output = Fraction<u64>;
}

impl Fraction<u64> {
    pub fn from(numerator: u64, denominator: u64) -> Fraction<u64> {
        if denominator == 0 {
            panic!()
        }
        Fraction {
            numerator,
            denominator,
        }
    }

    fn multiply_by(&self, fraction: Fraction<u64>) -> Fraction<u64> {
        Fraction::from(
            self.numerator * fraction.numerator,
            self.denominator * fraction.denominator,
        )
    }

    pub fn multiply_by_unity_fraction(&self, n: u64) -> Fraction<u64> {
        self.multiply_by(Fraction::from(n, n))
    }

    pub fn is_simplified(&self) -> bool {
        pgcd(self.numerator, self.denominator) == 1
    }

    pub fn simplify(&self) -> Self {
        if self.is_simplified() {
            return self.clone();
        }
        let pgcd = pgcd(self.numerator, self.denominator);
        Fraction::from(self.numerator / pgcd, self.denominator / pgcd)
    }

    pub fn evaluate(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

#[cfg(test)]

mod tests {
    use crate::fractions::{convert_to_same_den::convert_to_same_denominator, fraction::Fraction};

    #[test]
    fn test_evaluate() {
        let f = Fraction::from(1, 5);
        assert_eq!(f.evaluate(), 0.2);
    }

    #[test]
    fn adds_with_float() {
        let a = Fraction::from(1, 5);
        let b = 0.2;
        assert_eq!(a + b, 0.4);
    }

    #[test]
    fn test_multiply_by_unity_fraction() {
        let n = 3;
        let f = Fraction::from(3, 5);
        assert_eq!(f.multiply_by_unity_fraction(n), Fraction::from(9, 15));
    }

    #[test]
    fn test_is_simplified() {
        let fraction1 = Fraction::from(1, 2);
        assert!(fraction1.is_simplified())
    }

    #[test]
    fn test_is_simplified_with_zero_denominator() {
        let fraction1 = Fraction::from(1, 0);
        assert!(fraction1.is_simplified());
    }

    #[test]
    fn test_is_simplified_with_non_simplified() {
        let fraction1 = Fraction::from(2, 4);
        assert_eq!(fraction1.is_simplified(), false)
    }

    #[test]
    fn fractions_with_same_denominator() {
        let f1 = Fraction::from(1, 2);
        let f2 = Fraction::from(2, 2);
        assert_eq!(f1 + f2, Fraction::from(3, 2));
    }

    #[test]
    fn fractions_with_different_denominator() {
        let f1 = Fraction::from(1, 2);
        let f2 = Fraction::from(1, 4);
        assert_eq!(f1 + f2, Fraction::from(3, 4));
    }
}
