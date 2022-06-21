use std::iter::FusedIterator;
use std::ops::Range;
use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
use imgref::Img;
use crate::iter::{IterPtr, IterPtrMut};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IterWindowsPtr<T>(*const [T], usize, usize, Range<usize>);

unsafe impl<T: Sync> Send for IterWindowsPtr<T> {}

unsafe impl<T> Sync for IterWindowsPtr<T> {}

impl<T> IterWindowsPtr<T> {
	/// Creates a new [`IterWindowsPtr`]:
	///
	/// - `slice` is the slice that will be returned by the first iteration;
	/// - `slice_stride` is the stride of `slice`;
	/// - `iter_stride` is how far the slice will move each iteration;
	/// - `len` is how many iterations
	///
	/// For example, for an iterator over rows, this would be:
	///
	/// - the first row of the image;
	/// - `1`;
	/// - the stride of the image;
	/// - the height of the image
	///
	/// For an iterator over cols:
	///
	/// - the first column of the image;
	/// - the stride of the image;
	/// - `1`;
	/// - the width of the image
	///
	/// # Safety
	///
	/// The provided slice must be valid for the lifetime of the returned
	/// [`IterWindowsPtr`]. There must be at least `iter_stride` elements
	/// available past the end of the provided slice.
	///
	/// # Panics
	///
	/// Panics if the slice does not start and end on an element.
	#[inline]
	pub unsafe fn new(slice: *const [T], slice_stride: usize, iter_stride: usize, len: usize) -> Self {
		assert!(IterPtr::is_slice_perfect((*slice).len(), slice_stride));
		Self::new_unchecked(slice, slice_stride, iter_stride, len)
	}

	/// Same as [`new`], but does not verify the slice length.
	///
	/// # Safety
	///
	/// All safety invariants of [`new`] must be upheld, and the slice must
	/// start and end on an element.
	#[inline]
	pub unsafe fn new_unchecked(slice: *const [T], slice_stride: usize, iter_stride: usize, len: usize) -> Self {
		Self(slice, slice_stride, iter_stride, 0..len)
	}

	/// Creates a new [`IterWindowsPtr`] over the rows of an [`Img`].
	///
	/// # Safety
	///
	/// The buffer must be valid for the lifetime of the returned iterator.
	#[inline]
	pub unsafe fn rows<S: AsRef<[T]>>(buf: &Img<S>) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf().as_ref() as *const [T];
		Self::rows_ptr(Img::new_stride(buf, width, height, stride))
	}

	/// Creates a new [`IterWindowsPtr`] over the rows of an [`Img`].
	///
	/// # Safety
	///
	/// The buffer must be valid for the lifetime of the returned iterator.
	#[inline]
	pub unsafe fn rows_ptr(buf: Img<*const [T]>) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let first_row = slice_from_raw_parts(buf.buf().cast::<T>(), width);
		Self::new_unchecked(first_row, 1, stride, height)
	}

	/// Creates a new [`IterWindowsPtr`] over the cols of an [`Img`].
	///
	/// # Safety
	///
	/// The buffer must be valid for the lifetime of the returned iterator.
	#[inline]
	pub unsafe fn cols<S: AsRef<[T]>>(buf: &Img<S>) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf().as_ref() as *const [T];
		Self::cols_ptr(Img::new_stride(buf, width, height, stride))
	}

	/// Creates a new [`IterWindowsPtr`] over the cols of an [`Img`].
	///
	/// # Safety
	///
	/// The buffer must be valid for the lifetime of the returned iterator.
	#[inline]
	pub unsafe fn cols_ptr(buf: Img<*const [T]>) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let first_col = slice_from_raw_parts(buf.buf().cast::<T>(), stride * (height - 1) + 1);
		Self::new_unchecked(first_col, buf.stride(), 1, width)
	}

	#[inline]
	unsafe fn window(&self, offset: usize) -> *const [T] {
		let data = self.0.cast::<T>().add(offset);
		let len = (*self.0).len();
		slice_from_raw_parts(data, len)
	}
}

impl<T> Iterator for IterWindowsPtr<T> {
	type Item = IterPtr<T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.3.next().map(|index| unsafe { IterPtr::new(self.window(index * self.2), self.1) })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<T> DoubleEndedIterator for IterWindowsPtr<T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.3.next_back().map(|index| unsafe { IterPtr::new(self.window(index * self.2), self.1) })
	}
}

impl<T> ExactSizeIterator for IterWindowsPtr<T> {
	#[inline]
	fn len(&self) -> usize {
		self.3.len()
	}
}

impl<T> FusedIterator for IterWindowsPtr<T> {}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IterWindowsPtrMut<T>(*mut [T], usize, usize, Range<usize>);

unsafe impl<T: Send> Send for IterWindowsPtrMut<T> {}

unsafe impl<T> Sync for IterWindowsPtrMut<T> {}

impl<T> IterWindowsPtrMut<T> {
	/// Creates a new [`IterWindowsPtrMut`]:
	///
	/// - `slice` is the slice that will be returned by the first iteration;
	/// - `slice_stride` is the stride of `slice`;
	/// - `iter_stride` is how far the slice will move each iteration;
	/// - `len` is how many iterations
	///
	/// For example, for an iterator over rows, this would be:
	///
	/// - the first row of the image;
	/// - `1`;
	/// - the stride of the image;
	/// - the height of the image
	///
	/// For an iterator over cols:
	///
	/// - the first column of the image;
	/// - the stride of the image;
	/// - `1`;
	/// - the width of the image
	///
	/// # Safety
	///
	/// The provided slice must be valid for the lifetime of the returned
	/// [`IterWindowsPtrMut`]. There must be at least `iter_stride` elements
	/// available past the end of the provided slice.
	///
	/// # Panics
	///
	/// Panics if the slice does not start and end on an element.
	#[inline]
	pub unsafe fn new(slice: *mut [T], slice_stride: usize, iter_stride: usize, len: usize) -> Self {
		assert!(IterPtr::is_slice_perfect((*slice).len(), slice_stride));
		Self::new_unchecked(slice, slice_stride, iter_stride, len)
	}

	/// Same as [`new`], but does not verify the slice length.
	///
	/// # Safety
	///
	/// All safety invariants of [`new`] must be upheld, and the slice must
	/// start and end on an element.
	#[inline]
	pub unsafe fn new_unchecked(slice: *mut [T], slice_stride: usize, iter_stride: usize, len: usize) -> Self {
		Self(slice, slice_stride, iter_stride, 0..len)
	}

	/// Creates a new [`IterWindowsPtrMut`] over the rows of an [`Img`].
	///
	/// # Safety
	///
	/// The buffer must be valid for the lifetime of the returned iterator.
	#[inline]
	pub unsafe fn rows<S: AsMut<[T]>>(buf: &mut Img<S>) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf_mut().as_mut() as *mut [T];
		Self::rows_ptr(Img::new_stride(buf, width, height, stride))
	}

	/// Creates a new [`IterWindowsPtrMut`] over the rows of an [`Img`].
	///
	/// # Safety
	///
	/// The buffer must be valid for the lifetime of the returned iterator.
	#[inline]
	pub unsafe fn rows_ptr(buf: Img<*mut [T]>) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let first_row = slice_from_raw_parts_mut(buf.buf().cast::<T>(), width);
		Self::new_unchecked(first_row, 1, stride, height)
	}

	/// Creates a new [`IterWindowsPtrMut`] over the cols of an [`Img`].
	///
	/// # Safety
	///
	/// The buffer must be valid for the lifetime of the returned iterator.
	#[inline]
	pub unsafe fn cols<S: AsMut<[T]>>(buf: &mut Img<S>) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf_mut().as_mut() as *mut [T];
		Self::cols_ptr(Img::new_stride(buf, width, height, stride))
	}

	/// Creates a new [`IterWindowsPtrMut`] over the cols of an [`Img`].
	///
	/// # Safety
	///
	/// The buffer must be valid for the lifetime of the returned iterator.
	#[inline]
	pub unsafe fn cols_ptr(buf: Img<*mut [T]>) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let first_col = slice_from_raw_parts_mut(buf.buf().cast::<T>(), stride * (height - 1) + 1);
		Self::new_unchecked(first_col, buf.stride(), 1, width)
	}

	#[inline]
	unsafe fn window(&self, offset: usize) -> *mut [T] {
		let data = self.0.cast::<T>().add(offset);
		let len = (*self.0).len();
		slice_from_raw_parts_mut(data, len)
	}
}

impl<T> Iterator for IterWindowsPtrMut<T> {
	type Item = IterPtrMut<T>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.3.next().map(|index| unsafe { IterPtrMut::new(self.window(index * self.2), self.1) })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<T> DoubleEndedIterator for IterWindowsPtrMut<T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.3.next_back().map(|index| unsafe { IterPtrMut::new(self.window(index * self.2), self.1) })
	}
}

impl<T> ExactSizeIterator for IterWindowsPtrMut<T> {
	#[inline]
	fn len(&self) -> usize {
		self.3.len()
	}
}

impl<T> FusedIterator for IterWindowsPtrMut<T> {}
