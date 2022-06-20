use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ops::Range;
use imgref::Img;
use crate::iter::{Iter, IterMut};

mod ptr;

pub use ptr::*;

#[derive(Clone, Debug)]
pub struct IterRows<'a, T>(Img<*const [T]>, Range<usize>, PhantomData<&'a [T]>);

impl<'a, T> IterRows<'a, T> {
	/// Creates a new [`IterRows`] over the specified buffer.
	#[inline]
	pub fn new<S: AsRef<[T]>>(buf: &'a Img<S>) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf().as_ref() as *const [T];
		unsafe { Self::new_ptr(Img::new_stride(buf, width, height, stride)) }
	}

	/// Creates a new [`IterRows`] over the specified buffer.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for reads.
	#[inline]
	pub unsafe fn new_ptr(buf: Img<*const [T]>) -> Self {
		Self(buf, 0..buf.height(), PhantomData)
	}
}

impl<'a, T> Iterator for IterRows<'a, T> {
	type Item = Iter<'a, T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|row| unsafe { Iter::row_ptr(self.0, row) })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<'a, T> DoubleEndedIterator for IterRows<'a, T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.1.next_back().map(|row| unsafe { Iter::row_ptr(self.0, row) })
	}
}

impl<'a, T> ExactSizeIterator for IterRows<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.1.len()
	}
}

impl<'a, T> FusedIterator for IterRows<'a, T> {}

#[derive(Debug)]
pub struct IterRowsMut<'a, T>(Img<*mut [T]>, Range<usize>, PhantomData<&'a [T]>);

impl<'a, T> IterRowsMut<'a, T> {
	/// Creates a new [`IterRowsMut`] over the specified buffer.
	#[inline]
	pub fn new<S: AsMut<[T]>>(buf: &'a mut Img<S>) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf_mut().as_mut() as *mut [T];
		unsafe { Self::new_ptr(Img::new_stride(buf, width, height, stride)) }
	}

	/// Creates a new [`IterRowsMut`] over the specified buffer.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for reads.
	#[inline]
	pub unsafe fn new_ptr(buf: Img<*mut [T]>) -> Self {
		Self(buf, 0..buf.height(), PhantomData)
	}
}

impl<'a, T> Iterator for IterRowsMut<'a, T> {
	type Item = IterMut<'a, T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|row| unsafe { IterMut::row_ptr(self.0, row) })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<'a, T> DoubleEndedIterator for IterRowsMut<'a, T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.1.next_back().map(|row| unsafe { IterMut::row_ptr(self.0, row) })
	}
}

impl<'a, T> ExactSizeIterator for IterRowsMut<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.1.len()
	}
}

impl<'a, T> FusedIterator for IterRowsMut<'a, T> {}
