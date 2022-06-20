//! Contains the traits that allow obtaining iterators.

use imgref::Img;

use crate::iter::{
	Iter,
	IterMut,
	IterPtr,
	IterPtrMut,
	IterRows,
	IterRowsMut,
	IterRowsPtr,
	IterRowsPtrMut,
	IterCols,
	IterColsMut,
	IterColsPtr,
	IterColsPtrMut
};

mod sealed {
	pub trait SealedAsPtr {}

	pub trait SealedAsMutPtr {}

	pub trait SealedPtr {}

	pub trait SealedPtrMut {}

	pub trait Sealed {}

	pub trait SealedMut {}
}

/// The trait for images whose buffers can be converted to a `*const` pointer.
pub trait ImgAsPtr: sealed::SealedAsPtr {
	type Item;
	type AsPtr: ImgIterPtr<Item = Self::Item>;

	/// Returns an [`Img`] that points to this one's buffer.
	fn as_ptr(&self) -> Self::AsPtr;
}

/// The trait for [`Img`]s whose buffers can be converted to a `*mut` pointer
/// *through a shared borrow*. `Img<&mut [T]>` cannot implement this because a
/// mutable reference behind a shared reference becomes immutable - but
/// [`ImgIterMut`] has another [`as_mut_ptr`][ImgIterMut::as_mut_ptr] method.
pub trait ImgAsMutPtr: sealed::SealedAsMutPtr + ImgAsPtr {
	type AsMutPtr: ImgIterPtrMut<Item = Self::Item>;

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
	unsafe fn iter_rows_ptr(&self) -> IterRowsPtr<Self::Item> {
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
	unsafe fn iter_cols_ptr(&self) -> IterColsPtr<Self::Item> {
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
	unsafe fn iter_rows_ptr_mut(&self) -> IterRowsPtrMut<Self::Item> {
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
	unsafe fn iter_cols_ptr_mut(&self) -> IterColsPtrMut<Self::Item> {
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
	fn iter_rows(&self) -> IterRows<Self::Item>;

	/// Returns an iterator over the pixels of the specified column.
	///
	/// # Panics
	///
	/// Panics if the specified column is out of bounds for the [`Img`].
	fn iter_col(&self, col: usize) -> Iter<Self::Item>;

	/// Returns an iterator over columns.
	fn iter_cols(&self) -> IterCols<Self::Item>;
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
	fn iter_rows_mut(&mut self) -> IterRowsMut<Self::Item>;

	/// Returns an iterator over the pixels of the specified column.
	///
	/// # Panics
	///
	/// Panics if the specified column is out of bounds for the [`Img`].
	fn iter_col_mut(&mut self, col: usize) -> IterMut<Self::Item>;

	/// Returns an iterator over [`IterMut`]s.
	fn iter_cols_mut(&mut self) -> IterColsMut<Self::Item>;
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
impl<T> sealed::SealedPtrMut for Img<&mut [T]> {}
impl<T> sealed::Sealed for Img<&mut [T]> {}
impl<T> sealed::SealedMut for Img<&mut [T]> {}
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
	unsafe fn iter_rows_ptr(&self) -> IterRowsPtr<Self::Item> {
		IterRowsPtr::new(*self)
	}

	#[inline]
	unsafe fn iter_col_ptr(&self, col: usize) -> IterPtr<Self::Item> {
		IterPtr::col_ptr(*self, col)
	}

	#[inline]
	unsafe fn iter_cols_ptr(&self) -> IterColsPtr<Self::Item> {
		IterColsPtr::new(*self)
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
	unsafe fn iter_rows_ptr_mut(&self) -> IterRowsPtrMut<Self::Item> {
		IterRowsPtrMut::new(*self)
	}

	#[inline]
	unsafe fn iter_col_ptr_mut(&self, col: usize) -> IterPtrMut<Self::Item> {
		IterPtrMut::col_ptr(*self, col)
	}

	#[inline]
	unsafe fn iter_cols_ptr_mut(&self) -> IterColsPtrMut<Self::Item> {
		IterColsPtrMut::new(*self)
	}
}

impl<T> ImgIter for Img<&[T]> {
	#[inline]
	fn iter_row(&self, row: usize) -> Iter<Self::Item> {
		Iter::row(self, row)
	}

	#[inline]
	fn iter_rows(&self) -> IterRows<Self::Item> {
		IterRows::new(self)
	}

	#[inline]
	fn iter_col(&self, col: usize) -> Iter<Self::Item> {
		Iter::col(self, col)
	}

	#[inline]
	fn iter_cols(&self) -> IterCols<Self::Item> {
		IterCols::new(self)
	}
}

impl<T> ImgIter for Img<&mut [T]> {
	#[inline]
	fn iter_row(&self, row: usize) -> Iter<Self::Item> {
		Iter::row(self, row)
	}

	#[inline]
	fn iter_rows(&self) -> IterRows<Self::Item> {
		IterRows::new(self)
	}

	#[inline]
	fn iter_col(&self, col: usize) -> Iter<Self::Item> {
		Iter::col(self, col)
	}

	#[inline]
	fn iter_cols(&self) -> IterCols<Self::Item> {
		IterCols::new(self)
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
	fn iter_rows_mut(&mut self) -> IterRowsMut<Self::Item> {
		IterRowsMut::new(self)
	}

	#[inline]
	fn iter_col_mut(&mut self, col: usize) -> IterMut<Self::Item> {
		IterMut::col(self, col)
	}

	#[inline]
	fn iter_cols_mut(&mut self) -> IterColsMut<Self::Item> {
		IterColsMut::new(self)
	}
}
