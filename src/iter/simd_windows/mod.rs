use core::iter::FusedIterator;
use core::marker::PhantomData;
use imgref::Img;
use crate::iter::{Iter, IterMut, SimdIter, SimdIterMut};

mod ptr;

pub use ptr::*;

#[repr(transparent)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SimdIterWindows<'a, T, const LANES: usize>(SimdIterWindowsPtr<T, LANES>, PhantomData<&'a [T]>);

impl<'a, T, const LANES: usize> SimdIterWindows<'a, T, LANES> {
	/// Wraps an [`SimdIterWindowsPtr`] in an [`SimdIterWindows`].
	///
	/// # Safety
	///
	/// The [`SimdIterWindowsPtr`] must be valid for reads and shared references.
	#[inline]
	pub unsafe fn wrap(ptr: SimdIterWindowsPtr<T, LANES>) -> Self {
		Self(ptr, PhantomData)
	}

	/// Creates a new [`SimdIterWindows`] over the rows of an [`Img`].
	#[inline]
	pub fn rows<S: AsRef<[T]>>(buf: &'a Img<S>) -> Self {
		unsafe { Self::wrap(SimdIterWindowsPtr::rows(buf)) }
	}

	/// Creates a new [`SimdIterWindows`] over the cols of an [`Img`].
	#[inline]
	pub fn cols<S: AsRef<[T]>>(buf: &'a Img<S>) -> Self {
		unsafe { Self::wrap(SimdIterWindowsPtr::cols(buf)) }
	}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SimdIterWindow<'a, T, const LANES: usize> {
	Simd(SimdIter<'a, T, LANES>),
	Single(Iter<'a, T>)
}

impl<'a, T, const LANES: usize> SimdIterWindow<'a, T, LANES> {
	#[inline]
	pub unsafe fn wrap(other: SimdIterWindowPtr<T, LANES>) -> Self {
		match other {
			SimdIterWindowPtr::Simd(simd) => Self::Simd(SimdIter::wrap(simd)),
			SimdIterWindowPtr::Single(iter) => Self::Single(Iter::wrap(iter))
		}
	}
}

impl<'a, T, const LANES: usize> Iterator for SimdIterWindows<'a, T, LANES> {
	type Item = SimdIterWindow<'a, T, LANES>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|window| unsafe { SimdIterWindow::wrap(window) })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<'a, T, const LANES: usize> DoubleEndedIterator for SimdIterWindows<'a, T, LANES> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.0.next_back().map(|window| unsafe { SimdIterWindow::wrap(window) })
	}
}

impl<'a, T, const LANES: usize> ExactSizeIterator for SimdIterWindows<'a, T, LANES> {
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<'a, T, const LANES: usize> FusedIterator for SimdIterWindows<'a, T, LANES> {}

#[repr(transparent)]
#[derive(Eq, PartialEq, Debug)]
pub struct SimdIterWindowsMut<'a, T, const LANES: usize>(SimdIterWindowsPtrMut<T, LANES>, PhantomData<&'a [T]>);

impl<'a, T, const LANES: usize> SimdIterWindowsMut<'a, T, LANES> {
	/// Wraps an [`SimdIterWindowsPtrMut`] in an [`SimdIterWindowsMut`].
	///
	/// # Safety
	///
	/// The [`SimdIterWindowsPtrMut`] must be valid for reads and shared references.
	#[inline]
	pub unsafe fn wrap(ptr: SimdIterWindowsPtrMut<T, LANES>) -> Self {
		Self(ptr, PhantomData)
	}

	/// Creates a new [`SimdIterWindowsMut`] over the rows of an [`Img`].
	#[inline]
	pub fn rows<S: AsMut<[T]>>(buf: &'a mut Img<S>) -> Self {
		unsafe { Self::wrap(SimdIterWindowsPtrMut::rows(buf)) }
	}

	/// Creates a new [`SimdIterWindowsMut`] over the cols of an [`Img`].
	#[inline]
	pub fn cols<S: AsMut<[T]>>(buf: &'a mut Img<S>) -> Self {
		unsafe { Self::wrap(SimdIterWindowsPtrMut::cols(buf)) }
	}
}

#[derive(Eq, PartialEq, Debug)]
pub enum SimdIterWindowMut<'a, T, const LANES: usize> {
	Simd(SimdIterMut<'a, T, LANES>),
	Single(IterMut<'a, T>)
}

impl<'a, T, const LANES: usize> SimdIterWindowMut<'a, T, LANES> {
	#[inline]
	pub unsafe fn wrap(other: SimdIterWindowPtrMut<T, LANES>) -> Self {
		match other {
			SimdIterWindowPtrMut::Simd(simd) => Self::Simd(SimdIterMut::wrap(simd)),
			SimdIterWindowPtrMut::Single(iter) => Self::Single(IterMut::wrap(iter))
		}
	}
}

impl<'a, T, const LANES: usize> Iterator for SimdIterWindowsMut<'a, T, LANES> {
	type Item = SimdIterWindowMut<'a, T, LANES>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|window| unsafe { SimdIterWindowMut::wrap(window) })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<'a, T, const LANES: usize> DoubleEndedIterator for SimdIterWindowsMut<'a, T, LANES> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.0.next_back().map(|window| unsafe { SimdIterWindowMut::wrap(window) })
	}
}

impl<'a, T, const LANES: usize> ExactSizeIterator for SimdIterWindowsMut<'a, T, LANES> {
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<'a, T, const LANES: usize> FusedIterator for SimdIterWindowsMut<'a, T, LANES> {}
