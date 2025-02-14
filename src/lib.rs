/*
 * num-lazy is licensed under The 3-Clause BSD, see LICENSE.
 * Copyright 2025 Sira Pornsiriprasert <code@psira.me>
 */

/*! **num-lazy** helps you write numbers for generic-typed functions.
```
use num_lazy::declare_nums;
use num_traits::Float;
// Assign to generic type T.
// Important. Use the braces!
declare_nums!{T}

fn circumference<T: Float>(radius: T) -> T {
    two!() * pi!() * radius
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

/// Declare commonly used num generics in the module or crate root.
///
/// ```
/// use num_lazy::declare_nums;
/// use num_traits::Float;
/// // Assign to generic type T.
/// // Important. Use the braces!
/// declare_nums!{T}
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
/// This macro will populate the module with:
/// - `num!($n)`: equivalent to `$t::from($n).unwrap()`, where `$t` is the generic type identifier you
///    declared, and `$n` is any expression evaluated to a number.
/// - **Literals**
///     - `zero!()` to `ten!()`
///     - `hundred!()`, `thousand!()`, and `million!()`
///     - `half!()`, `third!()`, and `quarter!()`
///     - `tenth!()`, `hundredth!()`, `thousandth!()`, and `millionth!()`
/// - **Constants**
///     - `pi!()`, `pi_2!()`, `pi_3!()`, `frac_1_pi!()`, `frac_2_pi!()`, and `frac_2_sqrt_pi!()`
///     - `tau!()`
///     - `e!()`
///     - `ln_2!()`, `ln_10!()`, `log2_10!()`, `log2_e!()`, `log10_2!()`, and `log10_e!()`
///     - `sqrt_2!()` and `frac_1_sqrt_2!()`
///     - The golden ratio: `phi!()`
/// - **Special Constants**
///     - Infinity: `inf!()` and `neg_inf!()`
///     - `nan!()`
///     - Min/max type representation value: `min_val!()`, `max_val!()`, and `min_positive!()`
///     - Machine epsilon: `epsilon!()`
///     - Negative zero: `neg_zero!()`
/// - **Others**
///     - `_declare`: An internal function for declaring new numbers.
///         - `_declare!{@literal fourty_two, 42.0, "The universe constant `42`"}`
///         - `_declare!{@constant pi, PI, "π = `3.141592653589793`"}`
///         - `_declare!{@special inf, infinity, "Infinity (`∞`)"}`
#[macro_export]
macro_rules! declare_nums {
    ($t: ident) => {
        /// Unwrap the expression into the specified generic type.
        ///
        /// Equivalent to `$t::from($n).unwrap()`, where `$t` is the generic type identifier you
        /// declared, and `$n` is any expression evaluated to a number.
        macro_rules! num {
            ($n: expr) => {
                $t::from($n).unwrap()
            };
        }

        /// Declare new macro for a number. Meant to be used internally in num-lazy.
        ///
        /// `_declare!{@literal fourty_two, 42.0, "The universe constant `42`"}`
        ///
        /// `_declare!{@constant pi, PI, "π = `3.141592653589793`"}`
        ///
        /// `_declare!{@special inf, infinity, "Infinity (`∞`)"}`
        ///
        // It is documented this way to prevent doctest from compiling in the users' test.
        // Using `ignore` will still show that the test is ignored, which can be annoying.
        macro_rules! _declare {
            ($name:ident, $value: expr, $doc: expr) => {
                #[allow(unused_macros)]
                #[doc=$doc]
                macro_rules! $name {
                    () => {
                        $value
                    };
                }
            };
            (@literal $name:ident, $value: expr, $doc: expr) => {
                _declare! {$name, num!($value), $doc}
            };
            (@special $name:ident, $const_fn: ident, $doc: expr) => {
                _declare! {$name, $t::$const_fn(), $doc}
            };
            (@constant $name:ident, $constant: ident, $doc: expr) => {
                _declare! {$name, $t::from(std::f64::consts::$constant).unwrap(), $doc}
            };
        }

        _declare! {@literal zero, 0.0, "`0`"}
        _declare! {@literal one, 1.0, "`1`"}
        _declare! {@literal two, 2.0, "`2`"}
        _declare! {@literal three, 3.0, "`3`"}
        _declare! {@literal four, 4.0, "`4`"}
        _declare! {@literal five, 5.0, "`5`"}
        _declare! {@literal six, 6.0, "`6`"}
        _declare! {@literal seven, 7.0, "`7`"}
        _declare! {@literal eight, 8.0, "`8`"}
        _declare! {@literal nine, 9.0, "`9`"}
        _declare! {@literal ten, 10.0, "`10`"}
        _declare! {@literal hundred, 100.0, "`100`"}
        _declare! {@literal thousand, 1e3, "`1e3`"}
        _declare! {@literal million, 1e6, "`1e6`"}
        _declare! {@literal half, 0.5, "`0.5`"}
        _declare! {@literal third, 1.0/3.0, "`1/3`"}
        _declare! {@literal quarter, 0.25, "`0.25`"}
        _declare! {@literal tenth, 0.1, "`0.1`"}
        _declare! {@literal hundredth, 0.01, "`0.01`"}
        _declare! {@literal thousandth, 1e-3, "`1e-3`"}
        _declare! {@literal millionth, 1e-6, "`1e-6`"}

        _declare! {@constant pi, PI, "π = `3.141592653589793`"}
        _declare! {@constant pi_2, FRAC_PI_2, "π/2 = `1.5707963267948966`"}
        _declare! {@constant pi_3, FRAC_PI_3, "π/3 = `1.0471975511965979`"}
        _declare! {@constant frac_1_pi, FRAC_1_PI, "1/π = `0.3183098861837907`"}
        _declare! {@constant frac_2_pi, FRAC_2_PI, "2/π = `0.6366197723675814`"}
        _declare! {@constant frac_2_sqrt_pi, FRAC_2_SQRT_PI, "2/sqrt(π) = `1.1283791670955126`"}
        _declare! {@constant tau, TAU, "τ = 2π = `6.283185307179586`"}
        _declare! {@constant e, E, "Euler's number (e) = `2.718281828459045`"}
        _declare! {@constant ln_2, LN_2, "ln(2) = `0.6931471805599453`"}
        _declare! {@constant ln_10, LN_10, "ln(10) = `2.302585092994046`"}
        _declare! {@constant log2_10, LOG2_10, "log₂(10) = `3.321928094887362`"}
        _declare! {@constant log2_e, LOG2_E, "log₂(e) = `1.4426950408889634`"}
        _declare! {@constant log10_2, LOG10_2, "log₁₀(2) = `0.3010299956639812`"}
        _declare! {@constant log10_e, LOG10_E, "log₁₀(e) = `0.4342944819032518`"}
        _declare! {@constant sqrt_2, SQRT_2, "sqrt(2) = `1.4142135623730951`"}
        _declare! {@constant frac_1_sqrt_2, FRAC_1_SQRT_2, "1/sqrt(2) = `0.7071067811865476`"}
        _declare! {@constant phi, PHI, "The golden ratio (φ) = `1.618033988749895`"}

        _declare! {@special inf, infinity, "Infinity (`∞`)"}
        _declare! {@special neg_inf, neg_infinity, "Negative infinity (`-∞`)"}
        _declare! {@special nan, nan, "`NaN`"}
        _declare! {@special min_val, min_value, "The smallest finite value that this type can represent.\n- f32: `-3.4028235e38`\n- f64: `-1.7976931348623157e308`"}
        _declare! {@special max_val, max_value, "The largest finite value that this type can represent.\n- f32: `3.4028235e38`\n- f64: `1.7976931348623157e308`"}
        _declare! {@special min_positive, min_positive_value, "The smallest positive value that this type can represent.\n- f32: `1.1754944e-38`\n- f64: `2.2250738585072014e-308`"}
        _declare! {@special epsilon, epsilon, "`Machine epsilon` value for this type. This is the difference between `1.0` and the next larger representable number.\n- f32: `1.1920929e-7`\n- f64: `2.220446049250313e-16`"}
        _declare! {@special neg_zero, neg_zero, "`-0.0`"}
    };
}
