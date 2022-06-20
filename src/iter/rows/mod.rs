use std::iter::FusedIterator;
use std::ops::Range;
use imgref::Img;
use crate::iter::{IterRow, IterRowMut};

mod ptr;

pub use ptr::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IterRows<'a, T>(Img<&'a [T]>, Range<usize>);

impl<'a, T> IterRows<'a, T> {
	/// Creates a new [`IterRows`] over the specified buffer.
	#[inline]
	pub fn new<S: AsRef<[T]>>(buf: &'a Img<S>) -> Self {
		Self(Img::new_stride(buf.buf().as_ref(), buf.width(), buf.height(), buf.stride()), 0..buf.height())
	}
}

impl<'a, T> Iterator for IterRows<'a, T> {
	type Item = IterRow<'a, T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|index| {
			let offset = self.0.stride() * index;
			let len = self.0.width();
			let row = &self.0.buf()[offset..offset + len];
			IterRow::new_row(row)
		})
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
		self.1.next_back().map(|index| {
			let offset = self.0.stride() * index;
			let len = self.0.width();
			let row = &self.0.buf()[offset..offset + len];
			IterRow::new_row(row)
		})
	}
}

impl<'a, T> ExactSizeIterator for IterRows<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.1.len()
	}
}

impl<'a, T> FusedIterator for IterRows<'a, T> {}

#[derive(Eq, PartialEq, Debug)]
pub struct IterRowsMut<'a, T>(Img<&'a mut [T]>, Range<usize>);

impl<'a, T> IterRowsMut<'a, T> {
	/// Creates a new [`IterRows`] over the specified buffer.
	#[inline]
	pub fn new<S: AsMut<[T]>>(buf: &'a mut Img<S>) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		Self(Img::new_stride(buf.buf_mut().as_mut(), width, height, stride), 0..height)
	}
}

impl<'a, T> Iterator for IterRowsMut<'a, T> {
	type Item = IterRowMut<'a, T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|index| {
			let offset = self.0.stride() * index;
			let len = self.0.width();
			let row = unsafe { &mut *(&mut self.0.buf_mut()[offset..offset + len] as *mut [T]) };
			IterRowMut::new_row(row)
		})
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
		self.1.next_back().map(|index| {
			let offset = self.0.stride() * index;
			let len = self.0.width();
			let row = unsafe { &mut *(&mut self.0.buf_mut()[offset..offset + len] as *mut [T]) };
			IterRowMut::new_row(row)
		})
	}
}

impl<'a, T> ExactSizeIterator for IterRowsMut<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.1.len()
	}
}

impl<'a, T> FusedIterator for IterRowsMut<'a, T> {}
