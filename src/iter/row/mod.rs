use std::iter::FusedIterator;
use imgref::Img;
use crate::iter::{Iter, IterMut};

mod ptr;

pub use ptr::*;

#[repr(transparent)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IterRow<'a, T>(Iter<'a, T>);

impl<'a, T> IterRow<'a, T> {
	/// Creates a new [`IterRow`] over the specified row of a buffer.
	///
	/// # Panics
	///
	/// Panics if the provided row is out of bounds.
	#[inline]
	pub fn new<S: AsRef<[T]>>(buf: &'a Img<S>, row: usize) -> Self {
		assert!(row < buf.height());
		let offset = buf.stride() * row;
		let width = buf.width();
		let row = &buf.buf().as_ref()[offset..offset + width];
		Self::new_row(row)
	}

	/// Creates a new [`IterRowMut`] over the provided row and stride.
	///
	/// # Panics
	///
	/// Panics if the slice does not start and end on an element.
	#[inline]
	pub fn new_row(row: &'a [T]) -> Self {
		Self(Iter::new(row, 1))
	}

	/// Creates a new [`IterRowMut`] over the provided row and stride.
	///
	/// # Safety
	///
	/// The slice must start and end on an element.
	#[inline]
	pub unsafe fn new_row_unchecked(row: &'a [T]) -> Self {
		Self(Iter::new_unchecked(row, 1))
	}

	/// Creates a new [`IterRowMut`] over the provided row and stride.
	///
	/// # Panics
	///
	/// Panics if the slice does not start and end on an element.
	///
	/// # Safety
	///
	/// The provided slice must remain valid for the lifetime of this
	/// [`IterRowMut`].
	#[inline]
	pub unsafe fn new_row_ptr(row: *const [T]) -> Self {
		Self(Iter::new_ptr(row, 1))
	}

	/// Creates a new [`IterRowMut`] over the provided row and stride.
	///
	/// # Safety
	///
	/// The slice must start and end on an element, and the provided slice must
	/// remain valid for the lifetime of this [`IterRowMut`].
	#[inline]
	pub unsafe fn new_row_ptr_unchecked(row: *const [T]) -> Self {
		Self(Iter::new_ptr_unchecked(row, 1))
	}
}

impl<'a, T> Iterator for IterRow<'a, T> {
	type Item = &'a T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next()
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.0.len();
		(len, Some(len))
	}
}

impl<'a, T> DoubleEndedIterator for IterRow<'a, T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.0.next_back()
	}
}

impl<'a, T> ExactSizeIterator for IterRow<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<'a, T> FusedIterator for IterRow<'a, T> {}

#[repr(transparent)]
#[derive(Eq, PartialEq, Debug)]
pub struct IterRowMut<'a, T>(IterMut<'a, T>);

impl<'a, T> IterRowMut<'a, T> {
	/// Creates a new [`IterRowMut`] over the specified row of a buffer.
	///
	/// # Panics
	///
	/// Panics if the provided row is out of bounds.
	#[inline]
	pub fn new<S: AsMut<[T]>>(buf: &'a mut Img<S>, row: usize) -> Self {
		assert!(row < buf.height());
		let offset = buf.stride() * row;
		let width = buf.width();
		let row = &mut buf.buf_mut().as_mut()[offset..offset + width];
		Self::new_row(row)
	}

	/// Creates a new [`IterRowMut`] over the provided row and stride.
	///
	/// # Panics
	///
	/// Panics if the slice does not start and end on an element.
	#[inline]
	pub fn new_row(row: &'a mut [T]) -> Self {
		Self(IterMut::new(row, 1))
	}

	/// Creates a new [`IterRowMut`] over the provided row and stride.
	///
	/// # Safety
	///
	/// The slice must start and end on an element.
	#[inline]
	pub unsafe fn new_row_unchecked(row: &'a mut [T]) -> Self {
		Self(IterMut::new_unchecked(row, 1))
	}

	/// Creates a new [`IterRowMut`] over the provided row and stride.
	///
	/// # Panics
	///
	/// Panics if the slice does not start and end on an element.
	///
	/// # Safety
	///
	/// The provided slice must remain valid for the lifetime of this
	/// [`IterRowMut`].
	#[inline]
	pub unsafe fn new_row_ptr(row: *mut [T]) -> Self {
		Self(IterMut::new_ptr(row, 1))
	}

	/// Creates a new [`IterRowMut`] over the provided row and stride.
	///
	/// # Safety
	///
	/// The slice must start and end on an element, and the provided slice must
	/// remain valid for the lifetime of this [`IterRowMut`].
	#[inline]
	pub unsafe fn new_row_ptr_unchecked(row: *mut [T]) -> Self {
		Self(IterMut::new_ptr_unchecked(row, 1))
	}
}

impl<'a, T> Iterator for IterRowMut<'a, T> {
	type Item = &'a mut T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next()
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.0.len();
		(len, Some(len))
	}
}

impl<'a, T> DoubleEndedIterator for IterRowMut<'a, T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.0.next_back()
	}
}

impl<'a, T> ExactSizeIterator for IterRowMut<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<'a, T> FusedIterator for IterRowMut<'a, T> {}
