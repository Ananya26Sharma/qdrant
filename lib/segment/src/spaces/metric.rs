use crate::data_types::vectors::VectorElementType;
use crate::types::{Distance, ScoreType};

/// Defines how to compare vectors
pub trait Metric {
    fn distance() -> Distance;

    /// Greater the value - closer the vectors
    fn similarity(v1: &[VectorElementType], v2: &[VectorElementType], dim: usize) -> ScoreType;

    /// Necessary vector transformations performed before adding it to the collection (like normalization)
    /// Return None if metric does not required preprocessing
    fn preprocess(vector: &[VectorElementType]) -> Option<Vec<VectorElementType>>;

    /// Return length of vector after preprocessing
    fn preprocessed_len(dim: usize) -> Option<usize>;

    /// Restore original vector after preprocessing
    fn restore(vector: &[VectorElementType], dim: usize) -> Option<Vec<VectorElementType>>;

    /// correct metric score for displaying
    fn postprocess(score: ScoreType) -> ScoreType;
}
