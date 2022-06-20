use std::iter::FusedIterator;
use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
use imgref::Img;
use crate::iter::{IterPtr, IterPtrMut};

#[repr(transparent)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IterColPtr<T>(IterPtr<T>);

impl<T> IterColPtr<T> {
	/// Creates a new [`IterColPtr`] over the specified col of a buffer.
	///
	/// # Panics
	///
	/// Panics if the provided col is out of bounds.
	///
	/// # Safety
	///
	/// The provided [`Img`] must be valid for the lifetime of the returned
	/// [`IterColPtr`].
	#[inline]
	pub unsafe fn new(buf: &Img<*const [T]>, col: usize) -> Self {
		assert!(col < buf.width());
		Self::new_unchecked(buf, col)
	}

	/// Creates a new [`IterColPtr`] over the specified col of a buffer.
	///
	/// # Safety
	///
	/// The provided col must not be out of bounds, and the provided [`Img`]
	/// must be valid for the lifetime of the returned [`IterColPtr`].
	#[inline]
	pub unsafe fn new_unchecked(buf: &Img<*const [T]>, col: usize) -> Self {
		let first = buf.buf().cast::<T>().add(col);
		let stride = buf.stride();
		let col = slice_from_raw_parts(first, stride * (buf.height() - 1) + 1);
		Self(IterPtr::new(col, stride))
	}
}

impl<T> Iterator for IterColPtr<T> {
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

impl<T> DoubleEndedIterator for IterColPtr<T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		unsafe { self.0.next_back() }
	}
}

impl<T> ExactSizeIterator for IterColPtr<T> {
	#[inline]
	fn len(&self) -> usize {
		unsafe { self.0.len() }
	}
}

impl<T> FusedIterator for IterColPtr<T> {}

#[repr(transparent)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IterColPtrMut<T>(IterPtrMut<T>);

impl<T> IterColPtrMut<T> {
	/// Creates a new [`IterColPtrMut`] over the specified col of a buffer.
	///
	/// # Panics
	///
	/// Panics if the provided col is out of bounds.
	///
	/// # Safety
	///
	/// The provided [`Img`] must be valid for the lifetime of the returned
	/// [`IterColPtrMut`].
	#[inline]
	pub unsafe fn new(buf: &Img<*mut [T]>, col: usize) -> Self {
		assert!(col < buf.width());
		Self::new_unchecked(buf, col)
	}

	/// Creates a new [`IterColPtrMut`] over the specified col of a buffer.
	///
	/// # Safety
	///
	/// The provided col must not be out of bounds, and the provided [`Img`]
	/// must be valid for the lifetime of the returned [`IterColPtrMut`].
	#[inline]
	pub unsafe fn new_unchecked(buf: &Img<*mut [T]>, col: usize) -> Self {
		let first = buf.buf().cast::<T>().add(col);
		let stride = buf.stride();
		let col = slice_from_raw_parts_mut(first, stride * (buf.height() - 1) + 1);
		Self(IterPtrMut::new(col, stride))
	}
}

impl<T> Iterator for IterColPtrMut<T> {
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

impl<T> DoubleEndedIterator for IterColPtrMut<T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		unsafe { self.0.next_back() }
	}
}

impl<T> ExactSizeIterator for IterColPtrMut<T> {
	#[inline]
	fn len(&self) -> usize {
		unsafe { self.0.len() }
	}
}

impl<T> FusedIterator for IterColPtrMut<T> {}
