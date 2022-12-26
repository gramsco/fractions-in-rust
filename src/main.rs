mod fractions;
use crate::fractions::fraction::Fraction;

fn main() {
    let f1 = Fraction::from(1, 5);
    let f2 = Fraction::from(2, 10);
    let check = f1 == f2;
    let check2 = f1 == 0.2;
    println!("{check}, {check2}");
}
