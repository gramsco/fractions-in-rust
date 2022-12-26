use super::{factors::ppcm, fraction::Fraction};

pub fn convert_to_same_denominator(
    fraction1: Fraction<u64>,
    fraction2: Fraction<u64>,
) -> (Fraction<u64>, Fraction<u64>) {
    if fraction1.denominator == fraction2.denominator {
        return (fraction1, fraction2);
    }

    let pcm = ppcm(fraction1.denominator, fraction2.denominator);

    let f1_coeff = pcm / fraction1.denominator;
    let f2_coeff = pcm / fraction2.denominator;

    (
        fraction1.multiply_by_unity_fraction(f1_coeff),
        fraction2.multiply_by_unity_fraction(f2_coeff),
    )
}

#[cfg(test)]
mod tests {
    use crate::fractions::{convert_to_same_den::convert_to_same_denominator, fraction::Fraction};

    #[test]
    fn leaves_intact_fractions_if_same_denominator() {
        let fraction1 = Fraction::from(1, 2);
        let fraction2 = Fraction::from(3, 2);
        let (fraction1, fraction2) = convert_to_same_denominator(fraction1, fraction2);
        assert_eq!(fraction1, fraction1);
        assert_eq!(fraction2, Fraction::from(3, 2));
    }

    #[test]
    fn checks_for_same_fraction_but_not_simplified() {
        let fraction1 = Fraction::from(1, 2);
        let fraction2 = Fraction::from(2, 4);
        let (fraction1, fraction2) = convert_to_same_denominator(fraction1, fraction2);
        assert_eq!(fraction1, Fraction::from(2, 4));
        assert_eq!(fraction2, fraction2);
    }

    #[test]
    fn convert_one_fraction() {
        let fraction1 = Fraction::from(1, 2);
        let fraction2 = Fraction::from(1, 4);
        let (fraction1, fraction2) = convert_to_same_denominator(fraction1, fraction2);
        assert_eq!(fraction1, Fraction::from(2, 4));
        assert_eq!(fraction2, fraction2);
    }

    #[test]
    fn convert_both_fractions() {
        let fraction1 = Fraction::from(1, 3);
        let fraction2 = Fraction::from(1, 4);
        let (fraction1, fraction2) = convert_to_same_denominator(fraction1, fraction2);
        assert_eq!(fraction1, Fraction::from(4, 12));
        assert_eq!(fraction2, Fraction::from(3, 12));
    }
}
