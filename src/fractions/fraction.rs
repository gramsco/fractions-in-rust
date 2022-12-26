use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

use super::{convert_to_same_den::convert_to_same_denominator, factors::pgcd};
pub struct Fraction<T> {
    pub numerator: T,
    pub denominator: T,
}

pub fn fraction(a: u64, b: u64) -> Fraction<u64> {
    Fraction::from(a, b)
}

// main IMPL
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

// maths traits
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
    type Output = Fraction<u64>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.denominator == rhs.denominator {
            return Fraction::from(self.numerator + rhs.numerator, self.denominator);
        }
        let (fraction1, fraction2) = convert_to_same_denominator(self, rhs);
        fraction1.add(fraction2)
    }
}

impl PartialEq<Self> for Fraction<u64> {
    fn eq(&self, other: &Self) -> bool {
        let (f1, f2) = (self.simplify(), other.simplify());
        f1.numerator == f2.numerator && f1.denominator == f2.denominator
    }
}

impl PartialEq<f64> for Fraction<u64> {
    fn eq(&self, other: &f64) -> bool {
        self.evaluate() == *other
    }
}

// Utils traits
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

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod operations_on_fractions {
        use crate::fractions::fraction::Fraction;

        #[test]
        fn test_equality_when_obviously_equals() {
            let f1 = Fraction::from(1, 5);
            let f2 = Fraction::from(1, 5);
            assert_eq!(f1, f2);
        }

        #[test]
        fn test_equality_when_equals_when_simplified() {
            let f1 = Fraction::from(1, 5);
            let f2 = Fraction::from(2, 10);
            let f3 = Fraction::from(10, 50);
            assert_eq!(f1, f2);
            assert_eq!(f1, f3);
            assert_eq!(f2, f3);
        }

        #[test]
        fn test_equality_when_not_equals() {
            let f1 = Fraction::from(1, 5);
            let f2 = Fraction::from(2, 6);
            assert_ne!(f1, f2);
        }

        #[test]
        fn test_equality_with_float() {
            let f1 = Fraction::from(1, 5);
            assert_eq!(f1, 0.2);
        }

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
        fn add_fractions_with_same_denominators() {
            let f1 = Fraction::from(1, 2);
            let f2 = Fraction::from(2, 2);
            assert_eq!(f1 + f2, Fraction::from(3, 2));
        }

        #[test]
        fn add_fractions_with_different_denominators() {
            let f1 = Fraction::from(1, 2);
            let f2 = Fraction::from(1, 4);
            assert_eq!(f1 + f2, Fraction::from(3, 4));
        }
    }

    mod simplification {
        use crate::fractions::fraction::Fraction;

        #[test]
        fn test_is_simplified() {
            let fraction1 = Fraction::from(1, 2);
            assert!(fraction1.is_simplified())
        }

        #[test]
        #[should_panic]
        fn test_is_simplified_with_zero_denominator() {
            let fraction1 = Fraction::from(1, 0);
            fraction1.is_simplified();
        }

        #[test]
        fn test_is_simplified_with_non_simplified() {
            let fraction1 = Fraction::from(2, 4);
            assert_eq!(fraction1.is_simplified(), false)
        }

        #[test]
        fn simplify_when_necessary() {
            let fraction = Fraction::from(2, 8);
            assert_eq!(fraction.simplify(), Fraction::from(1, 4));
        }

        #[test]
        fn does_not_simplify_when_not_necessary() {
            let fraction = Fraction::from(1, 4);
            assert_eq!(fraction.simplify(), fraction);
        }
    }
}
