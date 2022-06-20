use std::iter::FusedIterator;
use imgref::Img;
use crate::iter::{Iter, IterMut};

mod ptr;

pub use ptr::*;

#[repr(transparent)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IterCol<'a, T>(Iter<'a, T>);

impl<'a, T> IterCol<'a, T> {
	/// Creates a new [`IterCol`] over the specified col of a buffer.
	///
	/// # Panics
	///
	/// Panics if the provided col is out of bounds.
	#[inline]
	pub fn new<S: AsRef<[T]>>(buf: &'a Img<S>, col: usize) -> Self {
		assert!(col < buf.width());
		let height = buf.height();
		let stride = buf.stride();
		let col = &buf.buf().as_ref()[col..col + (height - 1) * stride + 1];
		Self::new_col(col, stride)
	}

	/// Creates a new [`IterCol`] over the provided col and stride.
	///
	/// # Panics
	///
	/// Panics if the slice does not start and end on an element.
	#[inline]
	pub fn new_col(col: &'a [T], stride: usize) -> Self {
		Self(Iter::new(col, stride))
	}

	/// Creates a new [`IterCol`] over the provided col and stride.
	///
	/// # Safety
	///
	/// The slice must start and end on an element.
	#[inline]
	pub unsafe fn new_col_unchecked(col: &'a [T], stride: usize) -> Self {
		Self(Iter::new_unchecked(col, stride))
	}

	/// Creates a new [`IterCol`] over the provided col and stride.
	///
	/// # Panics
	///
	/// Panics if the slice does not start and end on an element.
	///
	/// # Safety
	///
	/// The provided slice must remain valid for the lifetime of this
	/// [`IterCol`].
	#[inline]
	pub unsafe fn new_col_ptr(col: *const [T], stride: usize) -> Self {
		Self(Iter::new_ptr(col, stride))
	}

	/// Creates a new [`IterCol`] over the provided col and stride.
	///
	/// # Safety
	///
	/// The slice must start and end on an element, and the provided slice must
	/// remain valid for the lifetime of this [`IterCol`].
	#[inline]
	pub unsafe fn new_col_ptr_unchecked(col: *const [T], stride: usize) -> Self {
		Self(Iter::new_ptr_unchecked(col, stride))
	}
}

impl<'a, T> Iterator for IterCol<'a, T> {
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

impl<'a, T> DoubleEndedIterator for IterCol<'a, T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.0.next_back()
	}
}

impl<'a, T> ExactSizeIterator for IterCol<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<'a, T> FusedIterator for IterCol<'a, T> {}

#[repr(transparent)]
#[derive(Eq, PartialEq, Debug)]
pub struct IterColMut<'a, T>(IterMut<'a, T>);

impl<'a, T> IterColMut<'a, T> {
	/// Creates a new [`IterColMut`] over the specified col of a buffer.
	///
	/// # Panics
	///
	/// Panics if the provided col is out of bounds.
	#[inline]
	pub fn new<S: AsMut<[T]>>(buf: &'a mut Img<S>, col: usize) -> Self {
		assert!(col < buf.width());
		let height = buf.height();
		let stride = buf.stride();
		let col = &mut buf.buf_mut().as_mut()[col..col + (height - 1) * stride + 1];
		Self::new_col(col, stride)
	}

	/// Creates a new [`IterColMut`] over the provided col and stride.
	///
	/// # Panics
	///
	/// Panics if the slice does not start and end on an element.
	#[inline]
	pub fn new_col(col: &'a mut [T], stride: usize) -> Self {
		Self(IterMut::new(col, stride))
	}

	/// Creates a new [`IterColMut`] over the provided col and stride.
	///
	/// # Safety
	///
	/// The slice must start and end on an element.
	#[inline]
	pub unsafe fn new_col_unchecked(col: &'a mut [T], stride: usize) -> Self {
		Self(IterMut::new_unchecked(col, stride))
	}

	/// Creates a new [`IterColMut`] over the provided col and stride.
	///
	/// # Panics
	///
	/// Panics if the slice does not start and end on an element.
	///
	/// # Safety
	///
	/// The provided slice must remain valid for the lifetime of this
	/// [`IterColMut`].
	#[inline]
	pub unsafe fn new_col_ptr(col: *mut [T], stride: usize) -> Self {
		Self(IterMut::new_ptr(col, stride))
	}

	/// Creates a new [`IterColMut`] over the provided col and stride.
	///
	/// # Safety
	///
	/// The slice must start and end on an element, and the provided slice must
	/// remain valid for the lifetime of this [`IterColMut`].
	#[inline]
	pub unsafe fn new_col_ptr_unchecked(col: *mut [T], stride: usize) -> Self {
		Self(IterMut::new_ptr_unchecked(col, stride))
	}
}

impl<'a, T> Iterator for IterColMut<'a, T> {
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

impl<'a, T> DoubleEndedIterator for IterColMut<'a, T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.0.next_back()
	}
}

impl<'a, T> ExactSizeIterator for IterColMut<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<'a, T> FusedIterator for IterColMut<'a, T> {}
