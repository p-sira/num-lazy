/*
 * num-lazy is licensed under The 3-Clause BSD, see LICENSE.
 * Copyright 2025 Sira Pornsiriprasert <code@psira.me>
 */

/*! **num-lazy** helps you write numbers for generic-typed functions.

It is recommended to use `num-lazy` along with [numeric-literals](https://crates.io/crates/numeric_literals).
Use num-lazy to access macros for constants and special values, while using `numeric_literals` for parsing
floats or numeric literals.
```
use num_lazy::declare_nums;
use numeric_literals::replace_numeric_literals;
use num_traits::Float;

declare_nums!{@constant T}
declare_nums!{@special T}
// declare_nums!{@literal T} // Use `numeric_literals` instead

// Or use this to declare all macros:
// declare_nums!{T}

#[replace_numeric_literals(T::from(literal).unwrap())]
fn circumference<T: Float>(radius: T) -> T {
    2 * pi!() * radius
}
#
# fn main() {
#     assert!(circumference(1.0_f64) == 6.283185307179586);
# }
```

See what numbers are declared in [declare_nums].

If you `declare_nums!()` in the root of your crate, you don't even need
to import the macros to submodules. This will not re-export the macros
to the public crate.

lib.rs or main.rs
```ignore
use num_lazy::declare_nums;
declare_nums!{T}

// My submodules
pub mod generic_math;
```

generic_math.rs
```ignore
pub fn circle_area<T: Float>(radius: T) -> T {
    pi!() * radius * radius
}
```

If you want to declare the numbers in the root of your crate but keep
the macro contained in a module, you can simply:

lib.rs or main.rs
```ignore
mod generics {
    use num_lazy::declare_nums;
    declare_nums!{T}
}
```

generic_math.rs
```ignore
use crate::generics::*;

pub fn circle_area<T: Float>(radius: T) -> T {
    pi!() * radius * radius
}
```
*/

/// Declare commonly used num generics.
///
/// ```
/// use num_lazy::declare_nums;
/// use num_traits::Float;
/// // Assign to generic type T.
/// // Important. Use the braces!
/// declare_nums!{T}
/// // Or declare as needed
/// // declare_nums!{@literal T}
/// // declare_nums!{@constant T}
/// // declare_nums!{@special T}
///
/// fn add_tiny<T: Float>(a: T) -> T {
///     let tiny = five!() * epsilon!();
///     a + tiny
/// }
///
/// fn main() {
///     assert!(add_tiny(1.0_f64) == 1.000000000000001);
///     assert!(add_tiny(1.0_f32) == 1.0000006);
/// }
/// ```
///
/// Using `declare_nums!{T}` will populate the module with all available macros:
/// - `num!($n)`: equivalent to `$t::from($n).unwrap()`, where `$t` is the generic type identifier you
///   declared, and `$n` is any expression evaluated to a number.
/// - Literals as in `declare_nums!{@literal T}`.
/// - Constants as in `declare_nums!{@constant T}`.
/// - Special as in `declare_nums!{@special T}`.
///
/// Each match arm will populate the module with:
/// - **Literals:** `declare_nums!{@literal T}`
///     - `zero!()` to `ten!()`
///     - `hundred!()`, `thousand!()`, and `million!()`
///     - `half!()`, `third!()`, and `quarter!()`
///     - `tenth!()`, `hundredth!()`, `thousandth!()`, and `millionth!()`
/// - **Constants:** `declare_nums!{@constant T}`
///     - `pi!()`, `pi_2!()`, `pi_3!()`, `frac_1_pi!()`, `frac_2_pi!()`, and `frac_2_sqrt_pi!()`
///     - `tau!()`
///     - `e!()`
///     - `ln_2!()`, `ln_10!()`, `log2_10!()`, `log2_e!()`, `log10_2!()`, and `log10_e!()`
///     - `sqrt_2!()` and `frac_1_sqrt_2!()`
///     - The golden ratio: `phi!()`
/// - **Special Constants:** `declare_nums!{@special T}`
///     - Infinity: `inf!()` and `neg_inf!()`
///     - `nan!()`
///     - Min/max type representation value: `min_val!()`, `max_val!()`, and `min_positive!()`
///     - Machine epsilon: `epsilon!()`
///     - Negative zero: `neg_zero!()`
#[macro_export]
macro_rules! declare_nums {
    {$t: ident} => {
        /// Unwrap the expression into the specified generic type.
        ///
        /// Equivalent to `$t::from($n).unwrap()`, where `$t` is the generic type identifier you
        /// declared, and `$n` is any expression evaluated to a number.
        #[allow(unused_macros)]
        macro_rules! num {
            ($n: expr) => {
                $t::from($n).unwrap()
            };
        }

        declare_nums!{@literal $t}
        declare_nums!{@constant $t}
        declare_nums!{@special $t}
    };
    {@literal $t:ident} => {
        macro_rules! _declare_literal {
            ($name:ident, $n: expr, $doc: expr) => {
                #[allow(unused_macros)]
                #[doc=$doc]
                macro_rules! $name {
                    () => {
                        $t::from($n).unwrap()
                    };
                }
            };
        }

        _declare_literal! { zero, 0.0, "`0`"}
        _declare_literal! { one, 1.0, "`1`"}
        _declare_literal! { two, 2.0, "`2`"}
        _declare_literal! { three, 3.0, "`3`"}
        _declare_literal! { four, 4.0, "`4`"}
        _declare_literal! { five, 5.0, "`5`"}
        _declare_literal! { six, 6.0, "`6`"}
        _declare_literal! { seven, 7.0, "`7`"}
        _declare_literal! { eight, 8.0, "`8`"}
        _declare_literal! { nine, 9.0, "`9`"}
        _declare_literal! { ten, 10.0, "`10`"}
        _declare_literal! { hundred, 100.0, "`100`"}
        _declare_literal! { thousand, 1e3, "`1e3`"}
        _declare_literal! { million, 1e6, "`1e6`"}
        _declare_literal! { half, 0.5, "`0.5`"}
        _declare_literal! { third, 1.0/3.0, "`1/3`"}
        _declare_literal! { quarter, 0.25, "`0.25`"}
        _declare_literal! { tenth, 0.1, "`0.1`"}
        _declare_literal! { hundredth, 0.01, "`0.01`"}
        _declare_literal! { thousandth, 1e-3, "`1e-3`"}
        _declare_literal! { millionth, 1e-6, "`1e-6`"}
    };
    (@constant $t:ident) => {
        macro_rules! _declare_constant {
            ($name:ident, $constant:ident, $doc:expr) => {
                #[allow(unused_macros)]
                #[doc=$doc]
                macro_rules! $name {
                    () => {
                        $t::from(std::f64::consts::$constant).unwrap()
                    };
                }
            };
        }
        _declare_constant! { pi, PI, "π = `3.141592653589793`"}
        _declare_constant! { pi_2, FRAC_PI_2, "π/2 = `1.5707963267948966`"}
        _declare_constant! { pi_3, FRAC_PI_3, "π/3 = `1.0471975511965979`"}
        _declare_constant! { frac_1_pi, FRAC_1_PI, "1/π = `0.3183098861837907`"}
        _declare_constant! { frac_2_pi, FRAC_2_PI, "2/π = `0.6366197723675814`"}
        _declare_constant! { frac_2_sqrt_pi, FRAC_2_SQRT_PI, "2/sqrt(π) = `1.1283791670955126`"}
        _declare_constant! { tau, TAU, "τ = 2π = `6.283185307179586`"}
        _declare_constant! { e, E, "Euler's number (e) = `2.718281828459045`"}
        _declare_constant! { ln_2, LN_2, "ln(2) = `0.6931471805599453`"}
        _declare_constant! { ln_10, LN_10, "ln(10) = `2.302585092994046`"}
        _declare_constant! { log2_10, LOG2_10, "log₂(10) = `3.321928094887362`"}
        _declare_constant! { log2_e, LOG2_E, "log₂(e) = `1.4426950408889634`"}
        _declare_constant! { log10_2, LOG10_2, "log₁₀(2) = `0.3010299956639812`"}
        _declare_constant! { log10_e, LOG10_E, "log₁₀(e) = `0.4342944819032518`"}
        _declare_constant! { sqrt_2, SQRT_2, "sqrt(2) = `1.4142135623730951`"}
        _declare_constant! { frac_1_sqrt_2, FRAC_1_SQRT_2, "1/sqrt(2) = `0.7071067811865476`"}
        _declare_constant! { phi, PHI, "The golden ratio (φ) = `1.618033988749895`"}
    };
    (@special $t:ident) => {
        macro_rules! _declare_special {
            ($name:ident, $const_fn:ident, $doc:expr) => {
                #[allow(unused_macros)]
                #[doc=$doc]
                macro_rules! $name {
                    () => {
                        $t::$const_fn()
                    };
                }
            };
        }
        _declare_special! { inf, infinity, "Infinity (`∞`)"}
        _declare_special! { neg_inf, neg_infinity, "Negative infinity (`-∞`)"}
        _declare_special! { nan, nan, "`NaN`"}
        _declare_special! { min_val, min_value, "The smallest finite value that this type can represent.\n- f32: `-3.4028235e38`\n- f64: `-1.7976931348623157e308`"}
        _declare_special! { max_val, max_value, "The largest finite value that this type can represent.\n- f32: `3.4028235e38`\n- f64: `1.7976931348623157e308`"}
        _declare_special! { min_positive, min_positive_value, "The smallest positive value that this type can represent.\n- f32: `1.1754944e-38`\n- f64: `2.2250738585072014e-308`"}
        _declare_special! { epsilon, epsilon, "`Machine epsilon` value for this type. This is the difference between `1.0` and the next larger representable number.\n- f32: `1.1920929e-7`\n- f64: `2.220446049250313e-16`"}
        _declare_special! { neg_zero, neg_zero, "`-0.0`"}
    };
}
