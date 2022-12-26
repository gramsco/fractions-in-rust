mod factors;

mod fractions;
use fractions::*;

fn main() {
    let v1 = Fraction::new(1, 2);
    let v2 = 0.12;
    let v3 = v1 + v2;
    println!("{v1:?} + {v2} = {v3}");
}
