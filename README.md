# Num-lazy
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

**Num-lazy** helps you write numbers for generic-typed functions, reduce typing, and improve readability!

<p align="center">
    <a href="https://github.com/p-sira/num-lazy/">
        <img src="https://github.com/p-sira/num-lazy/blob/main/images/num-lazy.png?raw=true" alt="num-lazy demo">
    </a>
</p>

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