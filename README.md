# Simple operations on fractions implemented in TDD

see main.rs:

```rust
mod fractions;
use crate::fractions::fraction::fraction;

fn main() {
    let f1 = fraction(1, 5);
    let f2 = fraction(2, 10);

    let x = f1.evaluate();
    println!("{x}"); // 0.2

    let check = f1 == f2;
    let check2 = f1 == 0.2;
    println!("{check}, {check2}"); // true, true

    let y = f1 + f2;
    println!("{y:?}"); // 2/5

    let z = f1 * f2;
    println!("{z:?}"); // 2/50 (<==> 1/25)
    println!("{}", z.evaluate()) // 0.04
}
```
