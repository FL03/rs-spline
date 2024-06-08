/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub type Result<T = ()> = core::result::Result<T, SplineError>;

#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "PascalCase", tag = "error")
)]
#[strum(serialize_all = "PascalCase")]
pub enum SplineError {
    InvalidKnotVector,
    DegreeMismatch,
    NotEnoughKnots { exp: usize, res: usize },
    NotEnoughPoints,
    Unknown(String),
}

impl SplineError {
    pub fn invalid_knot_vector() -> Self {
        Self::InvalidKnotVector
    }

    pub fn degree_mismatch() -> Self {
        Self::DegreeMismatch
    }

    pub fn not_enough_knots(exp: usize, res: usize) -> Self {
        Self::NotEnoughKnots { exp, res }
    }

    pub fn not_enough_points() -> Self {
        Self::NotEnoughPoints
    }

    pub fn unknown(err: impl ToString) -> Self {
        Self::Unknown(err.to_string())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SplineError {}
