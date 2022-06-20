use std::iter::FusedIterator;
use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
use imgref::Img;
use crate::iter::{IterPtr, IterPtrMut};

#[repr(transparent)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IterRowPtr<T>(IterPtr<T>);

impl<T> IterRowPtr<T> {
	/// Creates a new [`IterRowPtr`] over the specified row of a buffer.
	///
	/// # Panics
	///
	/// Panics if the provided row is out of bounds.
	///
	/// # Safety
	///
	/// The provided [`Img`] must be valid for the lifetime of the returned
	/// [`IterRowPtr`].
	#[inline]
	pub unsafe fn new(buf: &Img<*const [T]>, row: usize) -> Self {
		assert!(row < buf.height());
		Self::new_unchecked(buf, row)
	}

	/// Creates a new [`IterRowPtr`] over the specified row of a buffer.
	///
	/// # Safety
	///
	/// The provided row must not be out of bounds, and the provided [`Img`]
	/// must be valid for the lifetime of the returned [`IterRowPtr`].
	#[inline]
	pub unsafe fn new_unchecked(buf: &Img<*const [T]>, row: usize) -> Self {
		let first = buf.buf().cast::<T>().add(buf.stride() * row);
		let row = slice_from_raw_parts(first, buf.width());
		Self(IterPtr::new(row, 1))
	}
}

impl<T> Iterator for IterRowPtr<T> {
	type Item = *const T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		unsafe { self.0.next() }
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = unsafe { self.0.len() };
		(len, Some(len))
	}
}

impl<T> DoubleEndedIterator for IterRowPtr<T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		unsafe { self.0.next_back() }
	}
}

impl<T> ExactSizeIterator for IterRowPtr<T> {
	#[inline]
	fn len(&self) -> usize {
		unsafe { self.0.len() }
	}
}

impl<T> FusedIterator for IterRowPtr<T> {}

#[repr(transparent)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IterRowPtrMut<T>(IterPtrMut<T>);

impl<T> IterRowPtrMut<T> {
	/// Creates a new [`IterRowPtrMut`] over the specified row of a buffer.
	///
	/// # Panics
	///
	/// Panics if the provided row is out of bounds.
	///
	/// # Safety
	///
	/// The provided [`Img`] must be valid for the lifetime of the returned
	/// [`IterRowPtrMut`].
	#[inline]
	pub unsafe fn new(buf: &Img<*mut [T]>, row: usize) -> Self {
		assert!(row < buf.height());
		Self::new_unchecked(buf, row)
	}

	/// Creates a new [`IterRowPtrMut`] over the specified row of a buffer.
	///
	/// # Safety
	///
	/// The provided row must not be out of bounds, and the provided [`Img`]
	/// must be valid for the lifetime of the returned [`IterRowPtrMut`].
	#[inline]
	pub unsafe fn new_unchecked(buf: &Img<*mut [T]>, row: usize) -> Self {
		let first = buf.buf().cast::<T>().add(buf.stride() * row);
		let row = slice_from_raw_parts_mut(first, buf.width());
		Self(IterPtrMut::new(row, 1))
	}
}

impl<T> Iterator for IterRowPtrMut<T> {
	type Item = *mut T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		unsafe { self.0.next() }
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = unsafe { self.0.len() };
		(len, Some(len))
	}
}

impl<T> DoubleEndedIterator for IterRowPtrMut<T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		unsafe { self.0.next_back() }
	}
}

impl<T> ExactSizeIterator for IterRowPtrMut<T> {
	#[inline]
	fn len(&self) -> usize {
		unsafe { self.0.len() }
	}
}

impl<T> FusedIterator for IterRowPtrMut<T> {}
