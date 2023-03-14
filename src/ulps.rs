/// A trait for ulps based calculations and operations on floating point types.
pub trait Ulps
where
    Self: Copy,
{
    /// The integer representation of the floating point type implementing this trait.
    type IntRep;

    /// Returns the signed ulps distance between `self` and `other`.
    fn signed_ulps_distance(self, other: Self) -> Self::IntRep;
}

macro_rules! impl_ulps {
    ($float: ty, $int: ty, $uint: ty) => {
        impl Ulps for $float {
            type IntRep = $int;

            /// Returns the signed ulps distance between `self` and `other`.
            /// This corresponds to the ulps representation of `self - other`.
            fn signed_ulps_distance(self, other: Self) -> Self::IntRep {
                let i_self = self.to_bits() as $int;
                let i_other = other.to_bits() as $int;

                i_self.wrapping_sub(i_other)
            }
        }
    };
}

impl_ulps!(f32, i32, u32);
impl_ulps!(f64, i64, u64);

#[cfg(test)]
mod tests {
    use crate::ulps::Ulps;

    #[test]
    fn signed_ulps_distance_same_f32() {
        {
            let a: f32 = 0.0;
            let b: f32 = 0.0;
            assert_eq!(a, b);
            assert_eq!(a.signed_ulps_distance(b), 0);
        }
        {
            let a: f32 = 1e-45;
            let b: f32 = 1e-45;
            assert_eq!(a, b);
            assert_eq!(a.signed_ulps_distance(b), 0);
        }
    }

    #[test]
    fn signed_ulps_distance_different_f32() {
        {
            let a: f32 = 1e-45;
            let b: f32 = 4e-45;
            assert_ne!(a, b);
            assert_eq!(a.signed_ulps_distance(b), -2);
            assert_eq!(b.signed_ulps_distance(a), 2);
        }
        {
            let a: f32 = 1.0;
            let b: f32 = 1.0000008;
            assert_ne!(a, b);
            assert_eq!(a.signed_ulps_distance(b), -7);
            assert_eq!(b.signed_ulps_distance(a), 7);
        }
    }

    #[test]
    fn signed_ulps_distance_different_sign_f32() {
        let a: f32 = 1e-45;
        let b: f32 = -1e-45;
        assert_ne!(a, b);
        assert!(a.signed_ulps_distance(b) < -2);
        assert!(b.signed_ulps_distance(a) < -2);
    }

    #[test]
    fn signed_ulps_distance_same_f64() {
        {
            let a: f64 = 0.0;
            let b: f64 = 0.0;
            assert_eq!(a, b);
            assert_eq!(a.signed_ulps_distance(b), 0);
        }
        {
            let a: f64 = 5e-324;
            let b: f64 = 5e-324;
            assert_eq!(a, b);
            assert_eq!(a.signed_ulps_distance(b), 0);
        }
    }

    #[test]
    fn signed_ulps_distance_different_f64() {
        {
            let a: f64 = 5e-324;
            let b: f64 = 1.5e-323;
            assert_ne!(a, b);
            assert_eq!(a.signed_ulps_distance(b), -2);
            assert_eq!(b.signed_ulps_distance(a), 2);
        }
        {
            let a: f64 = 1.0;
            let b: f64 = 1.0000000000000016;
            assert_ne!(a, b);
            assert_eq!(a.signed_ulps_distance(b), -7);
            assert_eq!(b.signed_ulps_distance(a), 7);
        }
    }

    #[test]
    fn signed_ulps_distance_different_sign_f64() {
        let a: f64 = 5e-324;
        let b: f64 = -5e-324;
        assert_ne!(a, b);
        assert!(a.signed_ulps_distance(b) < -2);
        assert!(b.signed_ulps_distance(a) < -2);
    }
}
