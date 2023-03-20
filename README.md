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

Compare IEEE floating point primitives by nearly comparisons.

The issue in directly compare floating point primitives is, that they might be identical from a
logical point of view but because they have limited precision, they are not identical bit by bit.

Consider the following example, where a and b should be identical, but they are not:

```rust
let a: f32 = 1.0 + 1.04 + 1.1;
let b: f32 = 3.14;

assert!(a != b);
```

This crate provides functionality to solve this problem and offers traits and macros to compare
the floating point primitive types `f32` and `f64`.

## Usage

The default usage uses first an absolute difference comparison and second a ulps (unit of least
precision) comparison both with default tolerance values.

Implemented for the types `f32` and `f64` are trait functions you can use to do the comparison.

```rust
use nearly::NearlyEq;
assert!(a.nearly_eq(&b));
```

An alternative way to invoke a nearly comparison is by using the macros this crate provides.
There are macros returning the comparison result as a boolean, assert macros that panic if the
comparison evaluates to false and debug assert macros that are only enabled in non optimized builds

```rust
use nearly::nearly_eq;
assert!( nearly_eq!(a, b) );

use nearly::assert_nearly_eq;
assert_nearly_eq!(a, b);

use nearly::debug_assert_nearly_eq;
debug_assert_nearly_eq!(a, b);
```

## Documentation

For information on how to:

- use comparison based on non default tolerance values
- use absolute or ulps difference explicitly
- implement the crate traits to enable nearly comparison for your own types

see the detailed documentation here: https://docs.rs/nearly