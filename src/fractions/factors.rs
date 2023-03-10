pub fn ppcm(a: u64, b: u64) -> u64 {
    match (a, b) {
        (0, _) => 0,
        (_, 0) => 0,
        (a, b) => {
            if a == b {
                return a;
            }

            let min = match a < b {
                true => b,
                false => a,
            };

            let max = a * b;

            for el in (min..max).step_by(min.try_into().unwrap()) {
                if el % a == 0 && el % b == 0 {
                    return el;
                };
            }
            return max;
        }
    }
}

pub fn pgcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    return pgcd(b, a % b);
}

#[cfg(test)]
mod tests {

    use super::pgcd;
    use crate::fractions::factors::ppcm;

    #[test]
    fn ppcm_works() {
        let x = 8;
        let y = 5;
        assert_eq!(ppcm(x, y), 40);

        let x = 2;
        let y = 4;
        assert_eq!(ppcm(x, y), 4);

        let x = 17;
        let y = 21;
        assert_eq!(ppcm(x, y), x * y);
    }

    #[test]
    fn pgcd_works() {
        let a = 120;
        let b = 40;
        let p = pgcd(a, b);
        assert_eq!(p, b);
    }
}
