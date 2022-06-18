use std::cmp::min;
use std::iter::FusedIterator;
use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IterColPtr<T>(pub(crate) *const [T], pub(crate) usize);

unsafe impl<T: Sync> Send for IterColPtr<T> {}

impl<T> Iterator for IterColPtr<T> {
	type Item = *const T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		Some(unsafe { (*self.0).len() }).filter(|len| *len > 0).map(|len| {
			let first = self.0.cast::<T>();
			self.0 = slice_from_raw_parts(unsafe { first.add(min(self.1, len)) }, len.saturating_sub(self.1));
			first
		})
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<T> DoubleEndedIterator for IterColPtr<T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		Some(unsafe { (*self.0).len() }).filter(|len| *len > 0).map(|len| {
			let first = self.0.cast::<T>();
			self.0 = slice_from_raw_parts(first, len.saturating_sub(self.1));
			unsafe { first.add(len - 1) }
		})
	}
}

impl<T> ExactSizeIterator for IterColPtr<T> {
	#[inline]
	fn len(&self) -> usize {
		unsafe { ((*self.0).len() + (self.1 - 1)) / self.1 }
	}
}

impl<T> FusedIterator for IterColPtr<T> {}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IterColPtrMut<T>(pub(crate) *mut [T], pub(crate) usize);

unsafe impl<T: Send> Send for IterColPtrMut<T> {}

unsafe impl<T: Sync> Sync for IterColPtrMut<T> {}

impl<T> Iterator for IterColPtrMut<T> {
	type Item = *mut T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		Some(unsafe { (*self.0).len() }).filter(|len| *len > 0).map(|len| {
			let first = self.0.cast::<T>();
			self.0 = slice_from_raw_parts_mut(unsafe { first.add(min(self.1, len)) }, len.saturating_sub(self.1));
			first
		})
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<T> DoubleEndedIterator for IterColPtrMut<T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		Some(unsafe { (*self.0).len() }).filter(|len| *len > 0).map(|len| {
			let first = self.0.cast::<T>();
			self.0 = slice_from_raw_parts_mut(first, len.saturating_sub(self.1));
			unsafe { first.add(len - 1) }
		})
	}
}

impl<T> ExactSizeIterator for IterColPtrMut<T> {
	#[inline]
	fn len(&self) -> usize {
		unsafe { ((*self.0).len() + (self.1 - 1)) / self.1 }
	}
}

impl<T> FusedIterator for IterColPtrMut<T> {}
