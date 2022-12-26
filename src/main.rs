mod fractions;
use crate::fractions::fraction::Fraction;

fn main() {
    match Fraction::from(12, 24).is_simplified() {
        true => println!("yes"),
        _ => println!("false"),
    }
}
