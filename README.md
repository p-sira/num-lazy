# num-lazy
<p>
    <a href="https://opensource.org/license/BSD-3-clause">
        <img src="https://img.shields.io/badge/License-BSD--3--Clause-brightgreen.svg" alt="License">
    </a>
    <a href="https://crates.io/crates/num-lazy">
        <img src="https://img.shields.io/crates/v/num-lazy" alt="Crate">
    </a>
    <a href="https://docs.rs/num-lazy">
        <img src="https://img.shields.io/badge/Docs-docs.rs-blue" alt="Documentation">
    </a>
</p>

**num-lazy** helps you write numbers for generic-typed functions, reduce typing, and improve readability!

## Why Num-Lazy
Let's write a generic circumference function using `num-trait`.
```rust
fn circumference<T: Float>(radius: T) -> T {
    T::from(2.0).unwrap() * T::from(std::f64::consts::PI).unwrap() * radius
}
```
This doesn't look too bad. But you can imagine it getting out of hand for more complex functions. This is where num-lazy comes to the rescue! Let's implement using `num-lazy`.

```rust
fn circumference<T: Float>(radius: T) -> T {
    two!() * pi!() * radius
}
```

## Quick Start
Install num-lazy by:
```shell
>> cargo add num-lazy
```

Use `declare_nums!{T}` to bind num-lazy to generic type `T`.
```rust
use num_lazy::declare_nums;
use num_traits::Float;
declare_nums!{T}

fn circumference<T: Float>(radius: T) -> T {
    two!() * pi!() * radius
}

fn main() {
    assert!(circumference(1.0_f64) == 6.283185307179586);
}
```