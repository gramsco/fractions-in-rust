# Fractions in Rust :crab:

This was mainly a POC for me to work a bit of TDD in Rust, and to work with traits (debug, ops, etc). This should work well enough but this was not meant for production.

All fractions are `Fraction<u64>` for now and all fractions evaluate to `f64` type.

I might turn this into a lib later but I might not.

## Usage

see `main.rs` or tests for usage:

```rust
// main.rs
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
    println!("{}", z.evaluate()); // 0.04

    let f1 = fraction(2, 3);
    let n = 1;
    let r = f1 + n;
    println!("{r:?}"); // 5/3
}
```

## Todo

- adds substraction and division ops traits
- implements macro ( ? ) :

```rust
let f1 = frac!(1/7);
```
