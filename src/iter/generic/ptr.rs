use std::cmp::min;
use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct IterPtr<T>(*const [T], usize);

unsafe impl<T: Sync> Send for IterPtr<T> {}

unsafe impl<T: Sync> Sync for IterPtr<T> {}

impl<T> IterPtr<T> {
	/// Creates a new [`IterPtr`] over the specified slice and stride.
	#[inline]
	pub fn new(slice: *const [T], stride: usize) -> Self {
		Self(slice, stride)
	}

	/// Returns the number of items left.
	///
	/// # Safety
	///
	/// The slice pointer must still be valid.
	#[inline]
	pub unsafe fn len(&self) -> usize {
		let len = (*self.0).len();
		(len + (self.1 - 1)) / self.1
	}

	/// Returns the next item.
	///
	/// # Safety
	///
	/// The slice pointer must still be valid.
	#[inline]
	pub unsafe fn next(&mut self) -> Option<*const T> {
		let len = (*self.0).len();

		if len > 0 {
			let first = self.0.cast::<T>();
			self.0 = slice_from_raw_parts(first.add(min(self.1, len)), len.saturating_sub(self.1));
			Some(first)
		} else {
			None
		}
	}

	/// Returns the next item from the back.
	///
	/// # Safety
	///
	/// The slice pointer must still be valid.
	#[inline]
	pub unsafe fn next_back(&mut self) -> Option<*const T> {
		let len = (*self.0).len();

		if len > 0 {
			let first = self.0.cast::<T>();
			self.0 = slice_from_raw_parts(first, len.saturating_sub(self.1));
			Some(first.add(len - 1))
		} else {
			None
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct IterPtrMut<T>(*mut [T], usize);

unsafe impl<T: Send> Send for IterPtrMut<T> {}

unsafe impl<T: Sync> Sync for IterPtrMut<T> {}

impl<T> IterPtrMut<T> {
	/// Creates a new [`IterPtrMut`] over the specified slice and stride.
	#[inline]
	pub fn new(slice: *mut [T], stride: usize) -> Self {
		Self(slice, stride)
	}

	/// Returns the number of items left.
	///
	/// # Safety
	///
	/// The slice pointer must still be valid.
	#[inline]
	pub unsafe fn len(&self) -> usize {
		let len = (*self.0).len();
		(len + (self.1 - 1)) / self.1
	}

	/// Returns the next item.
	///
	/// # Safety
	///
	/// The slice pointer must still be valid.
	#[inline]
	pub unsafe fn next(&mut self) -> Option<*mut T> {
		let len = (*self.0).len();

		if len > 0 {
			let first = self.0.cast::<T>();
			self.0 = slice_from_raw_parts_mut(first.add(min(self.1, len)), len.saturating_sub(self.1));
			Some(first)
		} else {
			None
		}
	}

	/// Returns the next item from the back.
	///
	/// # Safety
	///
	/// The slice pointer must still be valid.
	#[inline]
	pub unsafe fn next_back(&mut self) -> Option<*mut T> {
		let len = (*self.0).len();

		if len > 0 {
			let first = self.0.cast::<T>();
			self.0 = slice_from_raw_parts_mut(first, len.saturating_sub(self.1));
			Some(first.add(len - 1))
		} else {
			None
		}
	}
}
