use core::iter::FusedIterator;
use core::marker::PhantomData;
use imgref::Img;
use crate::iter::{Iter, IterMut, IterPtr, IterPtrMut};

mod ptr;

pub use ptr::*;

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SimdIter<'a, T, const LANES: usize>(SimdIterPtr<T, LANES>, PhantomData<&'a [T]>);

#[allow(clippy::missing_safety_doc)]
impl<'a, T, const LANES: usize> SimdIter<'a, T, LANES> {
	/// Wraps a [`SimdIterPtr`] in a [`SimdIter`].
	///
	/// # Safety
	///
	/// The [`SimdIterPtr`] must be valid for reads and shared references.
	#[inline]
	pub unsafe fn wrap(ptr: SimdIterPtr<T, LANES>) -> Self {
		Self(ptr, PhantomData)
	}

	/// Creates a new [`SimdIter`] from the given [`Iter`] and gap.
	///
	/// The gap is the distance between successive items in the returned arrays.
	/// For example if the iterator is over a row and the gap is the stride,
	/// then this iterator will yield items from multiple rows at a time.
	///
	/// # Safety
	///
	/// The given gap must be valid.
	#[inline]
	pub unsafe fn new(iter: Iter<'a, T>, gap: usize) -> Self {
		Self::wrap(SimdIterPtr::new(iter.into_inner(), gap))
	}

	/// Creates a new [`SimdIter`] from the given [`IterPtr`] and gap.
	///
	/// The gap is the distance between successive items in the returned arrays.
	/// For example if the iterator is over a row and the gap is the stride,
	/// then this iterator will yield items from multiple rows at a time.
	///
	/// # Safety
	///
	/// The given iterator must be valid and the gap must be valid.
	#[inline]
	pub unsafe fn new_ptr(iter: IterPtr<T>, gap: usize) -> Self {
		Self::wrap(SimdIterPtr::new(iter, gap))
	}

	/// Creates a new [`SimdIter`] across `LANES` rows.
	///
	/// # Panics
	///
	/// Panics if the given `row + LANES > buf.height()`.
	#[inline]
	pub fn rows<S: AsRef<[T]>>(buf: &Img<S>, row: usize) -> Self {
		unsafe { Self::wrap(SimdIterPtr::rows(buf, row)) }
	}

	/// Creates a new [`SimdIter`] across `LANES` rows.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIter`].
	///
	/// # Panics
	///
	/// Panics if the given `row + LANES > buf.height()`.
	#[inline]
	pub unsafe fn rows_ptr(buf: Img<*const [T]>, row: usize) -> Self {
		Self::wrap(SimdIterPtr::rows_ptr(buf, row))
	}

	/// Creates a new [`SimdIter`] across `LANES` rows.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIter`].
	///
	/// The caller must ensure that `row + LANES > buf.height()`.
	#[inline]
	pub unsafe fn rows_ptr_unchecked(buf: Img<*const [T]>, row: usize) -> Self {
		Self::wrap(SimdIterPtr::rows_ptr_unchecked(buf, row))
	}

	/// Creates a new [`SimdIter`] across `LANES` cols.
	///
	/// # Panics
	///
	/// Panics if the given `col + LANES > buf.width()`.
	#[inline]
	pub fn cols<S: AsRef<[T]>>(buf: &Img<S>, col: usize) -> Self {
		unsafe { Self::wrap(SimdIterPtr::cols(buf, col)) }
	}

	/// Creates a new [`SimdIter`] across `LANES` cols.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIter`].
	///
	/// # Panics
	///
	/// Panics if the given `col + LANES > buf.width()`.
	#[inline]
	pub unsafe fn cols_ptr(buf: Img<*const [T]>, col: usize) -> Self {
		Self::wrap(SimdIterPtr::cols_ptr(buf, col))
	}

	/// Creates a new [`SimdIter`] across `LANES` cols.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIter`].
	///
	/// The caller must ensure that `col + LANES > buf.width()`.
	#[inline]
	pub unsafe fn cols_ptr_unchecked(buf: Img<*const [T]>, col: usize) -> Self {
		Self::wrap(SimdIterPtr::cols_ptr_unchecked(buf, col))
	}

	/// Converts this [`SimdIter`] into its inner [`SimdIterPtr`].
	#[inline]
	pub fn into_inner(self) -> SimdIterPtr<T, LANES> {
		self.0
	}
}

impl<'a, T, const LANES: usize> Iterator for SimdIter<'a, T, LANES> {
	type Item = [&'a T; LANES];

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|arr| arr.map(|ptr| unsafe { &*ptr }))
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.0.size_hint()
	}
}

impl<'a, T, const LANES: usize> DoubleEndedIterator for SimdIter<'a, T, LANES> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.0.next_back().map(|arr| arr.map(|ptr| unsafe { &*ptr }))
	}
}

impl<'a, T, const LANES: usize> ExactSizeIterator for SimdIter<'a, T, LANES> {
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<'a, T, const LANES: usize> FusedIterator for SimdIter<'a, T, LANES> {}

#[repr(transparent)]
#[derive(Eq, PartialEq, Debug)]
pub struct SimdIterMut<'a, T, const LANES: usize>(SimdIterPtrMut<T, LANES>, PhantomData<&'a mut [T]>);

#[allow(clippy::missing_safety_doc)]
impl<'a, T, const LANES: usize> SimdIterMut<'a, T, LANES> {
	/// Wraps a [`SimdIterPtrMut`] in a [`SimdIterMut`].
	///
	/// # Safety
	///
	/// The [`SimdIterPtrMut`] must be valid for reads and writes.
	#[inline]
	pub unsafe fn wrap(ptr: SimdIterPtrMut<T, LANES>) -> Self {
		Self(ptr, PhantomData)
	}

	/// Creates a new [`SimdIterMut`] from the given [`Iter`] and gap.
	///
	/// The gap is the distance between successive items in the returned arrays.
	/// For example if the iterator is over a row and the gap is the stride,
	/// then this iterator will yield items from multiple rows at a time.
	///
	/// # Safety
	///
	/// The given gap must be valid.
	#[inline]
	pub unsafe fn new(iter: IterMut<'a, T>, gap: usize) -> Self {
		Self::wrap(SimdIterPtrMut::new(iter.into_inner(), gap))
	}

	/// Creates a new [`SimdIterMut`] from the given [`IterPtr`] and gap.
	///
	/// The gap is the distance between successive items in the returned arrays.
	/// For example if the iterator is over a row and the gap is the stride,
	/// then this iterator will yield items from multiple rows at a time.
	///
	/// # Safety
	///
	/// The given iterator must be valid and the gap must be valid.
	#[inline]
	pub unsafe fn new_ptr(iter: IterPtrMut<T>, gap: usize) -> Self {
		Self::wrap(SimdIterPtrMut::new(iter, gap))
	}

	/// Creates a new [`SimdIterMut`] across `LANES` rows.
	///
	/// # Panics
	///
	/// Panics if the given `row + LANES > buf.height()`.
	#[inline]
	pub fn rows<S: AsMut<[T]>>(buf: &mut Img<S>, row: usize) -> Self {
		unsafe { Self::wrap(SimdIterPtrMut::rows(buf, row)) }
	}

	/// Creates a new [`SimdIterMut`] across `LANES` rows.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIterMut`].
	///
	/// # Panics
	///
	/// Panics if the given `row + LANES > buf.height()`.
	#[inline]
	pub unsafe fn rows_ptr(buf: Img<*mut [T]>, row: usize) -> Self {
		Self::wrap(SimdIterPtrMut::rows_ptr(buf, row))
	}

	/// Creates a new [`SimdIterMut`] across `LANES` rows.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIterMut`].
	///
	/// The caller must ensure that `row + LANES > buf.height()`.
	#[inline]
	pub unsafe fn rows_ptr_unchecked(buf: Img<*mut [T]>, row: usize) -> Self {
		Self::wrap(SimdIterPtrMut::rows_ptr_unchecked(buf, row))
	}

	/// Creates a new [`SimdIterMut`] across `LANES` cols.
	///
	/// # Panics
	///
	/// Panics if the given `col + LANES > buf.width()`.
	#[inline]
	pub fn cols<S: AsMut<[T]>>(buf: &mut Img<S>, col: usize) -> Self {
		unsafe { Self::wrap(SimdIterPtrMut::cols(buf, col)) }
	}

	/// Creates a new [`SimdIterMut`] across `LANES` cols.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIterMut`].
	///
	/// # Panics
	///
	/// Panics if the given `col + LANES > buf.width()`.
	#[inline]
	pub unsafe fn cols_ptr(buf: Img<*mut [T]>, col: usize) -> Self {
		Self::wrap(SimdIterPtrMut::cols_ptr(buf, col))
	}

	/// Creates a new [`SimdIterMut`] across `LANES` cols.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIterMut`].
	///
	/// The caller must ensure that `col + LANES > buf.width()`.
	#[inline]
	pub unsafe fn cols_ptr_unchecked(buf: Img<*mut [T]>, col: usize) -> Self {
		Self::wrap(SimdIterPtrMut::cols_ptr_unchecked(buf, col))
	}

	/// Converts this [`SimdIterMut`] into its inner [`SimdIterPtrMut`].
	#[inline]
	pub fn into_inner(self) -> SimdIterPtrMut<T, LANES> {
		self.0
	}
}

impl<'a, T, const LANES: usize> Iterator for SimdIterMut<'a, T, LANES> {
	type Item = [&'a mut T; LANES];

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|arr| arr.map(|ptr| unsafe { &mut *ptr }))
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.0.size_hint()
	}
}

impl<'a, T, const LANES: usize> DoubleEndedIterator for SimdIterMut<'a, T, LANES> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.0.next_back().map(|arr| arr.map(|ptr| unsafe { &mut *ptr }))
	}
}

impl<'a, T, const LANES: usize> ExactSizeIterator for SimdIterMut<'a, T, LANES> {
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<'a, T, const LANES: usize> FusedIterator for SimdIterMut<'a, T, LANES> {}
