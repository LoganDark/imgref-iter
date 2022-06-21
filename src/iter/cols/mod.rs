use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ops::Range;
use imgref::Img;
use crate::iter::{Iter, IterMut};

mod ptr;

pub use ptr::*;

#[derive(Clone, Debug)]
pub struct IterCols<'a, T>(Img<*const [T]>, Range<usize>, PhantomData<&'a [T]>);

unsafe impl<'a, T: Sync> Send for IterCols<'a, T> {}
unsafe impl<'a, T: Sync> Sync for IterCols<'a, T> {}

impl<'a, T> IterCols<'a, T> {
	/// Creates a new [`IterCols`] over the specified buffer.
	#[inline]
	pub fn new<S: AsRef<[T]>>(buf: &'a Img<S>) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf().as_ref() as *const [T];
		unsafe { Self::new_ptr(Img::new_stride(buf, width, height, stride)) }
	}

	/// Creates a new [`IterCols`] over the specified buffer.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for reads.
	#[inline]
	pub unsafe fn new_ptr(buf: Img<*const [T]>) -> Self {
		Self(buf, 0..buf.width(), PhantomData)
	}
}

impl<'a, T> Iterator for IterCols<'a, T> {
	type Item = Iter<'a, T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|col| unsafe { Iter::col_ptr(self.0, col) })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<'a, T> DoubleEndedIterator for IterCols<'a, T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.1.next_back().map(|col| unsafe { Iter::col_ptr(self.0, col) })
	}
}

impl<'a, T> ExactSizeIterator for IterCols<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.1.len()
	}
}

impl<'a, T> FusedIterator for IterCols<'a, T> {}

#[derive(Debug)]
pub struct IterColsMut<'a, T>(Img<*mut [T]>, Range<usize>, PhantomData<&'a [T]>);

unsafe impl<'a, T: Send> Send for IterColsMut<'a, T> {}
unsafe impl<'a, T: Sync> Sync for IterColsMut<'a, T> {}

impl<'a, T> IterColsMut<'a, T> {
	/// Creates a new [`IterColsMut`] over the specified buffer.
	#[inline]
	pub fn new<S: AsMut<[T]>>(buf: &'a mut Img<S>) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf_mut().as_mut() as *mut [T];
		unsafe { Self::new_ptr(Img::new_stride(buf, width, height, stride)) }
	}

	/// Creates a new [`IterColsMut`] over the specified buffer.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for reads.
	#[inline]
	pub unsafe fn new_ptr(buf: Img<*mut [T]>) -> Self {
		Self(buf, 0..buf.width(), PhantomData)
	}
}

impl<'a, T> Iterator for IterColsMut<'a, T> {
	type Item = IterMut<'a, T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|col| unsafe { IterMut::col_ptr(self.0, col) })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<'a, T> DoubleEndedIterator for IterColsMut<'a, T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.1.next_back().map(|col| unsafe { IterMut::col_ptr(self.0, col) })
	}
}

impl<'a, T> ExactSizeIterator for IterColsMut<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.1.len()
	}
}

impl<'a, T> FusedIterator for IterColsMut<'a, T> {}
