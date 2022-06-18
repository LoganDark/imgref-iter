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
//! Methods on [`ImgIterPtr`] and [`ImgIterPtrMut`] are `unsafe` because they
//! offset on the provided pointers. [`ImgIter`] and [`ImgIterMut`] cannot
//! include safe versions because the pointer iterators may outlive the
//! references.

pub mod traits;
pub mod iter;

#[cfg(doc)]
use traits::*;