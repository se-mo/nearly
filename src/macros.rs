use paste::paste;

macro_rules! add_nearly_macro {
    ($operator: ident, $doc_value_1: expr, $doc_value_2: expr, $doc_string: expr) => {
        paste! {
            #[doc = "Returns whether the first expressions is nearly " $doc_string]
            /// to the second expression using the provided tolerance.
            ///
            /// # Examples
            /// Comparison can be based on different tolerances:
            /// ```
            #[doc = "# use nearly::{ToleranceF32, nearly_" $operator "};"]
            #[doc = "let a: f32 = " $doc_value_1 ";"]
            #[doc = "let b: f32 = " $doc_value_2 ";"]
            ///
            /// // use epsilon based comparison
            #[doc = "let " $operator ": bool = nearly_" $operator "!(a, b, eps = 0.01);"]
            ///
            /// // use ulps based comparison
            #[doc = "let " $operator ": bool = nearly_" $operator "!(a, b, ulps = 5);"]
            ///
            /// // use epsilon and ulps based comparison
            #[doc = "let " $operator ": bool = nearly_" $operator
            "!(a, b, eps = 0.01, ulps = 5);"]
            #[doc = "let " $operator ": bool = nearly_" $operator
            "!(a, b, tol = ToleranceF32::new(0.01, 5));"]
            ///
            /// // use epsilon and ulps based comparison with default tolerance
            #[doc = "let " $operator ": bool = nearly_" $operator "!(a, b);"]
            /// ```
            #[macro_export]
            macro_rules! [<nearly_$operator>] {
                ($left: expr, $right: expr, eps = $eps:expr) => {
                    match (&$left, &$right, &$eps) {
                        (left_val, right_val, eps_val) => {
                            use nearly::NearlyEqEps;
                            left_val.[<nearly_ $operator _eps>](right_val, *eps_val)
                        }
                    }
                };
                ($left: expr, $right: expr, ulps = $ulps:expr) => {
                    match (&$left, &$right, &$ulps) {
                        (left_val, right_val, ulps_val) => {
                            use nearly::NearlyEqUlps;
                            left_val.[<nearly_ $operator _ulps>](right_val, *ulps_val)
                        }
                    }
                };
                ($left: expr, $right: expr, tol = $tolerance:expr) => {
                    match (&$left, &$right, &$tolerance) {
                        (left_val, right_val, tolerance_val) => {
                            use nearly::NearlyEqTol;
                            left_val.[<nearly_ $operator _tol>](right_val, *tolerance_val)
                        }
                    }
                };
                ($left: expr, $right: expr, eps = $eps: expr, ulps = $ulps: expr) => {
                    [<nearly_ $operator>]!($left, $right, tol = ($eps, $ulps).into());
                };
                ($left: expr, $right: expr) => {
                    match (&$left, &$right) {
                        (left_val, right_val) => {
                            use nearly::NearlyEq;
                            left_val.[<nearly_$operator>](right_val)
                        }
                    }
                };
            }
        }
    };
}

macro_rules! add_assert_nearly_macro {
    ($operator: ident, $doc_value_1: expr, $doc_value_2: expr, $doc_string: expr) => {
        paste::item! {
            #[doc = "Asserts that the first expressions is nearly " $doc_string]
            /// to the second expression using the provided tolerance.
            ///
            /// On panic, this macro will print the values of the expressions with their debug
            /// representations as well as the values of the provided tolerance.
            ///
            /// # Examples
            /// Comparison can be based on different tolerances:
            /// ```
            #[doc = "# use nearly::{ToleranceF32, assert_nearly_" $operator "};"]
            #[doc = "let a: f32 = " $doc_value_1 ";"]
            #[doc = "let b: f32 = " $doc_value_2 ";"]
            ///
            /// // use epsilon based comparison
            #[doc = "assert_nearly_" $operator "!(a, b, eps = 0.01);"]
            ///
            /// // use ulps based comparison
            #[doc = "assert_nearly_" $operator "!(a, b, ulps = 5);"]
            ///
            /// // use epsilon and ulps based comparison
            #[doc = "assert_nearly_" $operator "!(a, b, eps = 0.01, ulps = 5);"]
            #[doc = "assert_nearly_" $operator "!(a, b, tol = ToleranceF32::new(0.01, 5));"]
            ///
            /// // use epsilon and ulps based comparison with default tolerance
            #[doc = "assert_nearly_" $operator "!(a, b);"]
            /// ```
            #[macro_export]
            macro_rules! [<assert_nearly_$operator>] {
                ($left: expr, $right: expr, eps = $eps:expr) => {
                    match (&$left, &$right, &$eps) {
                        (left_val, right_val, eps_val) => {
                            use nearly::NearlyEqEps;
                            if !left_val.[<nearly_ $operator _eps>](right_val, *eps_val) {
                                panic!(
                                    r#"assertion failed: `(left nearly_{}_eps right)`
  left: `{:?}`,
 right: `{:?}`,
   eps: `{:?}`"#,
                                    stringify!($operator), left_val, right_val, eps_val
                                )
                            }
                        }
                    }
                };
                ($left: expr, $right: expr, ulps = $ulps: expr) => {
                    match (&$left, &$right, &$ulps) {
                        (left_val, right_val, ulps_val) => {
                            use nearly::NearlyEqUlps;
                            if !left_val.[<nearly_ $operator _ulps>](right_val, *ulps_val) {
                                panic!(
                                    r#"assertion failed: `(left nearly_{}_ulps right)`
  left: `{:?}`,
 right: `{:?}`,
  ulps: `{:?}`"#,
                                    stringify!($operator), left_val, right_val, ulps_val
                                )
                            }
                        }
                    }
                };
                ($left: expr, $right: expr, tol = $tolerance: expr) => {
                    match (&$left, &$right, &$tolerance) {
                        (left_val, right_val, tolerance_val) => {
                            use nearly::NearlyEqTol;
                            if !left_val.[<nearly_ $operator _tol>](right_val, *tolerance_val) {
                                panic!(
                                    r#"assertion failed: `(left nearly_{}_tol right)`
  left: `{:?}`,
 right: `{:?}`,
   eps: `{:?}`,
  ulps: `{:?}`"#,
                                    stringify!($operator), left_val, right_val,
                                    tolerance_val.eps, tolerance_val.ulps
                                )
                            }
                        }
                    }
                };
                ($left: expr, $right: expr, eps = $eps: expr, ulps = $ulps: expr) => {
                    [<assert_nearly_ $operator>]!($left, $right, tol = ($eps, $ulps).into());
                };
                ($left: expr, $right: expr) => {
                    match (&$left, &$right) {
                        (left_val, right_val) => {
                            use nearly::NearlyEq;
                            if !left_val.[<nearly_$operator>](right_val) {
                                panic!(
                                    r#"assertion failed: `(left nearly_{} right)`
  left: `{:?}`,
 right: `{:?}`,
   eps: `DEFAULT`,
  ulps: `DEFAULT`"#,
                                    stringify!($operator), left_val, right_val
                                )
                            }
                        }
                    }
                };
            }
        }
    };
}

macro_rules! add_debug_assert_nearly_macro {
    ($operator: ident, $doc_value_1: expr, $doc_value_2: expr, $doc_string: expr) => {
        paste! {
            #[doc = "Asserts that the first expressions is nearly " $doc_string]
            /// to the second expression using the provided tolerance.
            ///
            /// On panic, this macro will print the values of the expressions with their debug
            /// representations as well as the values of the provided tolerance.
            ///
            /// Like [debug_assert!] this macro is only enabled in non optimized builds.
            ///
            /// # Examples
            /// Comparison can be based on different tolerances:
            /// ```
            #[doc = "# use nearly::{ToleranceF32, debug_assert_nearly_" $operator "};"]
            #[doc = "let a: f32 = " $doc_value_1 ";"]
            #[doc = "let b: f32 = " $doc_value_2 ";"]
            ///
            /// // use epsilon based comparison
            #[doc = "debug_assert_nearly_" $operator "!(a, b, eps = 0.01);"]
            ///
            /// // use ulps based comparison
            #[doc = "debug_assert_nearly_" $operator "!(a, b, ulps = 5);"]
            ///
            /// // use epsilon and ulps based comparison
            #[doc = "debug_assert_nearly_" $operator "!(a, b, eps = 0.01, ulps = 5);"]
            #[doc = "debug_assert_nearly_" $operator "!(a, b, tol = ToleranceF32::new(0.01, 5));"]
            ///
            /// // use epsilon and ulps based comparison with default tolerance
            #[doc = "debug_assert_nearly_" $operator "!(a, b);"]
            /// ```
            #[macro_export]
            macro_rules! [<debug_assert_nearly_$operator>] {
                ($left: expr, $right: expr, eps = $eps:expr) => {
                    if cfg!(debug_assertions) {
                        use nearly::[<assert_nearly_ $operator>];
                        [<assert_nearly_ $operator>]!($left, $right, eps = $eps)
                    }
                };
                ($left: expr, $right: expr, ulps = $ulps:expr) => {
                    if cfg!(debug_assertions) {
                        use nearly::[<assert_nearly_ $operator>];
                        [<assert_nearly_ $operator>]!($left, $right, ulps = $ulps)
                    }
                };
                ($left: expr, $right: expr, tol = $tolerance:expr) => {
                    if cfg!(debug_assertions) {
                        use nearly::[<assert_nearly_ $operator>];
                        [<assert_nearly_ $operator>]!($left, $right, tol = $tolerance)
                    }
                };
                ($left: expr, $right: expr, eps = $eps: expr, ulps = $ulps: expr) => {
                    [<debug_assert_nearly_ $operator>]!($left, $right, tol = ($eps, $ulps).into());
                };
                ($left: expr, $right: expr) => {
                    if cfg!(debug_assertions) {
                        use nearly::[<assert_nearly_ $operator>];
                        [<assert_nearly_$operator>]!($left, $right)
                    }
                };
            }
        }
    };
}

macro_rules! add_macros {
    ($operator: ident, $doc_value_1: expr, $doc_value_2: expr, $doc_string: expr) => {
        add_nearly_macro!($operator, $doc_value_1, $doc_value_2, $doc_string);
        add_assert_nearly_macro!($operator, $doc_value_1, $doc_value_2, $doc_string);
        add_debug_assert_nearly_macro!($operator, $doc_value_1, $doc_value_2, $doc_string);
    };
}

add_macros!(eq, 1.0, 1.0, "equal");
add_macros!(ne, 1.0, 2.0, "not equal");
