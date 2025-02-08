/*
 * num-lazy is licensed under The 3-Clause BSD, see LICENSE.
 * Copyright 2025 Sira Pornsiriprasert <code@psira.me>
 */

use std::fmt::Display;

use num_lazy::declare_nums;
use num_traits::{Float, PrimInt};
declare_nums!{T}

#[test]
fn test_num() {
    fn float_function<T: Float>() {
        assert!(num!(42.42) == T::from(42.42).unwrap());
    }

    fn int_function<T: PrimInt>() {
        assert!(num!(5) == T::from(5).unwrap());
    }

    float_function::<f64>();
    float_function::<f32>();
    
    int_function::<i32>();
    int_function::<i64>();
}

#[test]
fn test_consts() {
    fn float_function<T: Float + Display>() {
        assert!(zero!() == T::zero());
        assert!(one!() == T::one());
        assert!(two!() == T::from(2.0).unwrap());
        assert!(half!() * pi!() == pi_2!());
        assert!(e!() == T::from(std::f64::consts::E).unwrap());
    }
    float_function::<f64>();
    float_function::<f32>();
}
