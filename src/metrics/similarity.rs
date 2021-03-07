//! This submodule provides some basic similarity measures
//!
//! It supports sparse vectors from `sprs` which seems to be the most popular
//! library for sparse algebra.

use sprs::CsVec;

/// Type for a similarity function
pub type Similarity = fn(&CsVec<f64>, &CsVec<f64>) -> f64;

/// Cosine similarity between two vectors.
///
/// Returns zero if one of the vectors is zero.
pub fn cosine(a: &CsVec<f64>, b: &CsVec<f64>) -> f64 {
    let norms = a.dot(a) * b.dot(b);
    if norms > 0.0 {
        a.dot(b) / norms.sqrt()
    } else {
        0.0
    }
}
