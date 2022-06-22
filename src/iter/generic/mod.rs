use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
use imgref::Img;

mod ptr;

pub use ptr::*;

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Iter<'a, T>(IterPtr<T>, PhantomData<&'a [T]>);

impl<'a, T> Iter<'a, T> {
	/// Wraps an [`IterPtr`] in an [`Iter`].
	///
	/// # Safety
	///
	/// The [`IterPtr`] must be valid for reads and shared references.
	#[inline]
	pub unsafe fn wrap(ptr: IterPtr<T>) -> Self {
		Self(ptr, PhantomData)
	}

	/// Creates a new [`Iter`] over the specified slice and stride.
	///
	/// # Panics
	///
	/// Panics if the given slice does not start and end at an element. That is,
	/// both the first and last elements of the slice must be elements that
	/// would be returned by this iterator. Do not include trailing stride.
	#[inline]
	pub fn new(slice: &[T], stride: usize) -> Self {
		unsafe { Self::new_ptr(slice as *const [T], stride) }
	}

	/// Creates a new [`Iter`] over the specified slice and stride.
	///
	/// # Safety
	///
	/// UB if the given slice does not start and end at an element. That is,
	/// both the first and last elements of the slice must be elements that
	/// would be returned by this iterator. Do not include trailing stride.
	#[inline]
	pub unsafe fn new_unchecked(slice: &[T], stride: usize) -> Self {
		Self::new_ptr_unchecked(slice as *const [T], stride)
	}

	/// Creates a new [`Iter`] over the specified slice and stride.
	///
	/// # Panics
	///
	/// Panics if the given slice does not start and end at an element. That is,
	/// both the first and last elements of the slice must be elements that
	/// would be returned by this iterator. Do not include trailing stride.
	#[inline]
	pub unsafe fn new_ptr(slice: *const [T], stride: usize) -> Self {
		assert!(IterPtr::is_slice_perfect((*slice).len(), stride));
		Self::new_ptr_unchecked(slice, stride)
	}

	/// Creates a new [`Iter`] over the specified slice and stride.
	///
	/// # Safety
	///
	/// UB if the given slice does not start and end at an element. That is,
	/// both the first and last elements of the slice must be elements that
	/// would be returned by this iterator. Do not include trailing stride.
	#[inline]
	pub unsafe fn new_ptr_unchecked(slice: *const [T], stride: usize) -> Self {
		Self::wrap(IterPtr::new(slice, stride))
	}

	/// Creates a new [`Iter`] over the specified buffer row.
	///
	/// # Panics
	///
	/// Panics if the given row is out of bounds.
	#[inline]
	pub fn row<S: AsRef<[T]>>(buf: &'a Img<S>, row: usize) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf().as_ref() as *const [T];
		unsafe { Self::row_ptr(Img::new_stride(buf, width, height, stride), row) }
	}

	/// Creates a new [`Iter`] over the specified buffer row.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for reads.
	///
	/// # Panics
	///
	/// Panics if the given row is out of bounds.
	#[inline]
	pub unsafe fn row_ptr(buf: Img<*const [T]>, row: usize) -> Self {
		assert!(row < buf.height());
		Self::row_ptr_unchecked(buf, row)
	}

	/// Creates a new [`Iter`] over the specified buffer row.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for reads, and the given row must not
	/// be out of bounds.
	#[inline]
	pub unsafe fn row_ptr_unchecked(buf: Img<*const [T]>, row: usize) -> Self {
		let slice = {
			let data = buf.buf().cast::<T>().add(row * buf.stride());
			let len = buf.width();
			slice_from_raw_parts(data, len)
		};

		Self::new_ptr_unchecked(slice, 1)
	}

	/// Creates a new [`Iter`] over the specified buffer col.
	///
	/// # Panics
	///
	/// Panics if the given col is out of bounds.
	#[inline]
	pub fn col<S: AsRef<[T]>>(buf: &'a Img<S>, col: usize) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf().as_ref() as *const [T];
		unsafe { Self::col_ptr(Img::new_stride(buf, width, height, stride), col) }
	}

	/// Creates a new [`Iter`] over the specified buffer col.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for reads.
	///
	/// # Panics
	///
	/// Panics if the given col is out of bounds.
	#[inline]
	pub unsafe fn col_ptr(buf: Img<*const [T]>, col: usize) -> Self {
		assert!(col < buf.width());
		Self::col_ptr_unchecked(buf, col)
	}

	/// Creates a new [`Iter`] over the specified buffer col.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for reads, and the given col must not
	/// be out of bounds.
	#[inline]
	pub unsafe fn col_ptr_unchecked(buf: Img<*const [T]>, col: usize) -> Self {
		let slice = {
			let data = buf.buf().cast::<T>().add(col);
			let len = buf.stride() * (buf.height() - 1) + 1;
			slice_from_raw_parts(data, len)
		};

		Self::new_ptr_unchecked(slice, buf.stride())
	}

	/// Converts this [`Iter`] into its inner [`IterPtr`].
	pub fn into_inner(self) -> IterPtr<T> {
		self.0
	}
}

impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|ptr| unsafe { &*ptr })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.0.next_back().map(|ptr| unsafe { &*ptr })
	}
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<'a, T> FusedIterator for Iter<'a, T> {}

#[repr(transparent)]
#[derive(Eq, PartialEq, Debug)]
pub struct IterMut<'a, T>(IterPtrMut<T>, PhantomData<&'a mut [T]>);

impl<'a, T> IterMut<'a, T> {
	/// Wraps an [`IterPtrMut`] in an [`IterMut`].
	///
	/// # Safety
	///
	/// The [`IterPtrMut`] must be valid for reads and writes.
	#[inline]
	pub unsafe fn wrap(ptr: IterPtrMut<T>) -> Self {
		Self(ptr, PhantomData)
	}

	/// Creates a new [`Iter`] over the specified slice and stride.
	///
	/// # Panics
	///
	/// Panics if the given slice does not start and end at an element. That is,
	/// both the first and last elements of the slice must be elements that
	/// would be returned by this iterator. Do not include trailing stride.
	#[inline]
	pub fn new(slice: &mut [T], stride: usize) -> Self {
		unsafe { Self::new_ptr(slice as *mut [T], stride) }
	}

	/// Creates a new [`Iter`] over the specified slice and stride.
	///
	/// # Safety
	///
	/// UB if the given slice does not start and end at an element. That is,
	/// both the first and last elements of the slice must be elements that
	/// would be returned by this iterator. Do not include trailing stride.
	#[inline]
	pub unsafe fn new_unchecked(slice: &mut [T], stride: usize) -> Self {
		Self::new_ptr_unchecked(slice as *mut [T], stride)
	}

	/// Creates a new [`Iter`] over the specified slice and stride.
	///
	/// # Panics
	///
	/// Panics if the given slice does not start and end at an element. That is,
	/// both the first and last elements of the slice must be elements that
	/// would be returned by this iterator. Do not include trailing stride.
	#[inline]
	pub unsafe fn new_ptr(slice: *mut [T], stride: usize) -> Self {
		assert!(IterPtr::is_slice_perfect((*slice).len(), stride));
		Self::new_ptr_unchecked(slice, stride)
	}

	/// Creates a new [`Iter`] over the specified slice and stride.
	///
	/// # Safety
	///
	/// UB if the given slice does not start and end at an element. That is,
	/// both the first and last elements of the slice must be elements that
	/// would be returned by this iterator. Do not include trailing stride.
	#[inline]
	pub unsafe fn new_ptr_unchecked(slice: *mut [T], stride: usize) -> Self {
		Self::wrap(IterPtrMut::new(slice, stride))
	}

	/// Creates a new [`Iter`] over the specified buffer row.
	///
	/// # Panics
	///
	/// Panics if the given row is out of bounds.
	#[inline]
	pub fn row<S: AsMut<[T]>>(buf: &'a mut Img<S>, row: usize) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf_mut().as_mut() as *mut [T];
		unsafe { Self::row_ptr(Img::new_stride(buf, width, height, stride), row) }
	}

	/// Creates a new [`Iter`] over the specified buffer row.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for reads.
	///
	/// # Panics
	///
	/// Panics if the given row is out of bounds.
	#[inline]
	pub unsafe fn row_ptr(buf: Img<*mut [T]>, row: usize) -> Self {
		assert!(row < buf.height());
		Self::row_ptr_unchecked(buf, row)
	}

	/// Creates a new [`Iter`] over the specified buffer row.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for reads, and the given row must not
	/// be out of bounds.
	#[inline]
	pub unsafe fn row_ptr_unchecked(buf: Img<*mut [T]>, row: usize) -> Self {
		let slice = {
			let data = buf.buf().cast::<T>().add(row * buf.stride());
			let len = buf.width();
			slice_from_raw_parts_mut(data, len)
		};

		Self::new_ptr_unchecked(slice, 1)
	}

	/// Creates a new [`Iter`] over the specified buffer col.
	///
	/// # Panics
	///
	/// Panics if the given col is out of bounds.
	#[inline]
	pub fn col<S: AsMut<[T]>>(buf: &'a mut Img<S>, col: usize) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf_mut().as_mut() as *mut [T];
		unsafe { Self::col_ptr(Img::new_stride(buf, width, height, stride), col) }
	}

	/// Creates a new [`Iter`] over the specified buffer col.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for reads.
	///
	/// # Panics
	///
	/// Panics if the given col is out of bounds.
	#[inline]
	pub unsafe fn col_ptr(buf: Img<*mut [T]>, col: usize) -> Self {
		assert!(col < buf.width());
		Self::col_ptr_unchecked(buf, col)
	}

	/// Creates a new [`Iter`] over the specified buffer col.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for reads, and the given col must not
	/// be out of bounds.
	#[inline]
	pub unsafe fn col_ptr_unchecked(buf: Img<*mut [T]>, col: usize) -> Self {
		let slice = {
			let data = buf.buf().cast::<T>().add(col);
			let len = buf.stride() * (buf.height() - 1) + 1;
			slice_from_raw_parts_mut(data, len)
		};

		Self::new_ptr_unchecked(slice, buf.stride())
	}

	/// Converts this [`IterMut`] into its inner [`IterPtrMut`].
	pub fn into_inner(self) -> IterPtrMut<T> {
		self.0
	}
}

impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|ptr| unsafe { &mut *ptr })
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.0.next_back().map(|ptr| unsafe { &mut *ptr })
	}
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<'a, T> FusedIterator for IterMut<'a, T> {}
