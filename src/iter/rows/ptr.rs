use std::iter::FusedIterator;
use std::ops::Range;
use imgref::Img;
use crate::iter::{IterPtr, IterPtrMut};

#[derive(Clone, Debug)]
pub struct IterRowsPtr<T>(Img<*const [T]>, Range<usize>);

unsafe impl<T: Sync> Send for IterRowsPtr<T> {}
unsafe impl<T: Sync> Sync for IterRowsPtr<T> {}

impl<T> IterRowsPtr<T> {
	/// Creates a new [`IterRowsPtr`] over the specified buffer.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`IterRowsPtr`].
	#[inline]
	pub unsafe fn new(buf: Img<*const [T]>) -> Self {
		Self(buf, 0..buf.height())
	}
}

impl<T> Iterator for IterRowsPtr<T> {
	type Item = IterPtr<T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|row| unsafe { IterPtr::row_ptr(self.0, row) })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<T> DoubleEndedIterator for IterRowsPtr<T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.1.next_back().map(|row| unsafe { IterPtr::row_ptr(self.0, row) })
	}
}

impl<T> ExactSizeIterator for IterRowsPtr<T> {
	#[inline]
	fn len(&self) -> usize {
		self.1.len()
	}
}

impl<T> FusedIterator for IterRowsPtr<T> {}

#[derive(Clone, Debug)]
pub struct IterRowsPtrMut<T>(Img<*mut [T]>, Range<usize>);

unsafe impl<T: Send> Send for IterRowsPtrMut<T> {}
unsafe impl<T: Sync> Sync for IterRowsPtrMut<T> {}

impl<T> IterRowsPtrMut<T> {
	/// Creates a new [`IterRowsPtrMut`] over the specified buffer.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`IterRowsPtrMut`].
	#[inline]
	pub unsafe fn new(buf: Img<*mut [T]>) -> Self {
		Self(buf, 0..buf.height())
	}
}

impl<T> Iterator for IterRowsPtrMut<T> {
	type Item = IterPtrMut<T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|row| unsafe { IterPtrMut::row_ptr(self.0, row) })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<T> DoubleEndedIterator for IterRowsPtrMut<T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.1.next_back().map(|row| unsafe { IterPtrMut::row_ptr(self.0, row) })
	}
}

impl<T> ExactSizeIterator for IterRowsPtrMut<T> {
	#[inline]
	fn len(&self) -> usize {
		self.1.len()
	}
}

impl<T> FusedIterator for IterRowsPtrMut<T> {}
