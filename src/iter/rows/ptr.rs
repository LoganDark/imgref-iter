use std::iter::FusedIterator;
use std::ops::Range;
use imgref::Img;

use crate::iter::row::*;
use crate::traits::{ImgIterPtr, ImgIterPtrMut};

#[derive(Clone, Debug)]
pub struct IterRowsPtr<T>(pub(crate) Img<*const [T]>, pub(crate) Range<usize>);

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
pub struct IterRowsPtrMut<T>(pub(crate) Img<*mut [T]>, pub(crate) Range<usize>);

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
