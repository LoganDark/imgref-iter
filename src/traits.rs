//! Contains the traits that allow obtaining iterators.

use imgref::Img;

use crate::iter::{
	Iter,
	IterMut,
	IterPtr,
	IterPtrMut,
	IterWindows,
	IterWindowsMut,
	IterWindowsPtr,
	IterWindowsPtrMut
};

#[cfg(feature = "simd")]
use crate::iter::{
	SimdIter,
	SimdIterMut,
	SimdIterPtr,
	SimdIterPtrMut,
	SimdIterWindows,
	SimdIterWindowsMut,
	SimdIterWindowsPtr,
	SimdIterWindowsPtrMut,
};

mod sealed {
	pub trait SealedAsPtr {}

	pub trait SealedAsMutPtr {}

	pub trait SealedPtr {}

	pub trait SealedPtrMut {}

	pub trait Sealed {}

	pub trait SealedMut {}

	#[cfg(feature = "simd")]
	pub trait SealedSimdPtr {}

	#[cfg(feature = "simd")]
	pub trait SealedSimdPtrMut {}

	#[cfg(feature = "simd")]
	pub trait SealedSimd {}

	#[cfg(feature = "simd")]
	pub trait SealedSimdMut {}
}

/// The trait for images whose buffers can be converted to a `*const` pointer.
pub trait ImgAsPtr: sealed::SealedAsPtr {
	type Item;

	#[cfg(not(feature = "simd"))]
	type AsPtr: ImgIterPtr<Item = Self::Item>;

	#[cfg(feature = "simd")]
	type AsPtr: ImgIterPtr<Item = Self::Item> + ImgSimdIterPtr;

	/// Returns an [`Img`] that points to this one's buffer.
	fn as_ptr(&self) -> Self::AsPtr;
}

/// The trait for [`Img`]s whose buffers can be converted to a `*mut` pointer
/// *through a shared borrow*. `Img<&mut [T]>` cannot implement this because a
/// mutable reference behind a shared reference becomes immutable - but
/// [`ImgIterMut`] has another [`as_mut_ptr`][ImgIterMut::as_mut_ptr] method.
pub trait ImgAsMutPtr: sealed::SealedAsMutPtr + ImgAsPtr {
	#[cfg(not(feature = "simd"))]
	type AsMutPtr: ImgIterPtrMut<Item = Self::Item>;

	#[cfg(feature = "simd")]
	type AsMutPtr: ImgIterPtrMut<Item = Self::Item> + ImgSimdIterPtrMut;

	/// Returns a [`Img`] that mutably points to this one's buffer.
	fn as_mut_ptr(&self) -> Self::AsMutPtr;
}

/// Exposes iterators that return `*const` pointers.
///
/// Implemented for buffer pointers, i.e. [`Img<*const [T]>`][Img] and
/// [`Img<*mut [T]>`][Img].
pub trait ImgIterPtr: sealed::SealedPtr + ImgAsPtr {
	/// Returns an iterator over pointers to the pixels of the specified row.
	/// row.
	///
	/// # Safety
	///
	/// The caller must ensure that the pointer contained by the [`Img`] is
	/// valid for reads from all pixels of the specified row, and that the
	/// pointer remains valid for the lifetime of the iterator.
	///
	/// # Panics
	///
	/// Panics if the specified row is out of bounds for the [`Img`].
	#[inline]
	unsafe fn iter_row_ptr(&self, row: usize) -> IterPtr<Self::Item> {
		self.as_ptr().iter_row_ptr(row)
	}

	/// Returns an iterator over [`IterPtr`]s.
	///
	/// # Safety
	///
	/// The caller must ensure that the pointer contained by the [`Img`] is
	/// valid for reads from all pixels, and that the pointer remains valid for
	/// the lifetime of the iterator.
	#[inline]
	unsafe fn iter_rows_ptr(&self) -> IterWindowsPtr<Self::Item> {
		self.as_ptr().iter_rows_ptr()
	}

	/// Returns an iterator over pointers to the pixels of the specified column.
	///
	/// # Safety
	///
	/// The caller must ensure that the pointer contained by the [`Img`] is
	/// valid for reads from all pixels of the specified column, and that the
	/// pointer remains valid for the lifetime of the iterator.
	///
	/// # Panics
	///
	/// Panics if the specified column is out of bounds for the [`Img`].
	#[inline]
	unsafe fn iter_col_ptr(&self, col: usize) -> IterPtr<Self::Item> {
		self.as_ptr().iter_col_ptr(col)
	}

	/// Returns an iterator over [`IterPtr`]s.
	///
	/// # Safety
	///
	/// The caller must ensure that the pointer contained by the [`Img`] is
	/// valid for reads from all pixels, and that the pointer remains valid for
	/// the lifetime of the iterator.
	#[inline]
	unsafe fn iter_cols_ptr(&self) -> IterWindowsPtr<Self::Item> {
		self.as_ptr().iter_cols_ptr()
	}
}

/// Exposes iterators that return `*mut` pointers.
///
/// Implemented for `mut` buffer pointers, i.e. [`Img<*mut [T]>`][Img].
pub trait ImgIterPtrMut: sealed::SealedPtrMut + ImgAsMutPtr + ImgIterPtr {
	/// Returns an iterator over `*mut` pointers to the pixels of the specified
	/// row.
	///
	/// # Safety
	///
	/// The caller must ensure that the pointer contained by the [`Img`] is
	/// valid for reads and writes for all pixels of the specified row, and that
	/// the pointer remains valid for the lifetime of the iterator.
	///
	/// # Panics
	///
	/// Panics if the specified row is out of bounds for the [`Img`].
	#[inline]
	unsafe fn iter_row_ptr_mut(&self, row: usize) -> IterPtrMut<Self::Item> {
		self.as_mut_ptr().iter_row_ptr_mut(row)
	}

	/// Returns an iterator over [`IterPtrMut`]s.
	///
	/// # Safety
	///
	/// The caller must ensure that the pointer contained by the [`Img`] is
	/// valid for reads and writes for all pixels, and that the pointer remains
	/// valid for the lifetime of the iterator.
	#[inline]
	unsafe fn iter_rows_ptr_mut(&self) -> IterWindowsPtrMut<Self::Item> {
		self.as_mut_ptr().iter_rows_ptr_mut()
	}

	/// Returns an iterator over `*mut` pointers to the pixels of the specified
	/// column.
	///
	/// # Safety
	///
	/// The caller must ensure that the pointer contained by the [`Img`] is
	/// valid for reads and writes for all pixels of the specified column, and
	/// that the pointer remains valid for the lifetime of the iterator.
	///
	/// # Panics
	///
	/// Panics if the specified column is out of bounds for the [`Img`].
	#[inline]
	unsafe fn iter_col_ptr_mut(&self, col: usize) -> IterPtrMut<Self::Item> {
		self.as_mut_ptr().iter_col_ptr_mut(col)
	}

	/// Returns an iterator over [`IterPtrMut`]s.
	///
	/// # Safety
	///
	/// The caller must ensure that the pointer contained by the [`Img`] is
	/// valid for reads and writes for all pixels, and that the pointer remains
	/// valid for the lifetime of the iterator.
	#[inline]
	unsafe fn iter_cols_ptr_mut(&self) -> IterWindowsPtrMut<Self::Item> {
		self.as_mut_ptr().iter_cols_ptr_mut()
	}
}

/// Exposes iterators that return `&` references.
///
/// Implemented for all ordinary references and owned containers, i.e.
/// [`Img<&[T]>`][Img].
pub trait ImgIter: sealed::Sealed + ImgAsPtr {
	/// Returns an iterator over the pixels of the specified row.
	///
	/// # Panics
	///
	/// Panics if the specified row is out of bounds for the [`Img`].
	fn iter_row(&self, row: usize) -> Iter<Self::Item>;

	/// Returns an iterator over rows.
	fn iter_rows(&self) -> IterWindows<Self::Item>;

	/// Returns an iterator over the pixels of the specified column.
	///
	/// # Panics
	///
	/// Panics if the specified column is out of bounds for the [`Img`].
	fn iter_col(&self, col: usize) -> Iter<Self::Item>;

	/// Returns an iterator over columns.
	fn iter_cols(&self) -> IterWindows<Self::Item>;
}

/// Exposes iterators that return `&mut` references.
///
/// Implemented for all mutable references and owned containers, i.e.
/// [`Img<&mut [T]>`][Img] or [`Img<Vec<T>>`][Img].
pub trait ImgIterMut: sealed::SealedMut + ImgIter {
	type AsMutPtr: ImgIterPtrMut<Item = Self::Item>;

	/// Returns an [`Img`] that mutably points to this one's buffer.
	fn as_mut_ptr(&mut self) -> Self::AsMutPtr;

	/// Returns an iterator over the pixels of the specified row.
	///
	/// # Panics
	///
	/// Panics if the specified row is out of bounds for the [`Img`].
	fn iter_row_mut(&mut self, row: usize) -> IterMut<Self::Item>;

	/// Returns an iterator over [`IterMut`]s.
	fn iter_rows_mut(&mut self) -> IterWindowsMut<Self::Item>;

	/// Returns an iterator over the pixels of the specified column.
	///
	/// # Panics
	///
	/// Panics if the specified column is out of bounds for the [`Img`].
	fn iter_col_mut(&mut self, col: usize) -> IterMut<Self::Item>;

	/// Returns an iterator over [`IterMut`]s.
	fn iter_cols_mut(&mut self) -> IterWindowsMut<Self::Item>;
}

/// Exposes iterators that return arrays of `*const` pointers.
///
/// Implemented for buffer pointers, i.e. [`Img<*const [T]>`][Img] and
/// [`Img<*mut [T]>`][Img].
#[cfg(feature = "simd")]
pub trait ImgSimdIterPtr: sealed::SealedSimdPtr + ImgIterPtr {
	/// Returns an iterator over pointers to the pixels of the specified row.
	/// row.
	///
	/// # Safety
	///
	/// The caller must ensure that the pointer contained by the [`Img`] is
	/// valid for reads from all pixels of the specified row, and that the
	/// pointer remains valid for the lifetime of the iterator.
	///
	/// # Panics
	///
	/// Panics if the specified row is out of bounds for the [`Img`].
	#[inline]
	unsafe fn simd_iter_row_ptr<const LANES: usize>(&self, row: usize) -> SimdIterPtr<Self::Item, LANES> {
		self.as_ptr().simd_iter_row_ptr::<LANES>(row)
	}

	/// Returns an iterator over [`SimdIterWindowPtr`]s.
	///
	/// # Safety
	///
	/// The caller must ensure that the pointer contained by the [`Img`] is
	/// valid for reads from all pixels, and that the pointer remains valid for
	/// the lifetime of the iterator.
	#[inline]
	unsafe fn simd_iter_rows_ptr<const LANES: usize>(&self) -> SimdIterWindowsPtr<Self::Item, LANES> {
		self.as_ptr().simd_iter_rows_ptr()
	}

	/// Returns an iterator over pointers to the pixels of the specified column.
	///
	/// # Safety
	///
	/// The caller must ensure that the pointer contained by the [`Img`] is
	/// valid for reads from all pixels of the specified column, and that the
	/// pointer remains valid for the lifetime of the iterator.
	///
	/// # Panics
	///
	/// Panics if the specified column is out of bounds for the [`Img`].
	#[inline]
	unsafe fn simd_iter_col_ptr<const LANES: usize>(&self, col: usize) -> SimdIterPtr<Self::Item, LANES> {
		self.as_ptr().simd_iter_col_ptr::<LANES>(col)
	}

	/// Returns an iterator over [`SimdIterWindowPtr`]s.
	///
	/// # Safety
	///
	/// The caller must ensure that the pointer contained by the [`Img`] is
	/// valid for reads from all pixels, and that the pointer remains valid for
	/// the lifetime of the iterator.
	#[inline]
	unsafe fn simd_iter_cols_ptr<const LANES: usize>(&self) -> SimdIterWindowsPtr<Self::Item, LANES> {
		self.as_ptr().simd_iter_cols_ptr()
	}
}

/// Exposes iterators that return arrays of `*mut` pointers.
///
/// Implemented for `mut` buffer pointers, i.e. [`Img<*mut [T]>`][Img].
#[cfg(feature = "simd")]
pub trait ImgSimdIterPtrMut: sealed::SealedSimdPtrMut + ImgSimdIterPtr + ImgIterPtrMut {
	/// Returns an iterator over pointers to the pixels of the specified row.
	/// row.
	///
	/// # Safety
	///
	/// The caller must ensure that the pointer contained by the [`Img`] is
	/// valid for reads from all pixels of the specified row, and that the
	/// pointer remains valid for the lifetime of the iterator.
	///
	/// # Panics
	///
	/// Panics if the specified row is out of bounds for the [`Img`].
	#[inline]
	unsafe fn simd_iter_row_ptr_mut<const LANES: usize>(&self, row: usize) -> SimdIterPtrMut<Self::Item, LANES> {
		self.as_mut_ptr().simd_iter_row_ptr_mut::<LANES>(row)
	}

	/// Returns an iterator over [`SimdIterWindowPtrMut`]s.
	///
	/// # Safety
	///
	/// The caller must ensure that the pointer contained by the [`Img`] is
	/// valid for reads and writes for all pixels, and that the pointer remains
	/// valid for the lifetime of the iterator.
	#[inline]
	unsafe fn simd_iter_rows_ptr_mut<const LANES: usize>(&self) -> SimdIterWindowsPtrMut<Self::Item, LANES> {
		self.as_mut_ptr().simd_iter_rows_ptr_mut()
	}

	/// Returns an iterator over pointers to the pixels of the specified column.
	///
	/// # Safety
	///
	/// The caller must ensure that the pointer contained by the [`Img`] is
	/// valid for reads from all pixels of the specified column, and that the
	/// pointer remains valid for the lifetime of the iterator.
	///
	/// # Panics
	///
	/// Panics if the specified column is out of bounds for the [`Img`].
	#[inline]
	unsafe fn simd_iter_col_ptr_mut<const LANES: usize>(&self, col: usize) -> SimdIterPtrMut<Self::Item, LANES> {
		self.as_mut_ptr().simd_iter_col_ptr_mut::<LANES>(col)
	}

	/// Returns an iterator over [`SimdIterWindowPtrMut`]s.
	///
	/// # Safety
	///
	/// The caller must ensure that the pointer contained by the [`Img`] is
	/// valid for reads and writes for all pixels, and that the pointer remains
	/// valid for the lifetime of the iterator.
	#[inline]
	unsafe fn simd_iter_cols_ptr_mut<const LANES: usize>(&self) -> SimdIterWindowsPtrMut<Self::Item, LANES> {
		self.as_mut_ptr().simd_iter_cols_ptr_mut()
	}
}

/// Exposes iterators that return arrays of `&` references.
///
/// Implemented for all ordinary references and owned containers, i.e.
/// [`Img<&[T]>`][Img].
#[cfg(feature = "simd")]
pub trait ImgSimdIter: sealed::SealedSimd + ImgIter {
	/// Returns an iterator over the pixels of the specified row.
	///
	/// # Panics
	///
	/// Panics if the specified row is out of bounds for the [`Img`].
	fn simd_iter_row<const LANES: usize>(&self, row: usize) -> SimdIter<Self::Item, LANES>;

	/// Returns an iterator over rows.
	fn simd_iter_rows<const LANES: usize>(&self) -> SimdIterWindows<Self::Item, LANES>;

	/// Returns an iterator over the pixels of the specified column.
	///
	/// # Panics
	///
	/// Panics if the specified column is out of bounds for the [`Img`].
	fn simd_iter_col<const LANES: usize>(&self, col: usize) -> SimdIter<Self::Item, LANES>;

	/// Returns an iterator over columns.
	fn simd_iter_cols<const LANES: usize>(&self) -> SimdIterWindows<Self::Item, LANES>;
}

/// Exposes iterators that return arrays of `&mut` references.
///
/// Implemented for all mutable references and owned containers, i.e.
/// [`Img<&mut [T]>`][Img] or [`Img<Vec<T>>`][Img].
#[cfg(feature = "simd")]
pub trait ImgSimdIterMut: sealed::SealedSimdMut + ImgIterMut {
	/// Returns an iterator over the pixels of the specified row.
	///
	/// # Panics
	///
	/// Panics if the specified row is out of bounds for the [`Img`].
	fn simd_iter_row_mut<const LANES: usize>(&mut self, row: usize) -> SimdIterMut<Self::Item, LANES>;

	/// Returns an iterator over [`SimdIterWindowMut`]s.
	fn simd_iter_rows_mut<const LANES: usize>(&mut self) -> SimdIterWindowsMut<Self::Item, LANES>;

	/// Returns an iterator over the pixels of the specified column.
	///
	/// # Panics
	///
	/// Panics if the specified column is out of bounds for the [`Img`].
	fn simd_iter_col_mut<const LANES: usize>(&mut self, col: usize) -> SimdIterMut<Self::Item, LANES>;

	/// Returns an iterator over [`SimdIterWindowMut`]s.
	fn simd_iter_cols_mut<const LANES: usize>(&mut self) -> SimdIterWindowsMut<Self::Item, LANES>;
}

// @formatter:off
impl<T> sealed::SealedAsPtr for Img<*const [T]> {}
impl<T> sealed::SealedPtr for Img<*const [T]> {}

impl<T> sealed::SealedAsPtr for Img<*mut [T]> {}
impl<T> sealed::SealedAsMutPtr for Img<*mut [T]> {}
impl<T> sealed::SealedPtr for Img<*mut [T]> {}
impl<T> sealed::SealedPtrMut for Img<*mut [T]> {}

impl<T> sealed::SealedAsPtr for Img<&[T]> {}
impl<T> sealed::SealedPtr for Img<&[T]> {}
impl<T> sealed::Sealed for Img<&[T]> {}

impl<T> sealed::SealedAsPtr for Img<&mut [T]> {}
impl<T> sealed::SealedAsMutPtr for Img<&mut [T]> {}
impl<T> sealed::SealedPtr for Img<&mut [T]> {}
impl<T> sealed::Sealed for Img<&mut [T]> {}
impl<T> sealed::SealedMut for Img<&mut [T]> {}

#[cfg(feature = "simd")] impl<T> sealed::SealedSimdPtr for Img<*const [T]> {}

#[cfg(feature = "simd")] impl<T> sealed::SealedSimdPtr for Img<*mut [T]> {}
#[cfg(feature = "simd")] impl<T> sealed::SealedSimdPtrMut for Img<*mut [T]> {}

#[cfg(feature = "simd")] impl<T> sealed::SealedSimdPtr for Img<&[T]> {}
#[cfg(feature = "simd")] impl<T> sealed::SealedSimd for Img<&[T]> {}

#[cfg(feature = "simd")] impl<T> sealed::SealedSimdPtr for Img<&mut [T]> {}
#[cfg(feature = "simd")] impl<T> sealed::SealedSimd for Img<&mut [T]> {}
#[cfg(feature = "simd")] impl<T> sealed::SealedSimdMut for Img<&mut [T]> {}
// @formatter:on

#[inline]
unsafe fn copy_buf_unchecked<T, U>(img: &Img<T>, map: impl FnOnce(&T) -> U) -> Img<U> {
	let (width, height, stride) = (img.width(), img.height(), img.stride());
	Img::new_stride(map(img.buf()), width, height, stride)
}

#[inline]
unsafe fn copy_buf_unchecked_mut<T, U>(img: &mut Img<T>, map: impl FnOnce(&mut T) -> U) -> Img<U> {
	let (width, height, stride) = (img.width(), img.height(), img.stride());
	Img::new_stride(map(img.buf_mut()), width, height, stride)
}

impl<T> ImgAsPtr for Img<*const [T]> {
	type Item = T;
	type AsPtr = Self;

	#[inline]
	fn as_ptr(&self) -> Self::AsPtr {
		*self
	}
}

impl<T> ImgAsPtr for Img<*mut [T]> {
	type Item = T;
	type AsPtr = Img<*const [T]>;

	#[inline]
	fn as_ptr(&self) -> Self::AsPtr {
		unsafe { copy_buf_unchecked(self, |buf| *buf as *const [T]) }
	}
}

impl<T> ImgAsPtr for Img<&[T]> {
	type Item = T;
	type AsPtr = Img<*const [T]>;

	#[inline]
	fn as_ptr(&self) -> Self::AsPtr {
		unsafe { copy_buf_unchecked(self, |buf| *buf as *const [T]) }
	}
}

impl<T> ImgAsPtr for Img<&mut [T]> {
	type Item = T;
	type AsPtr = Img<*const [T]>;

	#[inline]
	fn as_ptr(&self) -> Self::AsPtr {
		unsafe { copy_buf_unchecked(self, |buf| *buf as *const [T]) }
	}
}

impl<T> ImgAsMutPtr for Img<*mut [T]> {
	type AsMutPtr = Img<*mut [T]>;

	#[inline]
	fn as_mut_ptr(&self) -> Self::AsMutPtr {
		*self
	}
}

impl<T> ImgIterPtr for Img<*const [T]> {
	#[inline]
	unsafe fn iter_row_ptr(&self, row: usize) -> IterPtr<Self::Item> {
		IterPtr::row_ptr(*self, row)
	}

	#[inline]
	unsafe fn iter_rows_ptr(&self) -> IterWindowsPtr<Self::Item> {
		IterWindowsPtr::rows_ptr(*self)
	}

	#[inline]
	unsafe fn iter_col_ptr(&self, col: usize) -> IterPtr<Self::Item> {
		IterPtr::col_ptr(*self, col)
	}

	#[inline]
	unsafe fn iter_cols_ptr(&self) -> IterWindowsPtr<Self::Item> {
		IterWindowsPtr::cols_ptr(*self)
	}
}

impl<T> ImgIterPtr for Img<*mut [T]> {}

impl<T> ImgIterPtr for Img<&[T]> {}

impl<T> ImgIterPtr for Img<&mut [T]> {}

impl<T> ImgIterPtrMut for Img<*mut [T]> {
	#[inline]
	unsafe fn iter_row_ptr_mut(&self, row: usize) -> IterPtrMut<Self::Item> {
		IterPtrMut::row_ptr(*self, row)
	}

	#[inline]
	unsafe fn iter_rows_ptr_mut(&self) -> IterWindowsPtrMut<Self::Item> {
		IterWindowsPtrMut::rows_ptr(*self)
	}

	#[inline]
	unsafe fn iter_col_ptr_mut(&self, col: usize) -> IterPtrMut<Self::Item> {
		IterPtrMut::col_ptr(*self, col)
	}

	#[inline]
	unsafe fn iter_cols_ptr_mut(&self) -> IterWindowsPtrMut<Self::Item> {
		IterWindowsPtrMut::cols_ptr(*self)
	}
}

impl<T> ImgIter for Img<&[T]> {
	#[inline]
	fn iter_row(&self, row: usize) -> Iter<Self::Item> {
		Iter::row(self, row)
	}

	#[inline]
	fn iter_rows(&self) -> IterWindows<Self::Item> {
		IterWindows::rows(self)
	}

	#[inline]
	fn iter_col(&self, col: usize) -> Iter<Self::Item> {
		Iter::col(self, col)
	}

	#[inline]
	fn iter_cols(&self) -> IterWindows<Self::Item> {
		IterWindows::cols(self)
	}
}

impl<T> ImgIter for Img<&mut [T]> {
	#[inline]
	fn iter_row(&self, row: usize) -> Iter<Self::Item> {
		Iter::row(self, row)
	}

	#[inline]
	fn iter_rows(&self) -> IterWindows<Self::Item> {
		IterWindows::rows(self)
	}

	#[inline]
	fn iter_col(&self, col: usize) -> Iter<Self::Item> {
		Iter::col(self, col)
	}

	#[inline]
	fn iter_cols(&self) -> IterWindows<Self::Item> {
		IterWindows::cols(self)
	}
}

impl<T> ImgIterMut for Img<&mut [T]> {
	type AsMutPtr = Img<*mut [T]>;

	#[inline]
	fn as_mut_ptr(&mut self) -> Self::AsMutPtr {
		unsafe { copy_buf_unchecked_mut(self, |buf| *buf as *mut [T]) }
	}

	#[inline]
	fn iter_row_mut(&mut self, row: usize) -> IterMut<Self::Item> {
		IterMut::row(self, row)
	}

	#[inline]
	fn iter_rows_mut(&mut self) -> IterWindowsMut<Self::Item> {
		IterWindowsMut::rows(self)
	}

	#[inline]
	fn iter_col_mut(&mut self, col: usize) -> IterMut<Self::Item> {
		IterMut::col(self, col)
	}

	#[inline]
	fn iter_cols_mut(&mut self) -> IterWindowsMut<Self::Item> {
		IterWindowsMut::cols(self)
	}
}

#[cfg(feature = "simd")]
impl<T> ImgSimdIterPtr for Img<*const [T]> {
	#[inline]
	unsafe fn simd_iter_row_ptr<const LANES: usize>(&self, row: usize) -> SimdIterPtr<Self::Item, LANES> {
		SimdIterPtr::rows_ptr(*self, row)
	}

	#[inline]
	unsafe fn simd_iter_rows_ptr<const LANES: usize>(&self) -> SimdIterWindowsPtr<Self::Item, LANES> {
		SimdIterWindowsPtr::rows_ptr(*self)
	}

	#[inline]
	unsafe fn simd_iter_col_ptr<const LANES: usize>(&self, col: usize) -> SimdIterPtr<Self::Item, LANES> {
		SimdIterPtr::cols_ptr(*self, col)
	}

	#[inline]
	unsafe fn simd_iter_cols_ptr<const LANES: usize>(&self) -> SimdIterWindowsPtr<Self::Item, LANES> {
		SimdIterWindowsPtr::cols_ptr(*self)
	}
}

#[cfg(feature = "simd")]
impl<T> ImgSimdIterPtr for Img<*mut [T]> {}

#[cfg(feature = "simd")]
impl<T> ImgSimdIterPtr for Img<&[T]> {}

#[cfg(feature = "simd")]
impl<T> ImgSimdIterPtr for Img<&mut [T]> {}

#[cfg(feature = "simd")]
impl<T> ImgSimdIterPtrMut for Img<*mut [T]> {
	#[inline]
	unsafe fn simd_iter_row_ptr_mut<const LANES: usize>(&self, row: usize) -> SimdIterPtrMut<Self::Item, LANES> {
		SimdIterPtrMut::rows_ptr(*self, row)
	}

	#[inline]
	unsafe fn simd_iter_rows_ptr_mut<const LANES: usize>(&self) -> SimdIterWindowsPtrMut<Self::Item, LANES> {
		SimdIterWindowsPtrMut::rows_ptr(*self)
	}

	#[inline]
	unsafe fn simd_iter_col_ptr_mut<const LANES: usize>(&self, col: usize) -> SimdIterPtrMut<Self::Item, LANES> {
		SimdIterPtrMut::cols_ptr(*self, col)
	}

	#[inline]
	unsafe fn simd_iter_cols_ptr_mut<const LANES: usize>(&self) -> SimdIterWindowsPtrMut<Self::Item, LANES> {
		SimdIterWindowsPtrMut::cols_ptr(*self)
	}
}

#[cfg(feature = "simd")]
impl<T> ImgSimdIter for Img<&[T]> {
	#[inline]
	fn simd_iter_row<const LANES: usize>(&self, row: usize) -> SimdIter<Self::Item, LANES> {
		SimdIter::rows(self, row)
	}

	#[inline]
	fn simd_iter_rows<const LANES: usize>(&self) -> SimdIterWindows<Self::Item, LANES> {
		SimdIterWindows::rows(self)
	}

	#[inline]
	fn simd_iter_col<const LANES: usize>(&self, col: usize) -> SimdIter<Self::Item, LANES> {
		SimdIter::cols(self, col)
	}

	#[inline]
	fn simd_iter_cols<const LANES: usize>(&self) -> SimdIterWindows<Self::Item, LANES> {
		SimdIterWindows::cols(self)
	}
}

#[cfg(feature = "simd")]
impl<T> ImgSimdIter for Img<&mut [T]> {
	#[inline]
	fn simd_iter_row<const LANES: usize>(&self, row: usize) -> SimdIter<Self::Item, LANES> {
		SimdIter::rows(self, row)
	}

	#[inline]
	fn simd_iter_rows<const LANES: usize>(&self) -> SimdIterWindows<Self::Item, LANES> {
		SimdIterWindows::rows(self)
	}

	#[inline]
	fn simd_iter_col<const LANES: usize>(&self, col: usize) -> SimdIter<Self::Item, LANES> {
		SimdIter::cols(self, col)
	}

	#[inline]
	fn simd_iter_cols<const LANES: usize>(&self) -> SimdIterWindows<Self::Item, LANES> {
		SimdIterWindows::cols(self)
	}
}

#[cfg(feature = "simd")]
impl<T> ImgSimdIterMut for Img<&mut [T]> {
	#[inline]
	fn simd_iter_row_mut<const LANES: usize>(&mut self, row: usize) -> SimdIterMut<Self::Item, LANES> {
		SimdIterMut::rows(self, row)
	}

	#[inline]
	fn simd_iter_rows_mut<const LANES: usize>(&mut self) -> SimdIterWindowsMut<Self::Item, LANES> {
		SimdIterWindowsMut::rows(self)
	}

	#[inline]
	fn simd_iter_col_mut<const LANES: usize>(&mut self, col: usize) -> SimdIterMut<Self::Item, LANES> {
		SimdIterMut::cols(self, col)
	}

	#[inline]
	fn simd_iter_cols_mut<const LANES: usize>(&mut self) -> SimdIterWindowsMut<Self::Item, LANES> {
		SimdIterWindowsMut::cols(self)
	}
}
