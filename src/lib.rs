//! A small crate for iterating over the rows or columns of `imgref` buffers.
//!
//! This crate exports four traits that allow creating iterators over rows or
//! columns of [`Img`][imgref::Img]s:
//!
//! - [`ImgIterPtr`] for `Img<*const [T]>`; allows creating iterators over
//!   `*const` pointers
//!
//! - [`ImgIterPtrMut`] for `Img<*mut [T]>`; allows creating iterators over
//!   `*mut` pointers
//!
//! - [`ImgIter`] for `Img<&[T]>`; allows creating iterators over shared
//!   references
//!
//! - [`ImgIterMut`] for `Img<&mut [T]>`; allows creating iterators over mutable
//!   references
//!
//! As well as two utility traits for converting to `Img<*const [T]>` or
//! `Img<*mut [T]>`:
//!
//! - [`ImgAsPtr`] for conversions to `Img<*const [T]>`.
//!
//! - [`ImgAsMutPtr`] for conversions to `Img<*mut [T]>`.
//!
//!   This is actually not implemented by anything other than `Img<*mut [T]>`,
//!   but it exists for the purpose of documenting why it cannot be implemented
//!   for `Img<&mut [T]>`.
//!
//! Additionally, when the (experimental!) `simd` feature is enabled, there are
//! four more traits - [`ImgSimdIter`], [`ImgSimdIterMut`], [`ImgSimdIterPtr`],
//! and [`ImgSimdIterPtrMut`]. These allow creating iterators over *multiple*
//! rows or columns of an image at once. They don't actually depend on SIMD or a
//! nightly compiler - they just return multiple items at once.
//!
//! Methods on [`ImgIterPtr`] and [`ImgIterPtrMut`] are `unsafe` because they
//! offset on the provided pointers. [`ImgIter`] and [`ImgIterMut`] cannot
//! include safe versions because the pointer iterators may outlive the
//! references.

pub mod traits;
pub mod iter;

#[cfg(doc)]
use traits::*;
