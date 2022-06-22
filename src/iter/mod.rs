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
#[cfg(feature = "simd")]
mod simd_windows;

pub use generic::*;
pub use windows::*;
#[cfg(feature = "simd")]
pub use simd::*;
#[cfg(feature = "simd")]
pub use simd_windows::*;
