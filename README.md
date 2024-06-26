# nearly

[![license][license-badge]][license-url]&nbsp;
[![crate][crate-badge]][crate-url]&nbsp;
[![doc][doc-badge]][doc-url]&nbsp;
[![ci][ci-badge]][ci-url]

[license-badge]: https://img.shields.io/github/license/se-mo/nearly?color=blue&style=flat-square
[license-url]: https://github.com/se-mo/nearly/blob/main/LICENSE
[crate-badge]: https://img.shields.io/crates/v/nearly.svg?style=flat-square&logo=rust
[crate-url]: https://crates.io/crates/nearly
[doc-badge]: https://img.shields.io/docsrs/nearly?style=flat-square&logo=docsdotrs
[doc-url]: https://docs.rs/nearly
[ci-badge]: https://img.shields.io/github/actions/workflow/status/se-mo/nearly/ci.yml?label=ci&style=flat-square&logo=github
[ci-url]: https://github.com/se-mo/nearly/actions/workflows/ci.yml

Compare IEEE floating point types by nearly comparisons.

When comparing floating point types, because of their limited precision, they might not be
exactly identical. Consider the following example, where a and b appear to be identical, but
they are not:

```rust
let a: f32 = 1.0 + 1.04 + 1.1;
let b: f32 = 3.14;

assert!(a == b); // <-- PANICS
```

This crate provides macros to perform a comparison with some tolerance.

```rust
use nearly::nearly;
assert!( nearly!(a == b) ); // <-- OK
```

## Usage

The easiest way to use nearly comparisons is by invoking the `nearly!` macro. The macro returns
a boolean whether the comparison is true or false by using the provided tolerance.

The comparison can be:
  - `a == b` for testing whether a is nearly equal to b
  - `a != b` for testing whether a is not nearly equal to b
  - `a < b` for testing whether a is strict less than b but not nearly equal to b
  - `a <= b` for testing whether a is strict less than b or nearly equal to b
  - `a > b` for testing whether a is strict greater than b but not nearly equal to b
  - `a >= b` for testing whether a is strict greater than b or nearly equal to b

The tolerance used can be:
  - `eps` for an absolute epsilon tolerance
  - `ulps` for an ulps based tolerance
  - `tol` for an absolute epsilon and ulps based tolerance
  - `default` for an absolute epsilon and ulps based tolerance using default values

Here are some example calls:

```rust
use nearly::{nearly, Tolerance};

let a: f32 = 1.0 + 1.04 + 1.1;
let b: f32 = 3.14;

// use absolute epsilon tolerance
nearly!(a == b, eps = 0.001);

// use ulps based tolerance
nearly!(a == b, ulps = 5);

// use absolute epsilon and ulps based tolerance
nearly!(a == b, eps = 0.001, ulps = 5);
nearly!(a == b, tol = Tolerance::new(0.001, 5));

// use default absolute epsilon and default ulps based tolerance
nearly!(a == b);
```

There is also an `assert_nearly!` and `debug_assert_nearly!` macro you can use that panic if the
nearly comparison evaluates to false. The signature is the same as for the `nearly!` macro.

```rust
use nearly::{assert_nearly, debug_assert_nearly, Tolerance};

assert_nearly!(a == b, eps = 0.001);
assert_nearly!(a == b, ulps = 5);
assert_nearly!(a == b, eps = 0.001, ulps = 5);
assert_nearly!(a == b, tol = Tolerance::new(0.001, 5));
assert_nearly!(a == b);

debug_assert_nearly!(a == b, eps = 0.001);
debug_assert_nearly!(a == b, ulps = 5);
debug_assert_nearly!(a == b, eps = 0.001, ulps = 5);
debug_assert_nearly!(a == b, tol = Tolerance::new(0.001, 5));
debug_assert_nearly!(a == b);
```

The nearly functionality is also implemented for a variety of other types holding floats like
containers, maps, pointers or tuples. Here is an example of comparing two arrays of floats.

```rust
use nearly::nearly;

let a: [f32; 4] = [1.1, 2.2, 2.2, 4.4];
let b: [f32; 4] = [1.1, 2.2, 3.3, 4.4];

nearly!(a <= b, eps = 0.001, ulps = 5);
```

## Derive the nearly traits

The easiest way to add nearly comparison to your own types is by deriving the nearly traits.
Just derive `NearlyEq` and `NearlyOrd` to get full support on your type.
 
```rust
use nearly::{assert_nearly, NearlyEq, NearlyOrd};

#[derive(Debug, NearlyEq, NearlyOrd)]
struct Point {
    x: f32,
    y: f32,
}

let a = Point { x: 1.23, y: 4.56 };
let b = Point { x: 1.23, y: 4.567 };

assert_nearly!(a == b, eps = 0.01);
assert_nearly!(a <= b, eps = 0.01);
```
 
To use the `assert_nearly!` and `debug_assert_nearly!` macros, your type must also implement
the Debug trait.

You can derive the following traits:
  - `NearlyEqEps`: enables nearly support with absolute epsilon tolerance
  - `NearlyEqUlps`: enables nearly support with ulps based tolerance
  - `NearlyEqTol`: enables nearly support with absolute epsilon and ulps based tolerances
  - `NearlyEq`: enables nearly support with absolute epsilon and ulps based tolerances
    with default values
  - `NearlyOrdEps`: enables nearly ordering support with absolute epsilon tolerance
  - `NearlyOrdUlps`: enables nearly ordering support with ulps based tolerance
  - `NearlyOrdTol`: enables nearly ordering support with absolute epsilon and ulps based tolerances
  - `NearlyOrd`: enables nearly ordering support with absolute epsilon and ulps based tolerances
    with default values
