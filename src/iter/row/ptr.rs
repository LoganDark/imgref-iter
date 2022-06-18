use std::iter::FusedIterator;
use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};

#[repr(transparent)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IterRowPtr<T>(pub(crate) *const [T]);

unsafe impl<T: Sync> Send for IterRowPtr<T> {}

impl<T> Iterator for IterRowPtr<T> {
	type Item = *const T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		Some(unsafe { (*self.0).len() }).filter(|len| *len > 0).map(|len| {
			let first = self.0.cast::<T>();
			self.0 = slice_from_raw_parts(unsafe { first.add(1) }, len - 1);
			first
		})
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<T> DoubleEndedIterator for IterRowPtr<T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		Some(unsafe { (*self.0).len() }).filter(|len| *len > 0).map(|len| {
			let first = self.0.cast::<T>();
			self.0 = slice_from_raw_parts(first, len - 1);
			unsafe { first.add(len - 1) }
		})
	}
}

impl<T> ExactSizeIterator for IterRowPtr<T> {
	#[inline]
	fn len(&self) -> usize {
		unsafe { (*self.0).len() }
	}
}

impl<T> FusedIterator for IterRowPtr<T> {}

#[repr(transparent)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IterRowPtrMut<T>(pub(crate) *mut [T]);

unsafe impl<T: Send> Send for IterRowPtrMut<T> {}

unsafe impl<T: Sync> Sync for IterRowPtrMut<T> {}

impl<T> Iterator for IterRowPtrMut<T> {
	type Item = *mut T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		Some(unsafe { (*self.0).len() }).filter(|len| *len > 0).map(|len| {
			let first = self.0.cast::<T>();
			self.0 = slice_from_raw_parts_mut(unsafe { first.add(1) }, len - 1);
			first
		})
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<T> DoubleEndedIterator for IterRowPtrMut<T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		Some(unsafe { (*self.0).len() }).filter(|len| *len > 0).map(|len| {
			let first = self.0.cast::<T>();
			self.0 = slice_from_raw_parts_mut(first, len - 1);
			unsafe { first.add(len - 1) }
		})
	}
}

impl<T> ExactSizeIterator for IterRowPtrMut<T> {
	#[inline]
	fn len(&self) -> usize {
		unsafe { (*self.0).len() }
	}
}

impl<T> FusedIterator for IterRowPtrMut<T> {}
