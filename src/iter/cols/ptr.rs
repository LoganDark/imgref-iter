use std::iter::FusedIterator;
use std::ops::Range;
use imgref::Img;
use crate::iter::{IterPtr, IterPtrMut};

#[derive(Clone, Debug)]
pub struct IterColsPtr<T>(Img<*const [T]>, Range<usize>);

unsafe impl<T: Sync> Send for IterColsPtr<T> {}

unsafe impl<T> Sync for IterColsPtr<T> {}

impl<T> IterColsPtr<T> {
	/// Creates a new [`IterColsPtr`] over the specified buffer.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`IterColsPtr`].
	#[inline]
	pub unsafe fn new(buf: Img<*const [T]>) -> Self {
		Self(buf, 0..buf.width())
	}
}

impl<T> Iterator for IterColsPtr<T> {
	type Item = IterPtr<T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|col| unsafe { IterPtr::col_ptr(self.0, col) })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<T> DoubleEndedIterator for IterColsPtr<T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.1.next_back().map(|col| unsafe { IterPtr::col_ptr(self.0, col) })
	}
}

impl<T> ExactSizeIterator for IterColsPtr<T> {
	#[inline]
	fn len(&self) -> usize {
		self.1.len()
	}
}

impl<T> FusedIterator for IterColsPtr<T> {}

#[derive(Clone, Debug)]
pub struct IterColsPtrMut<T>(Img<*mut [T]>, Range<usize>);

unsafe impl<T: Send> Send for IterColsPtrMut<T> {}

unsafe impl<T> Sync for IterColsPtrMut<T> {}

impl<T> IterColsPtrMut<T> {
	/// Creates a new [`IterColsPtrMut`] over the specified buffer.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`IterColsPtrMut`].
	#[inline]
	pub unsafe fn new(buf: Img<*mut [T]>) -> Self {
		Self(buf, 0..buf.width())
	}
}

impl<T> Iterator for IterColsPtrMut<T> {
	type Item = IterPtrMut<T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|col| unsafe { IterPtrMut::col_ptr(self.0, col) })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<T> DoubleEndedIterator for IterColsPtrMut<T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.1.next_back().map(|col| unsafe { IterPtrMut::col_ptr(self.0, col) })
	}
}

impl<T> ExactSizeIterator for IterColsPtrMut<T> {
	#[inline]
	fn len(&self) -> usize {
		self.1.len()
	}
}

impl<T> FusedIterator for IterColsPtrMut<T> {}
