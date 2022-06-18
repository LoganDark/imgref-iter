use std::iter::FusedIterator;
use std::marker::PhantomData;

mod ptr;

pub use ptr::*;

#[repr(transparent)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IterCol<'a, T>(pub(crate) IterColPtr<T>, pub(crate) PhantomData<&'a [T]>);

impl<'a, T> Iterator for IterCol<'a, T> {
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

impl<'a, T> DoubleEndedIterator for IterCol<'a, T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.0.next_back().map(|ptr| unsafe { &*ptr })
	}
}

impl<'a, T> ExactSizeIterator for IterCol<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<'a, T> FusedIterator for IterCol<'a, T> {}

#[repr(transparent)]
#[derive(Eq, PartialEq, Debug)]
pub struct IterColMut<'a, T>(pub(crate) IterColPtrMut<T>, pub(crate) PhantomData<&'a mut [T]>);

impl<'a, T> Iterator for IterColMut<'a, T> {
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

impl<'a, T> DoubleEndedIterator for IterColMut<'a, T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.0.next_back().map(|ptr| unsafe { &mut *ptr })
	}
}

impl<'a, T> ExactSizeIterator for IterColMut<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<'a, T> FusedIterator for IterColMut<'a, T> {}
