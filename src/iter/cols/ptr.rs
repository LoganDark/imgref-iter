use std::iter::FusedIterator;
use std::ops::Range;
use imgref::Img;

use crate::iter::col::*;
use crate::traits::{ImgIterPtr, ImgIterPtrMut};

#[derive(Clone, Debug)]
pub struct IterColsPtr<T>(pub(crate) Img<*const [T]>, pub(crate) Range<usize>);

impl<T> Iterator for IterColsPtr<T> {
	type Item = IterColPtr<T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|index| unsafe { self.0.iter_col_ptr(index) })
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
		self.1.next_back().map(|index| unsafe { self.0.iter_col_ptr(index) })
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
pub struct IterColsPtrMut<T>(pub(crate) Img<*mut [T]>, pub(crate) Range<usize>);

impl<T> Iterator for IterColsPtrMut<T> {
	type Item = IterColPtrMut<T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|index| unsafe { self.0.iter_col_ptr_mut(index) })
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
		self.1.next_back().map(|index| unsafe { self.0.iter_col_ptr_mut(index) })
	}
}

impl<T> ExactSizeIterator for IterColsPtrMut<T> {
	#[inline]
	fn len(&self) -> usize {
		self.1.len()
	}
}

impl<T> FusedIterator for IterColsPtrMut<T> {}
