use core::iter::FusedIterator;
use core::ops::Range;
use core::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
use imgref::Img;
use crate::iter::{IterPtr, IterPtrMut, SimdIterPtr, SimdIterPtrMut};
use crate::{slice_ptr_len, slice_ptr_len_mut};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SimdIterWindowsPtr<T, const LANES: usize>(*const [T], usize, usize, Range<usize>);

unsafe impl<T: Sync, const LANES: usize> Send for SimdIterWindowsPtr<T, LANES> {}

unsafe impl<T, const LANES: usize> Sync for SimdIterWindowsPtr<T, LANES> {}

impl<T, const LANES: usize> SimdIterWindowsPtr<T, LANES> {
	/// Creates a new [`SimdIterWindowsPtr`]:
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
	/// [`SimdIterWindowsPtr`]. There must be at least `iter_stride` elements
	/// available past the end of the provided slice.
	///
	/// # Panics
	///
	/// Panics if the slice does not start and end on an element.
	#[inline]
	pub unsafe fn new(slice: *const [T], slice_stride: usize, iter_stride: usize, len: usize) -> Self {
		assert!(IterPtr::is_slice_perfect(slice_ptr_len(slice), slice_stride));
		Self::new_unchecked(slice, slice_stride, iter_stride, len)
	}

	/// Same as `new`, but does not verify the slice length.
	///
	/// # Safety
	///
	/// All safety invariants of `new` must be upheld, and the slice must start
	/// and end on an element.
	#[inline]
	pub unsafe fn new_unchecked(slice: *const [T], slice_stride: usize, iter_stride: usize, len: usize) -> Self {
		Self(slice, slice_stride, iter_stride, 0..len)
	}

	/// Creates a new [`SimdIterWindowsPtr`] over the rows of an [`Img`].
	///
	/// # Safety
	///
	/// The buffer must be valid for the lifetime of the returned iterator.
	///
	/// # Panics
	///
	/// Panics if the provided buffer has a width and height too large to fit in
	/// its backing store.
	#[inline]
	pub unsafe fn rows<S: AsRef<[T]>>(buf: &Img<S>) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf().as_ref() as *const [T];
		Self::rows_ptr(Img::new_stride(buf, width, height, stride))
	}

	/// Creates a new [`SimdIterWindowsPtr`] over the rows of an [`Img`].
	///
	/// # Safety
	///
	/// The buffer must be valid for the lifetime of the returned iterator.
	///
	/// # Panics
	///
	/// Panics if the provided buffer has a width and height too large to fit in
	/// its backing store.
	#[inline]
	pub unsafe fn rows_ptr(buf: Img<*const [T]>) -> Self {
		IterPtr::assert_slice_enough(buf);
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let first_row = slice_from_raw_parts(buf.buf().cast::<T>(), width);
		Self::new_unchecked(first_row, 1, stride, height)
	}

	/// Creates a new [`SimdIterWindowsPtr`] over the cols of an [`Img`].
	///
	/// # Safety
	///
	/// The buffer must be valid for the lifetime of the returned iterator.
	///
	/// # Panics
	///
	/// Panics if the provided buffer has a width and height too large to fit in
	/// its backing store.
	#[inline]
	pub unsafe fn cols<S: AsRef<[T]>>(buf: &Img<S>) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf().as_ref() as *const [T];
		Self::cols_ptr(Img::new_stride(buf, width, height, stride))
	}

	/// Creates a new [`SimdIterWindowsPtr`] over the cols of an [`Img`].
	///
	/// # Safety
	///
	/// The buffer must be valid for the lifetime of the returned iterator.
	///
	/// # Panics
	///
	/// Panics if the provided buffer has a width and height too large to fit in
	/// its backing store.
	#[inline]
	pub unsafe fn cols_ptr(buf: Img<*const [T]>) -> Self {
		IterPtr::assert_slice_enough(buf);
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let first_col = slice_from_raw_parts(buf.buf().cast::<T>(), stride * (height - 1) + 1);
		Self::new_unchecked(first_col, buf.stride(), 1, width)
	}

	#[inline]
	unsafe fn window(&self, offset: usize) -> *const [T] {
		let data = self.0.cast::<T>().add(offset);
		let len = slice_ptr_len(self.0);
		slice_from_raw_parts(data, len)
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SimdIterWindowPtr<T, const LANES: usize> {
	Simd(SimdIterPtr<T, LANES>),
	Single(IterPtr<T>)
}

impl<T, const LANES: usize> Iterator for SimdIterWindowsPtr<T, LANES> {
	type Item = SimdIterWindowPtr<T, LANES>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		let (simd, index) = if self.3.len() >= LANES {
			let index = self.3.next().unwrap();
			for _ in 0..LANES - 1 { self.3.next().unwrap(); }
			(true, index)
		} else {
			(false, self.3.next()?)
		};

		let iter = unsafe { IterPtr::new(self.window(index * self.2), self.1) };

		Some(if simd {
			SimdIterWindowPtr::Simd(unsafe { SimdIterPtr::new(iter, self.2) })
		} else {
			SimdIterWindowPtr::Single(iter)
		})
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<T, const LANES: usize> DoubleEndedIterator for SimdIterWindowsPtr<T, LANES> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		let (simd, index) = if self.3.len() >= LANES {
			(true, self.3.nth_back(LANES - 1).unwrap())
		} else {
			(false, self.3.next_back()?)
		};

		let iter = unsafe { IterPtr::new(self.window(index * self.2), self.1) };

		Some(if simd {
			SimdIterWindowPtr::Simd(unsafe { SimdIterPtr::new(iter, self.2) })
		} else {
			SimdIterWindowPtr::Single(iter)
		})
	}
}

impl<T, const LANES: usize> ExactSizeIterator for SimdIterWindowsPtr<T, LANES> {
	#[inline]
	fn len(&self) -> usize {
		self.3.len() / LANES + self.3.len() % LANES
	}
}

impl<T, const LANES: usize> FusedIterator for SimdIterWindowsPtr<T, LANES> {}


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SimdIterWindowsPtrMut<T, const LANES: usize>(*mut [T], usize, usize, Range<usize>);

unsafe impl<T: Sync, const LANES: usize> Send for SimdIterWindowsPtrMut<T, LANES> {}

unsafe impl<T, const LANES: usize> Sync for SimdIterWindowsPtrMut<T, LANES> {}

impl<T, const LANES: usize> SimdIterWindowsPtrMut<T, LANES> {
	/// Creates a new [`SimdIterWindowsPtrMut`]:
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
	/// [`SimdIterWindowsPtrMut`]. There must be at least `iter_stride` elements
	/// available past the end of the provided slice.
	///
	/// # Panics
	///
	/// Panics if the slice does not start and end on an element.
	#[inline]
	pub unsafe fn new(slice: *mut [T], slice_stride: usize, iter_stride: usize, len: usize) -> Self {
		assert!(IterPtr::is_slice_perfect(slice_ptr_len_mut(slice), slice_stride));
		Self::new_unchecked(slice, slice_stride, iter_stride, len)
	}

	/// Same as `new`, but does not verify the slice length.
	///
	/// # Safety
	///
	/// All safety invariants of `new` must be upheld, and the slice must start
	/// and end on an element.
	#[inline]
	pub unsafe fn new_unchecked(slice: *mut [T], slice_stride: usize, iter_stride: usize, len: usize) -> Self {
		Self(slice, slice_stride, iter_stride, 0..len)
	}

	/// Creates a new [`SimdIterWindowsPtrMut`] over the rows of an [`Img`].
	///
	/// # Safety
	///
	/// The buffer must be valid for the lifetime of the returned iterator.
	///
	/// # Panics
	///
	/// Panics if the provided buffer has a width and height too large to fit in
	/// its backing store.
	#[inline]
	pub unsafe fn rows<S: AsMut<[T]>>(buf: &mut Img<S>) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf_mut().as_mut() as *mut [T];
		Self::rows_ptr(Img::new_stride(buf, width, height, stride))
	}

	/// Creates a new [`SimdIterWindowsPtrMut`] over the rows of an [`Img`].
	///
	/// # Safety
	///
	/// The buffer must be valid for the lifetime of the returned iterator.
	///
	/// # Panics
	///
	/// Panics if the provided buffer has a width and height too large to fit in
	/// its backing store.
	#[inline]
	pub unsafe fn rows_ptr(buf: Img<*mut [T]>) -> Self {
		IterPtrMut::assert_slice_enough(buf);
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let first_row = slice_from_raw_parts_mut(buf.buf().cast::<T>(), width);
		Self::new_unchecked(first_row, 1, stride, height)
	}

	/// Creates a new [`SimdIterWindowsPtrMut`] over the cols of an [`Img`].
	///
	/// # Safety
	///
	/// The buffer must be valid for the lifetime of the returned iterator.
	///
	/// # Panics
	///
	/// Panics if the provided buffer has a width and height too large to fit in
	/// its backing store.
	#[inline]
	pub unsafe fn cols<S: AsMut<[T]>>(buf: &mut Img<S>) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf_mut().as_mut() as *mut [T];
		Self::cols_ptr(Img::new_stride(buf, width, height, stride))
	}

	/// Creates a new [`SimdIterWindowsPtrMut`] over the cols of an [`Img`].
	///
	/// # Safety
	///
	/// The buffer must be valid for the lifetime of the returned iterator.
	///
	/// # Panics
	///
	/// Panics if the provided buffer has a width and height too large to fit in
	/// its backing store.
	#[inline]
	pub unsafe fn cols_ptr(buf: Img<*mut [T]>) -> Self {
		IterPtrMut::assert_slice_enough(buf);
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let first_col = slice_from_raw_parts_mut(buf.buf().cast::<T>(), stride * (height - 1) + 1);
		Self::new_unchecked(first_col, buf.stride(), 1, width)
	}

	#[inline]
	unsafe fn window(&self, offset: usize) -> *mut [T] {
		let data = self.0.cast::<T>().add(offset);
		let len = slice_ptr_len_mut(self.0);
		slice_from_raw_parts_mut(data, len)
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SimdIterWindowPtrMut<T, const LANES: usize> {
	Simd(SimdIterPtrMut<T, LANES>),
	Single(IterPtrMut<T>)
}

impl<T, const LANES: usize> Iterator for SimdIterWindowsPtrMut<T, LANES> {
	type Item = SimdIterWindowPtrMut<T, LANES>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		let (simd, index) = if self.3.len() >= LANES {
			let index = self.3.next().unwrap();
			for _ in 0..LANES - 1 { self.3.next().unwrap(); }
			(true, index)
		} else {
			(false, self.3.next()?)
		};

		let iter = unsafe { IterPtrMut::new(self.window(index * self.2), self.1) };

		Some(if simd {
			SimdIterWindowPtrMut::Simd(unsafe { SimdIterPtrMut::new(iter, self.2) })
		} else {
			SimdIterWindowPtrMut::Single(iter)
		})
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<T, const LANES: usize> DoubleEndedIterator for SimdIterWindowsPtrMut<T, LANES> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		let (simd, index) = if self.3.len() >= LANES {
			(true, self.3.nth_back(LANES - 1).unwrap())
		} else {
			(false, self.3.next_back()?)
		};

		let iter = unsafe { IterPtrMut::new(self.window(index * self.2), self.1) };

		Some(if simd {
			SimdIterWindowPtrMut::Simd(unsafe { SimdIterPtrMut::new(iter, self.2) })
		} else {
			SimdIterWindowPtrMut::Single(iter)
		})
	}
}

impl<T, const LANES: usize> ExactSizeIterator for SimdIterWindowsPtrMut<T, LANES> {
	#[inline]
	fn len(&self) -> usize {
		self.3.len() / LANES + self.3.len() % LANES
	}
}

impl<T, const LANES: usize> FusedIterator for SimdIterWindowsPtrMut<T, LANES> {}
