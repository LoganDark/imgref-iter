use std::iter::FusedIterator;
use std::ops::Range;
use imgref::Img;
use crate::iter::{IterCol, IterColMut};

mod ptr;

pub use ptr::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IterCols<'a, T>(Img<&'a [T]>, Range<usize>);

impl<'a, T> IterCols<'a, T> {
	/// Creates a new [`IterCols`] over the specified buffer.
	#[inline]
	pub fn new<S: AsRef<[T]>>(buf: &'a Img<S>) -> Self {
		Self(Img::new_stride(buf.buf().as_ref(), buf.width(), buf.height(), buf.stride()), 0..buf.width())
	}
}

impl<'a, T> Iterator for IterCols<'a, T> {
	type Item = IterCol<'a, T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|index| {
			let height = self.0.height();
			let stride = self.0.stride();
			let col = &self.0.buf()[index .. index + (height - 1) * stride + 1];
			IterCol::new_col(col, stride)
		})
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
		self.1.next_back().map(|index| {
			let height = self.0.height();
			let stride = self.0.stride();
			let col = &self.0.buf()[index .. index + (height - 1) * stride + 1];
			IterCol::new_col(col, stride)
		})
	}
}

impl<'a, T> ExactSizeIterator for IterCols<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.1.len()
	}
}

impl<'a, T> FusedIterator for IterCols<'a, T> {}

#[derive(Eq, PartialEq, Debug)]
pub struct IterColsMut<'a, T>(Img<&'a mut [T]>, Range<usize>);

impl<'a, T> IterColsMut<'a, T> {
	/// Creates a new [`IterCols`] over the specified buffer.
	#[inline]
	pub fn new<S: AsMut<[T]>>(buf: &'a mut Img<S>) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		Self(Img::new_stride(buf.buf_mut().as_mut(), width, height, stride), 0..width)
	}
}

impl<'a, T> Iterator for IterColsMut<'a, T> {
	type Item = IterColMut<'a, T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|index| {
			let height = self.0.height();
			let stride = self.0.stride();
			let col = unsafe { &mut *(&mut self.0.buf_mut()[index .. index + (height - 1) * stride + 1] as *mut [T]) };
			IterColMut::new_col(col, stride)
		})
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
		self.1.next_back().map(|index| {
			let height = self.0.height();
			let stride = self.0.stride();
			let col = unsafe { &mut *(&mut self.0.buf_mut()[index .. index + (height - 1) * stride + 1] as *mut [T]) };
			IterColMut::new_col(col, stride)
		})
	}
}

impl<'a, T> ExactSizeIterator for IterColsMut<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.1.len()
	}
}

impl<'a, T> FusedIterator for IterColsMut<'a, T> {}
