use std::marker::PhantomData;

mod ptr;

pub use ptr::*;

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Iter<'a, T>(IterPtr<T>, PhantomData<&'a [T]>);

impl<'a, T> Iter<'a, T> {
	/// Creates a new [`Iter`] over the given slice and stride.
	///
	/// # Panics
	///
	/// Panics if the slice does not start and end on an element; that is,
	/// `slice.len() % stride <= 1`.
	#[inline]
	pub fn new(slice: &'a [T], stride: usize) -> Self {
		assert!(slice.len() % stride <= 1, "slice must start and end on an element");
		unsafe { Self::new_unchecked(slice, stride) }
	}

	/// Creates a new [`Iter`] over the given slice and stride.
	///
	/// # Safety
	///
	/// The given slice must start and end on an element; that is,
	/// `slice.len() % stride <= 1`.
	#[inline]
	pub unsafe fn new_unchecked(slice: &'a [T], stride: usize) -> Self {
		Self(IterPtr::new(slice as *const [T], stride), PhantomData)
	}

	/// Creates a new [`Iter`] over the given slice pointer and stride.
	///
	/// # Panics
	///
	/// Panics if the slice does not start and end on an element; that is,
	/// `slice.len() % stride <= 1`.
	///
	/// # Safety
	///
	/// The given slice must be valid for reads, and remain valid for the
	/// lifetime of the returned [`Iter`].
	#[inline]
	pub unsafe fn new_ptr(slice: *const [T], stride: usize) -> Self {
		assert!((*slice).len() % stride <= 1, "slice must start and end on an element");
		Self::new_ptr_unchecked(slice, stride)
	}

	/// Creates a new [`Iter`] over the given slice pointer and stride.
	///
	/// # Safety
	///
	/// The given slice must be valid for reads, and remain valid for the
	/// lifetime of the returned [`Iter`]. The slice must start and end on an
	/// element; that is, `slice.len() % stride <= 1`.
	#[inline]
	pub unsafe fn new_ptr_unchecked(slice: *const [T], stride: usize) -> Self {
		Self(IterPtr::new(slice, stride), PhantomData)
	}
}

impl<'a, T> Iter<'a, T> {
	/// Returns the number of items left.
	#[inline]
	pub fn len(&self) -> usize {
		unsafe { self.0.len() }
	}

	/// Returns the next item.
	#[inline]
	pub fn next(&mut self) -> Option<&'a T> {
		unsafe { self.0.next().map(|ptr| &*ptr) }
	}

	/// Returns the next item from the back.
	#[inline]
	pub fn next_back(&mut self) -> Option<&'a T> {
		unsafe { self.0.next_back().map(|ptr| &*ptr) }
	}
}

#[repr(transparent)]
#[derive(Eq, PartialEq, Debug)]
pub struct IterMut<'a, T>(IterPtrMut<T>, PhantomData<&'a mut [T]>);

impl<'a, T> IterMut<'a, T> {
	/// Creates a new [`IterMut`] over the given slice and stride.
	///
	/// # Panics
	///
	/// Panics if the slice does not start and end on an element; that is,
	/// `slice.len() % stride <= 1`.
	#[inline]
	pub fn new(slice: &'a mut [T], stride: usize) -> Self {
		assert!(slice.len() % stride <= 1, "slice must start and end on an element");
		unsafe { Self::new_unchecked(slice, stride) }
	}

	/// Creates a new [`IterMut`] over the given slice and stride.
	///
	/// # Safety
	///
	/// The given slice must start and end on an element; that is,
	/// `slice.len() % stride <= 1`.
	#[inline]
	pub unsafe fn new_unchecked(slice: &'a mut [T], stride: usize) -> Self {
		Self(IterPtrMut::new(slice as *mut [T], stride), PhantomData)
	}

	/// Creates a new [`IterMut`] over the given slice pointer and stride.
	///
	/// # Panics
	///
	/// Panics if the slice does not start and end on an element; that is,
	/// `slice.len() % stride <= 1`.
	///
	/// # Safety
	///
	/// The given slice must be valid for reads, and remain valid for the
	/// lifetime of the returned [`IterMut`].
	#[inline]
	pub unsafe fn new_ptr(slice: *mut [T], stride: usize) -> Self {
		assert!((*slice).len() % stride <= 1, "slice must start and end on an element");
		Self::new_ptr_unchecked(slice, stride)
	}

	/// Creates a new [`IterMut`] over the given slice pointer and stride.
	///
	/// # Safety
	///
	/// The given slice must be valid for reads, and remain valid for the
	/// lifetime of the returned [`IterMut`]. The slice must start and end on an
	/// element; that is, `slice.len() % stride <= 1`.
	#[inline]
	pub unsafe fn new_ptr_unchecked(slice: *mut [T], stride: usize) -> Self {
		Self(IterPtrMut::new(slice, stride), PhantomData)
	}
}

impl<'a, T> IterMut<'a, T> {
	/// Returns the number of items left.
	#[inline]
	pub fn len(&self) -> usize {
		unsafe { self.0.len() }
	}

	/// Returns the next item.
	#[inline]
	pub fn next(&mut self) -> Option<&'a mut T> {
		unsafe { self.0.next().map(|ptr| &mut *ptr) }
	}

	/// Returns the next item from the back.
	#[inline]
	pub fn next_back(&mut self) -> Option<&'a mut T> {
		unsafe { self.0.next_back().map(|ptr| &mut *ptr) }
	}
}
