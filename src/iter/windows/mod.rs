use std::iter::FusedIterator;
use std::marker::PhantomData;
use imgref::Img;
use crate::iter::{Iter, IterMut};

mod ptr;

pub use ptr::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IterWindows<'a, T>(IterWindowsPtr<T>, PhantomData<&'a [T]>);

impl<'a, T> IterWindows<'a, T> {
	/// Wraps an [`IterWindowsPtr`] in an [`IterWindows`].
	///
	/// # Safety
	///
	/// The [`IterWindowsPtr`] must be valid for reads and shared references.
	#[inline]
	pub unsafe fn wrap(ptr: IterWindowsPtr<T>) -> Self {
		Self(ptr, PhantomData)
	}

	/// Creates a new [`IterWindows`] over the rows of an [`Img`].
	pub fn rows<S: AsRef<[T]>>(buf: &'a Img<S>) -> Self {
		unsafe { Self::wrap(IterWindowsPtr::rows(buf)) }
	}

	/// Creates a new [`IterWindows`] over the cols of an [`Img`].
	pub fn cols<S: AsRef<[T]>>(buf: &'a Img<S>) -> Self {
		unsafe { Self::wrap(IterWindowsPtr::cols(buf)) }
	}
}

impl<'a, T> Iterator for IterWindows<'a, T> {
	type Item = Iter<'a, T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|ptr| unsafe { Iter::wrap(ptr) })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<'a, T> DoubleEndedIterator for IterWindows<'a, T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.0.next_back().map(|ptr| unsafe { Iter::wrap(ptr) })
	}
}

impl<'a, T> ExactSizeIterator for IterWindows<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<'a, T> FusedIterator for IterWindows<'a, T> {}

#[derive(Eq, PartialEq, Debug)]
pub struct IterWindowsMut<'a, T>(IterWindowsPtrMut<T>, PhantomData<&'a mut [T]>);

impl<'a, T> IterWindowsMut<'a, T> {
	/// Wraps an [`IterWindowsPtrMut`] in an [`IterWindowsMut`].
	///
	/// # Safety
	///
	/// The [`IterWindowsPtrMut`] must be valid for reads and shared references.
	#[inline]
	pub unsafe fn wrap(ptr: IterWindowsPtrMut<T>) -> Self {
		Self(ptr, PhantomData)
	}

	/// Creates a new [`IterWindowsMut`] over the rows of an [`Img`].
	pub fn rows<S: AsMut<[T]>>(buf: &'a mut Img<S>) -> Self {
		unsafe { Self::wrap(IterWindowsPtrMut::rows(buf)) }
	}

	/// Creates a new [`IterWindowsMut`] over the cols of an [`Img`].
	pub fn cols<S: AsMut<[T]>>(buf: &'a mut Img<S>) -> Self {
		unsafe { Self::wrap(IterWindowsPtrMut::cols(buf)) }
	}
}

impl<'a, T> Iterator for IterWindowsMut<'a, T> {
	type Item = IterMut<'a, T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|ptr| unsafe { IterMut::wrap(ptr) })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<'a, T> DoubleEndedIterator for IterWindowsMut<'a, T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.0.next_back().map(|ptr| unsafe { IterMut::wrap(ptr) })
	}
}

impl<'a, T> ExactSizeIterator for IterWindowsMut<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<'a, T> FusedIterator for IterWindowsMut<'a, T> {}
