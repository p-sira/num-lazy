/*
 * num-lazy is licensed under The 3-Clause BSD, see LICENSE.
 * Copyright 2025 Sira Pornsiriprasert <code@psira.me>
 */

use num_lazy::declare_nums;
use num_traits::Float;

declare_nums! {F}

#[test]
fn test_alternative_name() {
    fn float_function<F: Float>() {
        assert!(zero!() == F::zero());
        assert!(one!() == F::one());
        assert!(two!() == F::from(2.0).unwrap())
    }

    float_function::<f64>();
    float_function::<f32>();
}
