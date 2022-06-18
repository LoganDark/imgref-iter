use std::iter::FusedIterator;
use std::marker::PhantomData;

mod ptr;

pub use ptr::*;

#[repr(transparent)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IterRow<'a, T>(pub(crate) IterRowPtr<T>, pub(crate) PhantomData<&'a [T]>);

impl<'a, T> Iterator for IterRow<'a, T> {
	type Item = &'a T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|ptr| unsafe { &*ptr })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.0.size_hint()
	}
}

impl<'a, T> DoubleEndedIterator for IterRow<'a, T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.0.next_back().map(|ptr| unsafe { &*ptr })
	}
}

impl<'a, T> ExactSizeIterator for IterRow<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<'a, T> FusedIterator for IterRow<'a, T> {}

#[repr(transparent)]
#[derive(Eq, PartialEq, Debug)]
pub struct IterRowMut<'a, T>(pub(crate) IterRowPtrMut<T>, pub(crate) PhantomData<&'a mut [T]>);

impl<'a, T> Iterator for IterRowMut<'a, T> {
	type Item = &'a mut T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|ptr| unsafe { &mut *ptr })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.0.size_hint()
	}
}

impl<'a, T> DoubleEndedIterator for IterRowMut<'a, T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.0.next_back().map(|ptr| unsafe { &mut *ptr })
	}
}

impl<'a, T> ExactSizeIterator for IterRowMut<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<'a, T> FusedIterator for IterRowMut<'a, T> {}
