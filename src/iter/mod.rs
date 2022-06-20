//! Contains implementations for all the iterators offered by this crate.

// Iter
// IterMut
// IterPtr
// IterPtrMut
// IterRows
// IterRowsMut
// IterRowsPtr
// IterRowsPtrMut
// IterCols
// IterColsMut
// IterColsPtr
// IterColsPtrMut

mod generic;
mod rows;
mod cols;
#[cfg(feature = "simd")]
mod simd;

pub use generic::*;
pub use rows::*;
pub use cols::*;
#[cfg(feature = "simd")]
pub use simd::*;
