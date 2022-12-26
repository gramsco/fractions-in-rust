use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

use crate::factors::{pgcd, ppcm};

pub struct Fraction<T> {
    numerator: T,
    denominator: T,
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
        Fraction::new(self.numerator, self.denominator)
    }
}

pub fn find_common_basis(
    fraction1: Fraction<u64>,
    fraction2: Fraction<u64>,
) -> (Fraction<u64>, Fraction<u64>) {
    let (fraction1, fraction2) = (fraction1.simplify(), fraction2.simplify());

    if fraction1.denominator == fraction2.denominator {
        return (fraction1, fraction2);
    }

    let pcm = ppcm(fraction1.denominator, fraction2.denominator);

    let f1_multiplier = pcm / fraction1.denominator;
    let f2_multiplier = pcm / fraction2.denominator;

    let fraction1 = fraction1.multiply_by_unity_fraction(f1_multiplier);
    let fraction2 = fraction2.multiply_by_unity_fraction(f2_multiplier);

    (fraction1, fraction2)
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
            find_common_basis(Fraction::new(self.numerator, self.denominator), rhs);

        Fraction::new(
            fraction1.numerator + fraction2.numerator,
            fraction1.denominator,
        )
    }

    type Output = Fraction<u64>;
}

impl Fraction<u64> {
    pub fn new(numerator: u64, denominator: u64) -> Fraction<u64> {
        if denominator == 0 {
            panic!()
        }
        Fraction {
            numerator,
            denominator,
        }
    }

    fn multiply_by(&self, fraction: Fraction<u64>) -> Fraction<u64> {
        Fraction::new(
            self.numerator * fraction.numerator,
            self.denominator * fraction.denominator,
        )
    }

    fn multiply_by_unity_fraction(&self, n: u64) -> Fraction<u64> {
        self.multiply_by(Fraction::new(n, n))
    }

    pub fn is_simplified(&self) -> bool {
        pgcd(self.numerator, self.denominator) == 1
    }

    pub fn simplify(&self) -> Self {
        if self.is_simplified() {
            return Fraction {
                numerator: self.numerator,
                denominator: self.denominator,
            };
        }
        let pgcd = pgcd(self.numerator, self.denominator);
        Fraction::new(self.numerator / pgcd, self.denominator / pgcd)
    }

    pub fn evaluate(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

#[cfg(test)]

mod tests {
    use crate::{find_common_basis, Fraction};

    #[test]
    fn test_evaluate() {
        let f = Fraction::new(1, 5);
        assert_eq!(f.evaluate(), 0.2);
    }

    #[test]
    fn adds_with_float() {
        let a = Fraction::new(1, 5);
        let b = 0.2;
        assert_eq!(a + b, 0.4);
    }

    #[test]
    fn test_multiply_by_unity_fraction() {
        let n = 3;
        let f = Fraction::new(3, 5);
        assert_eq!(f.multiply_by_unity_fraction(n), Fraction::new(9, 15));
    }

    #[test]
    fn test_is_simplified() {
        let fraction1 = Fraction {
            numerator: 1,
            denominator: 2,
        };
        assert!(fraction1.is_simplified())
    }

    #[test]

    fn test_is_simplified_with_zero_denominator() {
        let fraction1 = Fraction {
            numerator: 1,
            denominator: 0,
        };
        assert!(fraction1.is_simplified());
    }

    #[test]
    fn test_is_simplified_with_non_simplified() {
        let fraction1 = Fraction {
            numerator: 2,
            denominator: 4,
        };
        assert_eq!(fraction1.is_simplified(), false)
    }

    #[test]
    fn test_find_common_basis_returns_same_if_common_basis() {
        let fraction1 = Fraction {
            numerator: 1,
            denominator: 2,
        };
        let fraction2 = Fraction {
            numerator: 3,
            denominator: 2,
        };
        let (fraction1, fraction2) = find_common_basis(fraction1, fraction2);
        assert_eq!(
            fraction1,
            Fraction {
                numerator: 1,
                denominator: 2
            }
        );
        assert_eq!(
            fraction2,
            Fraction {
                numerator: 3,
                denominator: 2
            }
        );
    }

    #[test]
    fn test_find_common_basis() {
        let fraction1 = Fraction {
            numerator: 1,
            denominator: 2,
        };
        let fraction2 = Fraction {
            numerator: 1,
            denominator: 4,
        };
        let (fraction1, fraction2) = find_common_basis(fraction1, fraction2);
        assert_eq!(
            fraction1,
            Fraction {
                numerator: 2,
                denominator: 4
            },
        );
        assert_eq!(fraction2, fraction2);
    }

    #[test]
    fn fractions_with_same_denominator() {
        let f1 = Fraction::new(1, 2);
        let f2 = Fraction::new(2, 2);
        assert_eq!(f1 + f2, Fraction::new(3, 2));
    }
    #[test]
    fn fractions_with_different_denominator() {
        let f1 = Fraction {
            numerator: 1,
            denominator: 2,
        };
        let f2 = Fraction {
            numerator: 1,
            denominator: 4,
        };

        assert_eq!(
            f1 + f2,
            Fraction {
                numerator: 3,
                denominator: 4
            }
        );
    }
}
