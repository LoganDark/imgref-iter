use core::iter::FusedIterator;
use imgref::Img;
use crate::iter::{IterPtr, IterPtrMut};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SimdIterPtr<T, const LANES: usize>(IterPtr<T>, usize);

impl<T, const LANES: usize> SimdIterPtr<T, LANES> {
	/// Creates a new [`SimdIterPtr`] from the given [`IterPtr`] and gap.
	///
	/// The gap is the distance between successive items in the returned arrays.
	/// For example if the iterator is over a row and the gap is the stride,
	/// then this iterator will yield items from multiple rows at a time.
	///
	/// # Safety
	///
	/// The given iterator must be valid and the gap must be valid.
	#[inline]
	pub unsafe fn new(iter: IterPtr<T>, gap: usize) -> Self {
		Self(iter, gap)
	}

	/// Creates a new [`SimdIterPtr`] across `LANES` rows.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIterPtr`].
	///
	/// # Panics
	///
	/// Panics if the given `row + LANES > buf.height()`.
	#[inline]
	pub unsafe fn rows<S: AsRef<[T]>>(buf: &Img<S>, row: usize) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf().as_ref() as *const [T];
		let buf = Img::new_stride(buf, width, height, stride);
		Self::rows_ptr(buf, row)
	}

	/// Creates a new [`SimdIterPtr`] across `LANES` rows.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIterPtr`].
	///
	/// # Panics
	///
	/// Panics if the provided buffer has a width and height too large to fit in
	/// its backing store.
	///
	/// Panics if the given `row + LANES > buf.height()`.
	#[inline]
	pub unsafe fn rows_ptr(buf: Img<*const [T]>, row: usize) -> Self {
		IterPtr::assert_slice_enough(buf);
		assert!(row + LANES <= buf.height());
		Self::rows_ptr_unchecked(buf, row)
	}

	/// Creates a new [`SimdIterPtr`] across `LANES` rows.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIterPtr`].
	///
	/// The caller must ensure that `row + LANES > buf.height()`.
	#[inline]
	pub unsafe fn rows_ptr_unchecked(buf: Img<*const [T]>, row: usize) -> Self {
		let gap = buf.stride();
		Self::new(IterPtr::row_ptr(buf, row), gap)
	}

	/// Creates a new [`SimdIterPtr`] across `LANES` cols.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIterPtr`].
	///
	/// # Panics
	///
	/// Panics if the given `col + LANES > buf.width()`.
	#[inline]
	pub unsafe fn cols<S: AsRef<[T]>>(buf: &Img<S>, col: usize) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf().as_ref() as *const [T];
		let buf = Img::new_stride(buf, width, height, stride);
		Self::cols_ptr(buf, col)
	}

	/// Creates a new [`SimdIterPtr`] across `LANES` cols.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIterPtr`].
	///
	/// # Panics
	///
	/// Panics if the provided buffer has a width and height too large to fit in
	/// its backing store.
	///
	/// Panics if the given `col + LANES > buf.width()`.
	#[inline]
	pub unsafe fn cols_ptr(buf: Img<*const [T]>, col: usize) -> Self {
		IterPtr::assert_slice_enough(buf);
		assert!(col + LANES <= buf.width());
		Self::cols_ptr_unchecked(buf, col)
	}

	/// Creates a new [`SimdIterPtr`] across `LANES` cols.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIterPtr`].
	///
	/// The caller must ensure that `col + LANES > buf.width()`.
	#[inline]
	pub unsafe fn cols_ptr_unchecked(buf: Img<*const [T]>, col: usize) -> Self {
		Self::new(IterPtr::col_ptr(buf, col), 1)
	}

	/// Converts this [`SimdIterPtr`] into its inner [`IterPtr`].
	pub fn into_inner(self) -> IterPtr<T> {
		self.0
	}

	#[inline]
	fn expand(&self, one: *const T) -> [*const T; LANES] {
		let mut countup = 0usize..;
		[(); LANES].map(move |_| unsafe { one.add(self.1 * countup.next().unwrap()) })
	}
}

impl<T, const LANES: usize> Iterator for SimdIterPtr<T, LANES> {
	type Item = [*const T; LANES];

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|ptr| self.expand(ptr))
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.0.size_hint()
	}
}

impl<T, const LANES: usize> DoubleEndedIterator for SimdIterPtr<T, LANES> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.0.next_back().map(|ptr| self.expand(ptr))
	}
}

impl<T, const LANES: usize> ExactSizeIterator for SimdIterPtr<T, LANES> {
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<T, const LANES: usize> FusedIterator for SimdIterPtr<T, LANES> {}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SimdIterPtrMut<T, const LANES: usize>(IterPtrMut<T>, usize);

impl<T, const LANES: usize> SimdIterPtrMut<T, LANES> {
	/// Creates a new [`SimdIterPtrMut`] from the given [`IterPtrMut`] and gap.
	///
	/// The gap is the distance between successive items in the returned arrays.
	/// For example if the iterator is over a row and the gap is the stride,
	/// then this iterator will yield items from multiple rows at a time.
	///
	/// # Safety
	///
	/// The given iterator must be valid and the gap must be valid.
	#[inline]
	pub unsafe fn new(iter: IterPtrMut<T>, gap: usize) -> Self {
		Self(iter, gap)
	}

	/// Creates a new [`SimdIterPtrMut`] across `LANES` rows.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIterPtrMut`].
	///
	/// # Panics
	///
	/// Panics if the given `row + LANES > buf.height()`.
	#[inline]
	pub unsafe fn rows<S: AsMut<[T]>>(buf: &mut Img<S>, row: usize) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf_mut().as_mut() as *mut [T];
		let buf = Img::new_stride(buf, width, height, stride);
		Self::rows_ptr(buf, row)
	}

	/// Creates a new [`SimdIterPtrMut`] across `LANES` rows.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIterPtrMut`].
	///
	/// # Panics
	///
	/// Panics if the provided buffer has a width and height too large to fit in
	/// its backing store.
	///
	/// Panics if the given `row + LANES > buf.height()`.
	#[inline]
	pub unsafe fn rows_ptr(buf: Img<*mut [T]>, row: usize) -> Self {
		IterPtrMut::assert_slice_enough(buf);
		assert!(row + LANES <= buf.height());
		Self::rows_ptr_unchecked(buf, row)
	}

	/// Creates a new [`SimdIterPtrMut`] across `LANES` rows.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIterPtrMut`].
	///
	/// The caller must ensure that `row + LANES > buf.height()`.
	#[inline]
	pub unsafe fn rows_ptr_unchecked(buf: Img<*mut [T]>, row: usize) -> Self {
		let gap = buf.stride();
		Self::new(IterPtrMut::row_ptr(buf, row), gap)
	}

	/// Creates a new [`SimdIterPtrMut`] across `LANES` cols.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIterPtrMut`].
	///
	/// # Panics
	///
	/// Panics if the given `col + LANES > buf.width()`.
	#[inline]
	pub unsafe fn cols<S: AsMut<[T]>>(buf: &mut Img<S>, col: usize) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf_mut().as_mut() as *mut [T];
		let buf = Img::new_stride(buf, width, height, stride);
		Self::cols_ptr(buf, col)
	}

	/// Creates a new [`SimdIterPtrMut`] across `LANES` cols.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIterPtrMut`].
	///
	/// # Panics
	///
	/// Panics if the provided buffer has a width and height too large to fit in
	/// its backing store.
	///
	/// Panics if the given `col + LANES > buf.width()`.
	#[inline]
	pub unsafe fn cols_ptr(buf: Img<*mut [T]>, col: usize) -> Self {
		IterPtrMut::assert_slice_enough(buf);
		assert!(col + LANES <= buf.width());
		Self::cols_ptr_unchecked(buf, col)
	}

	/// Creates a new [`SimdIterPtrMut`] across `LANES` cols.
	///
	/// # Safety
	///
	/// The provided buffer must be valid for the lifetime of the returned
	/// [`SimdIterPtrMut`].
	///
	/// The caller must ensure that `col + LANES > buf.width()`.
	#[inline]
	pub unsafe fn cols_ptr_unchecked(buf: Img<*mut [T]>, col: usize) -> Self {
		Self::new(IterPtrMut::col_ptr(buf, col), 1)
	}

	/// Converts this [`SimdIterPtrMut`] into its inner [`IterPtrMut`].
	pub fn into_inner(self) -> IterPtrMut<T> {
		self.0
	}

	#[inline]
	fn expand(&self, one: *mut T) -> [*mut T; LANES] {
		let mut countup = 0usize..;
		[(); LANES].map(move |_| unsafe { one.add(self.1 * countup.next().unwrap()) })
	}
}

impl<T, const LANES: usize> Iterator for SimdIterPtrMut<T, LANES> {
	type Item = [*mut T; LANES];

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|ptr| self.expand(ptr))
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.0.size_hint()
	}
}

impl<T, const LANES: usize> DoubleEndedIterator for SimdIterPtrMut<T, LANES> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.0.next_back().map(|ptr| self.expand(ptr))
	}
}

impl<T, const LANES: usize> ExactSizeIterator for SimdIterPtrMut<T, LANES> {
	#[inline]
	fn len(&self) -> usize {
		self.0.len()
	}
}

impl<T, const LANES: usize> FusedIterator for SimdIterPtrMut<T, LANES> {}
