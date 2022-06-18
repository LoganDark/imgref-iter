//! Contains the traits that allow obtaining iterators.

use std::marker::PhantomData;
use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
use imgref::Img;

use crate::iter::{
	IterRow,
	IterRowMut,
	IterRowPtr,
	IterRowPtrMut,
	IterRows,
	IterRowsMut,
	IterRowsPtr,
	IterRowsPtrMut,
	IterCol,
	IterColMut,
	IterColPtr,
	IterColPtrMut,
	IterCols,
	IterColsMut,
	IterColsPtr,
	IterColsPtrMut
};

mod sealed { pub trait Sealed {} }

/// Exposes iterators that return `*const` pointers.
///
/// Implemented for buffer pointers, i.e. [`Img<*const [T]>`][Img] and
/// [`Img<*mut [T]>`][Img].
pub trait ImgIterPtr: sealed::Sealed {
	type Item;
	type AsPtr: ImgIterPtr<Item = Self::Item>;

	/// Returns this [`Img`] as with the buffer type converted to a pointer.
	fn as_ptr(&self) -> Self::AsPtr;

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
	unsafe fn iter_row_ptr(&self, row: usize) -> IterRowPtr<Self::Item> {
		self.as_ptr().iter_row_ptr(row)
	}

	/// Returns an iterator over [`IterRowPtr`]s.
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
	unsafe fn iter_col_ptr(&self, col: usize) -> IterColPtr<Self::Item> {
		self.as_ptr().iter_col_ptr(col)
	}

	/// Returns an iterator over [`IterColPtr`]s.
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
pub trait ImgIterPtrMut: ImgIterPtr {
	type AsMutPtr: ImgIterPtrMut<Item = Self::Item>;

	/// Returns this [`Img`] as with the buffer type converted to a mutable
	/// pointer.
	fn as_mut_ptr(&mut self) -> Self::AsMutPtr;

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
	unsafe fn iter_row_ptr_mut(&mut self, row: usize) -> IterRowPtrMut<Self::Item> {
		self.as_mut_ptr().iter_row_ptr_mut(row)
	}

	/// Returns an iterator over [`IterRowPtrMut`]s.
	///
	/// # Safety
	///
	/// The caller must ensure that the pointer contained by the [`Img`] is
	/// valid for reads and writes for all pixels, and that the pointer remains
	/// valid for the lifetime of the iterator.
	#[inline]
	unsafe fn iter_rows_ptr_mut(&mut self) -> IterRowsPtrMut<Self::Item> {
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
	unsafe fn iter_col_ptr_mut(&mut self, col: usize) -> IterColPtrMut<Self::Item> {
		self.as_mut_ptr().iter_col_ptr_mut(col)
	}

	/// Returns an iterator over [`IterColPtrMut`]s.
	///
	/// # Safety
	///
	/// The caller must ensure that the pointer contained by the [`Img`] is
	/// valid for reads and writes for all pixels, and that the pointer remains
	/// valid for the lifetime of the iterator.
	#[inline]
	unsafe fn iter_cols_ptr_mut(&mut self) -> IterColsPtrMut<Self::Item> {
		self.as_mut_ptr().iter_cols_ptr_mut()
	}
}

/// Exposes iterators that return `&` references.
///
/// Implemented for all ordinary references and owned containers, i.e.
/// [`Img<&[T]>`][Img].
pub trait ImgIter: ImgIterPtr {
	/// Returns an iterator over the pixels of the specified row.
	///
	/// # Panics
	///
	/// Panics if the specified row is out of bounds for the [`Img`].
	fn iter_row(&self, row: usize) -> IterRow<Self::Item>;

	/// Returns an iterator over [`IterRow`]s.
	fn iter_rows(&self) -> IterRows<Self::Item>;

	/// Returns an iterator over the pixels of the specified column.
	///
	/// # Panics
	///
	/// Panics if the specified column is out of bounds for the [`Img`].
	fn iter_col(&self, col: usize) -> IterCol<Self::Item>;

	/// Returns an iterator over [`IterCol`]s.
	fn iter_cols(&self) -> IterCols<Self::Item>;
}

/// Exposes iterators that return `&mut` references.
///
/// Implemented for all mutable references and owned containers, i.e.
/// [`Img<&mut [T]>`][Img] or [`Img<Vec<T>>`][Img].
pub trait ImgIterMut: ImgIter + ImgIterPtrMut {
	/// Returns an iterator over the pixels of the specified row.
	///
	/// # Panics
	///
	/// Panics if the specified row is out of bounds for the [`Img`].
	fn iter_row_mut(&mut self, row: usize) -> IterRowMut<Self::Item>;

	/// Returns an iterator over [`IterRowMut`]s.
	fn iter_rows_mut(&mut self) -> IterRowsMut<Self::Item>;

	/// Returns an iterator over the pixels of the specified column.
	///
	/// # Panics
	///
	/// Panics if the specified column is out of bounds for the [`Img`].
	fn iter_col_mut(&mut self, col: usize) -> IterColMut<Self::Item>;

	/// Returns an iterator over [`IterColMut`]s.
	fn iter_cols_mut(&mut self) -> IterColsMut<Self::Item>;
}

impl<T> sealed::Sealed for Img<*const [T]> {}

impl<T> sealed::Sealed for Img<*mut [T]> {}

impl<T> sealed::Sealed for Img<&[T]> {}

impl<T> sealed::Sealed for Img<&mut [T]> {}

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

impl<T> ImgIterPtr for Img<*const [T]> {
	type Item = T;
	type AsPtr = Img<*const [T]>;

	#[inline]
	fn as_ptr(&self) -> Self::AsPtr {
		*self
	}

	#[inline]
	unsafe fn iter_row_ptr(&self, row: usize) -> IterRowPtr<Self::Item> {
		assert!(row < self.height());

		let slice = slice_from_raw_parts(
			self.buf().cast::<T>().add(self.stride() * row),
			self.width()
		);

		IterRowPtr(slice)
	}

	#[inline]
	unsafe fn iter_rows_ptr(&self) -> IterRowsPtr<Self::Item> {
		IterRowsPtr(self.clone(), 0..self.height())
	}

	#[inline]
	unsafe fn iter_col_ptr(&self, col: usize) -> IterColPtr<Self::Item> {
		assert!(col < self.width());

		// ensure first element is first and last element is last;
		// DO NOT INCLUDE TRAILING STRIDE, as that breaks DoubleEndedIterator
		let slice = slice_from_raw_parts(
			self.buf().cast::<T>().add(col),
			self.stride() * (self.height() - 1) + 1
		);

		IterColPtr(slice, self.stride())
	}

	#[inline]
	unsafe fn iter_cols_ptr(&self) -> IterColsPtr<Self::Item> {
		IterColsPtr(self.clone(), 0..self.width())
	}
}

impl<T> ImgIterPtr for Img<*mut [T]> {
	type Item = T;
	type AsPtr = Img<*const [T]>;

	#[inline]
	fn as_ptr(&self) -> Self::AsPtr {
		unsafe { copy_buf_unchecked(self, |buf| *buf as *const [T]) }
	}
}

impl<T> ImgIterPtr for Img<&[T]> {
	type Item = T;
	type AsPtr = Img<*const [T]>;

	#[inline]
	fn as_ptr(&self) -> Self::AsPtr {
		unsafe { copy_buf_unchecked(self, |buf| *buf as *const [T]) }
	}
}

impl<T> ImgIterPtr for Img<&mut [T]> {
	type Item = T;
	type AsPtr = Img<*const [T]>;

	#[inline]
	fn as_ptr(&self) -> Self::AsPtr {
		unsafe { copy_buf_unchecked(self, |buf| *buf as *const [T]) }
	}
}

impl<T> ImgIterPtrMut for Img<*mut [T]> {
	type AsMutPtr = Img<*mut [T]>;

	#[inline]
	fn as_mut_ptr(&mut self) -> Self::AsMutPtr {
		*self
	}

	#[inline]
	unsafe fn iter_row_ptr_mut(&mut self, row: usize) -> IterRowPtrMut<Self::Item> {
		assert!(row < self.height());

		let slice = slice_from_raw_parts_mut(
			self.buf().cast::<T>().add(self.stride() * row),
			self.width()
		);

		IterRowPtrMut(slice)
	}

	#[inline]
	unsafe fn iter_rows_ptr_mut(&mut self) -> IterRowsPtrMut<Self::Item> {
		IterRowsPtrMut(self.clone(), 0..self.height())
	}

	#[inline]
	unsafe fn iter_col_ptr_mut(&mut self, col: usize) -> IterColPtrMut<Self::Item> {
		assert!(col < self.width());

		// ensure first element is first and last element is last;
		// DO NOT INCLUDE TRAILING STRIDE, as that breaks DoubleEndedIterator
		let slice = slice_from_raw_parts_mut(
			self.buf().cast::<T>().add(col),
			self.stride() * (self.height() - 1) + 1
		);

		IterColPtrMut(slice, self.stride())
	}

	#[inline]
	unsafe fn iter_cols_ptr_mut(&mut self) -> IterColsPtrMut<Self::Item> {
		IterColsPtrMut(self.clone(), 0..self.width())
	}
}

impl<T> ImgIterPtrMut for Img<&mut [T]> {
	type AsMutPtr = Img<*mut [T]>;

	#[inline]
	fn as_mut_ptr(&mut self) -> Self::AsMutPtr {
		unsafe { copy_buf_unchecked_mut(self, |buf| *buf as *mut [T]) }
	}
}

impl<T> ImgIter for Img<&[T]> {
	#[inline]
	fn iter_row(&self, row: usize) -> IterRow<Self::Item> {
		IterRow(unsafe { self.iter_row_ptr(row) }, PhantomData)
	}

	#[inline]
	fn iter_rows(&self) -> IterRows<Self::Item> {
		IterRows(
			unsafe { copy_buf_unchecked(self, |buf| *buf as *const [T]) },
			0..self.height(),
			PhantomData
		)
	}

	#[inline]
	fn iter_col(&self, col: usize) -> IterCol<Self::Item> {
		IterCol(unsafe { self.iter_col_ptr(col) }, PhantomData)
	}

	#[inline]
	fn iter_cols(&self) -> IterCols<Self::Item> {
		IterCols(
			unsafe { copy_buf_unchecked(self, |buf| *buf as *const [T]) },
			0..self.width(),
			PhantomData
		)
	}
}

impl<T> ImgIter for Img<&mut [T]> {
	#[inline]
	fn iter_row(&self, row: usize) -> IterRow<Self::Item> {
		IterRow(unsafe { self.iter_row_ptr(row) }, PhantomData)
	}

	#[inline]
	fn iter_rows(&self) -> IterRows<Self::Item> {
		IterRows(
			unsafe { copy_buf_unchecked(self, |buf| *buf as *const [T]) },
			0..self.height(),
			PhantomData
		)
	}

	#[inline]
	fn iter_col(&self, col: usize) -> IterCol<Self::Item> {
		IterCol(unsafe { self.iter_col_ptr(col) }, PhantomData)
	}

	#[inline]
	fn iter_cols(&self) -> IterCols<Self::Item> {
		IterCols(
			unsafe { copy_buf_unchecked(self, |buf| *buf as *const [T]) },
			0..self.width(),
			PhantomData
		)
	}
}

impl<T> ImgIterMut for Img<&mut [T]> {
	#[inline]
	fn iter_row_mut(&mut self, row: usize) -> IterRowMut<Self::Item> {
		IterRowMut(unsafe { self.iter_row_ptr_mut(row) }, PhantomData)
	}

	#[inline]
	fn iter_rows_mut(&mut self) -> IterRowsMut<Self::Item> {
		IterRowsMut(
			unsafe { copy_buf_unchecked_mut(self, |buf| *buf as *mut [T]) },
			0..self.height(),
			PhantomData
		)
	}

	#[inline]
	fn iter_col_mut(&mut self, col: usize) -> IterColMut<Self::Item> {
		IterColMut(unsafe { self.iter_col_ptr_mut(col) }, PhantomData)
	}

	#[inline]
	fn iter_cols_mut(&mut self) -> IterColsMut<Self::Item> {
		IterColsMut(
			unsafe { copy_buf_unchecked_mut(self, |buf| *buf as *mut [T]) },
			0..self.width(),
			PhantomData
		)
	}
}
