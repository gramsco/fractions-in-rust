use super::{factors::ppcm, fraction::Fraction};

pub fn convert_to_same_denominator(
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

#[cfg(test)]

mod tests {
    use crate::fractions::{
        convert_to_same_den::{self, convert_to_same_denominator},
        fraction::Fraction,
    };

    #[test]
    fn test_find_common_basis_returns_same_if_common_basis() {
        let fraction1 = Fraction::from(1, 2);
        let fraction2 = Fraction::from(3, 2);
        let (fraction1, fraction2) = convert_to_same_denominator(fraction1, fraction2);
        assert_eq!(fraction1, fraction1);
        assert_eq!(fraction2, Fraction::from(3, 2));
    }

    #[test]
    fn test_find_common_basis() {
        let fraction1 = Fraction::from(1, 2);
        let fraction2 = Fraction::from(1, 4);
        let (fraction1, fraction2) = convert_to_same_denominator(fraction1, fraction2);
        assert_eq!(fraction1, Fraction::from(2, 4));
        assert_eq!(fraction2, fraction2);
    }
}
