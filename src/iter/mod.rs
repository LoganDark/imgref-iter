//! Contains implementations for all the iterators offered by this crate.

// Iter
// IterMut
// IterPtr
// IterPtrMut
// IterWindows
// IterWindowsMut
// IterWindowsPtr
// IterWindowsPtrMut
// SimdIter
// SimdIterMut
// SimdIterPtr
// SimdIterPtrMut

mod generic;
mod windows;
#[cfg(feature = "simd")]
mod simd;

pub use generic::*;
pub use windows::*;
#[cfg(feature = "simd")]
pub use simd::*;
