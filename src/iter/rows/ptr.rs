use std::iter::FusedIterator;
use std::ops::Range;
use imgref::Img;
use crate::iter::{IterRowPtr, IterRowPtrMut};
use crate::traits::{ImgIterPtr, ImgIterPtrMut};

#[derive(Clone, Debug)]
pub struct IterRowsPtr<T>(Img<*const [T]>, Range<usize>);

impl<T> IterRowsPtr<T> {
	/// Creates a new [`IterRowsPtr`] over the specified buffer.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`IterRowsPtr`].
	#[inline]
	pub unsafe fn new(buf: &Img<*const [T]>) -> Self {
		Self(Img::new_stride(*buf.buf(), buf.width(), buf.height(), buf.stride()), 0..buf.height())
	}
}

impl<T> Iterator for IterRowsPtr<T> {
	type Item = IterRowPtr<T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|index| unsafe { self.0.iter_row_ptr(index) })
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
		self.1.next_back().map(|index| unsafe { self.0.iter_row_ptr(index) })
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

impl<T> IterRowsPtrMut<T> {
	/// Creates a new [`IterRowsPtrMut`] over the specified buffer.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`IterRowsPtrMut`].
	#[inline]
	pub unsafe fn new(buf: &Img<*mut [T]>) -> Self {
		Self(Img::new_stride(*buf.buf(), buf.width(), buf.height(), buf.stride()), 0..buf.height())
	}
}

impl<T> Iterator for IterRowsPtrMut<T> {
	type Item = IterRowPtrMut<T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|index| unsafe { self.0.iter_row_ptr_mut(index) })
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
		self.1.next_back().map(|index| unsafe { self.0.iter_row_ptr_mut(index) })
	}
}

impl<T> ExactSizeIterator for IterRowsPtrMut<T> {
	#[inline]
	fn len(&self) -> usize {
		self.1.len()
	}
}

impl<T> FusedIterator for IterRowsPtrMut<T> {}
