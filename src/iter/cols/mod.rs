use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ops::Range;
use imgref::Img;

use crate::iter::col::*;
use crate::traits::{ImgIterPtr, ImgIterPtrMut};

mod ptr;

pub use ptr::*;

#[derive(Clone, Debug)]
pub struct IterCols<'a, T>(pub(crate) Img<*const [T]>, pub(crate) Range<usize>, pub(crate) PhantomData<&'a [T]>);

impl<'a, T> Iterator for IterCols<'a, T> {
	type Item = IterCol<'a, T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|index| IterCol(unsafe { self.0.iter_col_ptr(index) }, PhantomData))
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
		self.1.next_back().map(|index| IterCol(unsafe { self.0.iter_col_ptr(index) }, PhantomData))
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
pub struct IterColsMut<'a, T>(pub(crate) Img<*mut [T]>, pub(crate) Range<usize>, pub(crate) PhantomData<&'a mut [T]>);

impl<'a, T> Iterator for IterColsMut<'a, T> {
	type Item = IterColMut<'a, T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|index| IterColMut(unsafe { self.0.iter_col_ptr_mut(index) }, PhantomData))
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
		self.1.next_back().map(|index| IterColMut(unsafe { self.0.iter_col_ptr_mut(index) }, PhantomData))
	}
}

impl<'a, T> ExactSizeIterator for IterColsMut<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.1.len()
	}
}

impl<'a, T> FusedIterator for IterColsMut<'a, T> {}
