#![cfg(feature = "std")]

use crate::nearly_eq::{NearlyEq, NearlyEqEps, NearlyEqTol, NearlyEqUlps};
use crate::nearly_ord::{NearlyOrd, NearlyOrdEps, NearlyOrdTol, NearlyOrdUlps};
use crate::tolerance::{
    EpsTolerance, EpsToleranceType, Tolerance, UlpsTolerance, UlpsToleranceType,
};

use std::collections::{BTreeMap, HashMap};
use std::hash::{BuildHasher, Hash};

///////////////
// nearly_eq //
///////////////

impl<K, Lhs, Rhs, S> NearlyEqEps<HashMap<K, Rhs, S>, Lhs, Rhs> for HashMap<K, Lhs, S>
where
    K: Eq + Hash,
    Lhs: NearlyEqEps<Rhs> + EpsTolerance<Rhs>,
    S: BuildHasher,
{
    fn nearly_eq_eps(&self, other: &HashMap<K, Rhs, S>, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, v_lhs)| {
                other
                    .get(key)
                    .map_or(false, |v_rhs| NearlyEqEps::nearly_eq_eps(v_lhs, v_rhs, eps))
            })
    }
}

impl<K, Lhs, Rhs, S> NearlyEqUlps<HashMap<K, Rhs, S>, Lhs, Rhs> for HashMap<K, Lhs, S>
where
    K: Eq + Hash,
    Lhs: NearlyEqUlps<Rhs> + UlpsTolerance<Rhs>,
    S: BuildHasher,
{
    fn nearly_eq_ulps(
        &self,
        other: &HashMap<K, Rhs, S>,
        ulps: &UlpsToleranceType<Lhs, Rhs>,
    ) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, v_lhs)| {
                other.get(key).map_or(false, |v_rhs| {
                    NearlyEqUlps::nearly_eq_ulps(v_lhs, v_rhs, ulps)
                })
            })
    }
}

impl<K, Lhs, Rhs, S> NearlyEqTol<HashMap<K, Rhs, S>, Lhs, Rhs> for HashMap<K, Lhs, S>
where
    K: Eq + Hash,
    Lhs: NearlyEqTol<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
    S: BuildHasher,
{
    fn nearly_eq_tol(&self, other: &HashMap<K, Rhs, S>, tol: &Tolerance<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, v_lhs)| {
                other
                    .get(key)
                    .map_or(false, |v_rhs| NearlyEqTol::nearly_eq_tol(v_lhs, v_rhs, tol))
            })
    }
}

impl<K, Lhs, Rhs, S> NearlyEq<HashMap<K, Rhs, S>, Lhs, Rhs> for HashMap<K, Lhs, S>
where
    K: Eq + Hash,
    Lhs: NearlyEq<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
    S: BuildHasher,
{
}

impl<K, Lhs, Rhs> NearlyEqEps<BTreeMap<K, Rhs>, Lhs, Rhs> for BTreeMap<K, Lhs>
where
    K: PartialEq,
    Lhs: NearlyEqEps<Rhs> + EpsTolerance<Rhs>,
{
    fn nearly_eq_eps(&self, other: &BTreeMap<K, Rhs>, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other)
                .all(|(a, b)| a.0 == b.0 && NearlyEqEps::nearly_eq_eps(a.1, b.1, eps))
    }
}

impl<K, Lhs, Rhs> NearlyEqUlps<BTreeMap<K, Rhs>, Lhs, Rhs> for BTreeMap<K, Lhs>
where
    K: PartialEq,
    Lhs: NearlyEqUlps<Rhs> + UlpsTolerance<Rhs>,
{
    fn nearly_eq_ulps(&self, other: &BTreeMap<K, Rhs>, ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other)
                .all(|(a, b)| a.0 == b.0 && NearlyEqUlps::nearly_eq_ulps(a.1, b.1, ulps))
    }
}

impl<K, Lhs, Rhs> NearlyEqTol<BTreeMap<K, Rhs>, Lhs, Rhs> for BTreeMap<K, Lhs>
where
    K: PartialEq,
    Lhs: NearlyEqTol<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
{
    fn nearly_eq_tol(&self, other: &BTreeMap<K, Rhs>, tol: &Tolerance<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other)
                .all(|(a, b)| a.0 == b.0 && NearlyEqTol::nearly_eq_tol(a.1, b.1, tol))
    }
}

impl<K, Lhs, Rhs> NearlyEq<BTreeMap<K, Rhs>, Lhs, Rhs> for BTreeMap<K, Lhs>
where
    K: PartialEq,
    Lhs: NearlyEq<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
{
}

////////////////
// nearly_ord //
////////////////

impl<K, Lhs, Rhs, S> NearlyOrdEps<HashMap<K, Rhs, S>, Lhs, Rhs> for HashMap<K, Lhs, S>
where
    K: Eq + Hash,
    Lhs: NearlyOrdEps<Rhs> + EpsTolerance<Rhs>,
    S: BuildHasher,
{
    fn nearly_lt_eps(&self, other: &HashMap<K, Rhs, S>, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, v_lhs)| {
                other.get(key).map_or(false, |v_rhs| {
                    NearlyOrdEps::nearly_lt_eps(v_lhs, v_rhs, eps)
                })
            })
    }

    fn nearly_le_eps(&self, other: &HashMap<K, Rhs, S>, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, v_lhs)| {
                other.get(key).map_or(false, |v_rhs| {
                    NearlyOrdEps::nearly_le_eps(v_lhs, v_rhs, eps)
                })
            })
    }

    fn nearly_gt_eps(&self, other: &HashMap<K, Rhs, S>, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, v_lhs)| {
                other.get(key).map_or(false, |v_rhs| {
                    NearlyOrdEps::nearly_gt_eps(v_lhs, v_rhs, eps)
                })
            })
    }

    fn nearly_ge_eps(&self, other: &HashMap<K, Rhs, S>, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, v_lhs)| {
                other.get(key).map_or(false, |v_rhs| {
                    NearlyOrdEps::nearly_ge_eps(v_lhs, v_rhs, eps)
                })
            })
    }
}

impl<K, Lhs, Rhs, S> NearlyOrdUlps<HashMap<K, Rhs, S>, Lhs, Rhs> for HashMap<K, Lhs, S>
where
    K: Eq + Hash,
    Lhs: NearlyOrdUlps<Rhs> + UlpsTolerance<Rhs>,
    S: BuildHasher,
{
    fn nearly_lt_ulps(
        &self,
        other: &HashMap<K, Rhs, S>,
        ulps: &UlpsToleranceType<Lhs, Rhs>,
    ) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, v_lhs)| {
                other.get(key).map_or(false, |v_rhs| {
                    NearlyOrdUlps::nearly_lt_ulps(v_lhs, v_rhs, ulps)
                })
            })
    }

    fn nearly_le_ulps(
        &self,
        other: &HashMap<K, Rhs, S>,
        ulps: &UlpsToleranceType<Lhs, Rhs>,
    ) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, v_lhs)| {
                other.get(key).map_or(false, |v_rhs| {
                    NearlyOrdUlps::nearly_le_ulps(v_lhs, v_rhs, ulps)
                })
            })
    }

    fn nearly_gt_ulps(
        &self,
        other: &HashMap<K, Rhs, S>,
        ulps: &UlpsToleranceType<Lhs, Rhs>,
    ) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, v_lhs)| {
                other.get(key).map_or(false, |v_rhs| {
                    NearlyOrdUlps::nearly_gt_ulps(v_lhs, v_rhs, ulps)
                })
            })
    }

    fn nearly_ge_ulps(
        &self,
        other: &HashMap<K, Rhs, S>,
        ulps: &UlpsToleranceType<Lhs, Rhs>,
    ) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, v_lhs)| {
                other.get(key).map_or(false, |v_rhs| {
                    NearlyOrdUlps::nearly_ge_ulps(v_lhs, v_rhs, ulps)
                })
            })
    }
}

impl<K, Lhs, Rhs, S> NearlyOrdTol<HashMap<K, Rhs, S>, Lhs, Rhs> for HashMap<K, Lhs, S>
where
    K: Eq + Hash,
    Lhs: NearlyOrdTol<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
    S: BuildHasher,
{
    fn nearly_lt_tol(&self, other: &HashMap<K, Rhs, S>, tol: &Tolerance<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, v_lhs)| {
                other.get(key).map_or(false, |v_rhs| {
                    NearlyOrdTol::nearly_lt_tol(v_lhs, v_rhs, tol)
                })
            })
    }

    fn nearly_le_tol(&self, other: &HashMap<K, Rhs, S>, tol: &Tolerance<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, v_lhs)| {
                other.get(key).map_or(false, |v_rhs| {
                    NearlyOrdTol::nearly_le_tol(v_lhs, v_rhs, tol)
                })
            })
    }

    fn nearly_gt_tol(&self, other: &HashMap<K, Rhs, S>, tol: &Tolerance<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, v_lhs)| {
                other.get(key).map_or(false, |v_rhs| {
                    NearlyOrdTol::nearly_gt_tol(v_lhs, v_rhs, tol)
                })
            })
    }

    fn nearly_ge_tol(&self, other: &HashMap<K, Rhs, S>, tol: &Tolerance<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self.iter().all(|(key, v_lhs)| {
                other.get(key).map_or(false, |v_rhs| {
                    NearlyOrdTol::nearly_ge_tol(v_lhs, v_rhs, tol)
                })
            })
    }
}

impl<K, Lhs, Rhs, S> NearlyOrd<HashMap<K, Rhs, S>, Lhs, Rhs> for HashMap<K, Lhs, S>
where
    K: Eq + Hash,
    Lhs: NearlyOrd<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
    S: BuildHasher,
{
}

impl<K, Lhs, Rhs> NearlyOrdEps<BTreeMap<K, Rhs>, Lhs, Rhs> for BTreeMap<K, Lhs>
where
    K: PartialEq,
    Lhs: NearlyOrdEps<Rhs> + EpsTolerance<Rhs>,
{
    fn nearly_lt_eps(&self, other: &BTreeMap<K, Rhs>, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other)
                .all(|(a, b)| a.0 == b.0 && NearlyOrdEps::nearly_lt_eps(a.1, b.1, eps))
    }

    fn nearly_le_eps(&self, other: &BTreeMap<K, Rhs>, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other)
                .all(|(a, b)| a.0 == b.0 && NearlyOrdEps::nearly_le_eps(a.1, b.1, eps))
    }

    fn nearly_gt_eps(&self, other: &BTreeMap<K, Rhs>, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other)
                .all(|(a, b)| a.0 == b.0 && NearlyOrdEps::nearly_gt_eps(a.1, b.1, eps))
    }

    fn nearly_ge_eps(&self, other: &BTreeMap<K, Rhs>, eps: &EpsToleranceType<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other)
                .all(|(a, b)| a.0 == b.0 && NearlyOrdEps::nearly_ge_eps(a.1, b.1, eps))
    }
}

impl<K, Lhs, Rhs> NearlyOrdUlps<BTreeMap<K, Rhs>, Lhs, Rhs> for BTreeMap<K, Lhs>
where
    K: PartialEq,
    Lhs: NearlyOrdUlps<Rhs> + UlpsTolerance<Rhs>,
{
    fn nearly_lt_ulps(&self, other: &BTreeMap<K, Rhs>, ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other)
                .all(|(a, b)| a.0 == b.0 && NearlyOrdUlps::nearly_lt_ulps(a.1, b.1, ulps))
    }

    fn nearly_le_ulps(&self, other: &BTreeMap<K, Rhs>, ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other)
                .all(|(a, b)| a.0 == b.0 && NearlyOrdUlps::nearly_le_ulps(a.1, b.1, ulps))
    }

    fn nearly_gt_ulps(&self, other: &BTreeMap<K, Rhs>, ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other)
                .all(|(a, b)| a.0 == b.0 && NearlyOrdUlps::nearly_gt_ulps(a.1, b.1, ulps))
    }

    fn nearly_ge_ulps(&self, other: &BTreeMap<K, Rhs>, ulps: &UlpsToleranceType<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other)
                .all(|(a, b)| a.0 == b.0 && NearlyOrdUlps::nearly_ge_ulps(a.1, b.1, ulps))
    }
}

impl<K, Lhs, Rhs> NearlyOrdTol<BTreeMap<K, Rhs>, Lhs, Rhs> for BTreeMap<K, Lhs>
where
    K: PartialEq,
    Lhs: NearlyOrdTol<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
{
    fn nearly_lt_tol(&self, other: &BTreeMap<K, Rhs>, tol: &Tolerance<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other)
                .all(|(a, b)| a.0 == b.0 && NearlyOrdTol::nearly_lt_tol(a.1, b.1, tol))
    }

    fn nearly_le_tol(&self, other: &BTreeMap<K, Rhs>, tol: &Tolerance<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other)
                .all(|(a, b)| a.0 == b.0 && NearlyOrdTol::nearly_le_tol(a.1, b.1, tol))
    }

    fn nearly_gt_tol(&self, other: &BTreeMap<K, Rhs>, tol: &Tolerance<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other)
                .all(|(a, b)| a.0 == b.0 && NearlyOrdTol::nearly_gt_tol(a.1, b.1, tol))
    }

    fn nearly_ge_tol(&self, other: &BTreeMap<K, Rhs>, tol: &Tolerance<Lhs, Rhs>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other)
                .all(|(a, b)| a.0 == b.0 && NearlyOrdTol::nearly_ge_tol(a.1, b.1, tol))
    }
}

impl<K, Lhs, Rhs> NearlyOrd<BTreeMap<K, Rhs>, Lhs, Rhs> for BTreeMap<K, Lhs>
where
    K: PartialEq,
    Lhs: NearlyOrd<Rhs> + EpsTolerance<Rhs> + UlpsTolerance<Rhs>,
{
}
