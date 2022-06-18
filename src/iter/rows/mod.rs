use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ops::Range;
use imgref::Img;

use crate::iter::row::*;
use crate::traits::{ImgIterPtr, ImgIterPtrMut};

mod ptr;

pub use ptr::*;

#[derive(Clone, Debug)]
pub struct IterRows<'a, T>(pub(crate) Img<*const [T]>, pub(crate) Range<usize>, pub(crate) PhantomData<&'a [T]>);

impl<'a, T> Iterator for IterRows<'a, T> {
	type Item = IterRow<'a, T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|index| IterRow(unsafe { self.0.iter_row_ptr(index) }, PhantomData))
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
		self.1.next_back().map(|index| IterRow(unsafe { self.0.iter_row_ptr(index) }, PhantomData))
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
pub struct IterRowsMut<'a, T>(pub(crate) Img<*mut [T]>, pub(crate) Range<usize>, pub(crate) PhantomData<&'a mut [T]>);

impl<'a, T> Iterator for IterRowsMut<'a, T> {
	type Item = IterRowMut<'a, T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next().map(|index| IterRowMut(unsafe { self.0.iter_row_ptr_mut(index) }, PhantomData))
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
		self.1.next_back().map(|index| IterRowMut(unsafe { self.0.iter_row_ptr_mut(index) }, PhantomData))
	}
}

impl<'a, T> ExactSizeIterator for IterRowsMut<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.1.len()
	}
}

impl<'a, T> FusedIterator for IterRowsMut<'a, T> {}
