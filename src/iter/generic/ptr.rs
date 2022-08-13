use std::cmp::min;
use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
use imgref::Img;
use crate::{slice_ptr_len, slice_ptr_len_mut};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct IterPtr<T>(*const [T], usize);

unsafe impl<T: Sync> Send for IterPtr<T> {}

unsafe impl<T> Sync for IterPtr<T> {}

impl IterPtr<()> {
	/// This crate's iterators are double-ended, so there must be an element on
	/// both sides of the slice.
	///
	/// Even if they weren't, trailing stride is not even guaranteed, meaning
	/// that relying on it would be a mistake. Offsetting into it would be UB.
	/// Instead we just rely on elements from first to last existing, and
	/// everything around them is forbidden territory.
	#[doc(hidden)]
	#[inline(always)]
	pub(crate) fn is_slice_perfect(len: usize, stride: usize) -> bool {
		len == 0 || stride == 1 || len % stride == 1
	}

	/// It's possible to construct `Img`s with any dimensions over any slice,
	/// which means the slice might not have enough elements, even if the `Img`
	/// says it does.
	///
	/// If the slice has enough elements, does nothing. Else, panics with a
	/// descriptive message.
	#[doc(hidden)]
	#[inline(always)]
	pub(crate) fn assert_slice_enough<T>(img: Img<*const [T]>) {
		let (width, height, stride) = (img.width(), img.height(), img.stride());
		let needed = stride * (height - 1) + width;
		let got = unsafe { slice_ptr_len(*img.buf()) };

		if got < needed {
			panic!("image (with width {width}, height {height} and stride {stride}) backing buffer too short; needed {needed} elements, but only got {got}");
		} else if width > stride {
			panic!("image (with width {width}, height {height} and stride {stride}) width is greater than stride")
		}
	}
}

impl<T> IterPtr<T> {
	/// Creates a new [`IterPtr`] over the specified slice and stride.
	///
	/// # Safety
	///
	/// The given slice must outlive this [`IterPtr`].
	///
	/// # Panics
	///
	/// Panics if the given slice does not start and end at an element. That is,
	/// both the first and last elements of the slice must be elements that
	/// would be returned by this iterator. Do not include trailing stride.
	#[inline]
	pub unsafe fn new(slice: *const [T], stride: usize) -> Self {
		assert!(IterPtr::is_slice_perfect(slice_ptr_len(slice), stride));
		Self::new_unchecked(slice, stride)
	}

	/// Creates a new [`IterPtr`] over the specified slice and stride.
	///
	/// # Safety
	///
	/// The given slice must outlive this [`IterPtr`].
	///
	/// UB if the given slice does not start and end at an element. That is,
	/// both the first and last elements of the slice must be elements that
	/// would be returned by this iterator. Do not include trailing stride.
	#[inline]
	pub unsafe fn new_unchecked(slice: *const [T], stride: usize) -> Self {
		Self(slice, stride)
	}

	/// Creates a new [`IterPtr`] over the specified slice and stride.
	///
	/// # Safety
	///
	/// The given slice must outlive this [`IterPtr`].
	///
	/// # Panics
	///
	/// Panics if the given slice does not start and end at an element. That is,
	/// both the first and last elements of the slice must be elements that
	/// would be returned by this iterator. Do not include trailing stride.
	#[inline]
	pub unsafe fn new_slice(slice: &[T], stride: usize) -> Self {
		Self::new(slice as *const [T], stride)
	}

	/// Creates a new [`IterPtr`] over the specified slice and stride.
	///
	/// # Safety
	///
	/// The given slice must outlive this [`IterPtr`].
	///
	/// UB if the given slice does not start and end at an element. That is,
	/// both the first and last elements of the slice must be elements that
	/// would be returned by this iterator. Do not include trailing stride.
	#[inline]
	pub unsafe fn new_slice_unchecked(slice: &[T], stride: usize) -> Self {
		Self::new_unchecked(slice as *const [T], stride)
	}

	/// Creates a new [`IterPtr`] over the specified buffer row.
	///
	/// # Safety
	///
	/// The given buffer must outlive this [`IterPtr`].
	///
	/// # Panics
	///
	/// Panics if the given row is out of bounds.
	#[inline]
	pub unsafe fn row<S: AsRef<[T]>>(buf: &Img<S>, row: usize) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf().as_ref() as *const [T];
		Self::row_ptr(Img::new_stride(buf, width, height, stride), row)
	}

	/// Creates a new [`IterPtr`] over the specified buffer row.
	///
	/// # Safety
	///
	/// The given buffer must outlive this [`IterPtr`].
	///
	/// # Panics
	///
	/// Panics if the provided buffer has a width and height too large to fit in
	/// its backing store.
	///
	/// Panics if the given row is out of bounds.
	#[inline]
	pub unsafe fn row_ptr(buf: Img<*const [T]>, row: usize) -> Self {
		IterPtr::assert_slice_enough(buf);
		assert!(row < buf.height());
		Self::row_ptr_unchecked(buf, row)
	}

	/// Creates a new [`IterPtr`] over the specified buffer row.
	///
	/// # Safety
	///
	/// The given slice must outlive this [`IterPtr`].
	///
	/// The given row must not be out of bounds.
	#[inline]
	pub unsafe fn row_ptr_unchecked(buf: Img<*const [T]>, row: usize) -> Self {
		let slice = {
			let data = buf.buf().cast::<T>().add(row * buf.stride());
			let len = buf.width();
			slice_from_raw_parts(data, len)
		};

		Self::new_unchecked(slice, 1)
	}

	/// Creates a new [`IterPtr`] over the specified buffer col.
	///
	/// # Safety
	///
	/// The given buffer must outlive this [`IterPtr`].
	///
	/// # Panics
	///
	/// Panics if the given col is out of bounds.
	#[inline]
	pub unsafe fn col<S: AsRef<[T]>>(buf: &Img<S>, col: usize) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf().as_ref() as *const [T];
		Self::col_ptr(Img::new_stride(buf, width, height, stride), col)
	}

	/// Creates a new [`IterPtr`] over the specified buffer col.
	///
	/// # Safety
	///
	/// The given slice must outlive this [`IterPtr`].
	///
	/// # Panics
	///
	/// Panics if the provided buffer has a width and height too large to fit in
	/// its backing store.
	///
	/// Panics if the given col is out of bounds.
	#[inline]
	pub unsafe fn col_ptr(buf: Img<*const [T]>, col: usize) -> Self {
		IterPtr::assert_slice_enough(buf);
		assert!(col < buf.width());
		Self::col_ptr_unchecked(buf, col)
	}

	/// Creates a new [`IterPtr`] over the specified buffer col.
	///
	/// # Safety
	///
	/// The given slice must outlive this [`IterPtr`].
	///
	/// The given col must not be out of bounds.
	#[inline]
	pub unsafe fn col_ptr_unchecked(buf: Img<*const [T]>, col: usize) -> Self {
		let slice = {
			let data = buf.buf().cast::<T>().add(col);
			let len = buf.stride() * (buf.height() - 1) + 1;
			slice_from_raw_parts(data, len)
		};

		Self::new_unchecked(slice, buf.stride())
	}
}

impl<T> Iterator for IterPtr<T> {
	type Item = *const T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		let len = unsafe { slice_ptr_len(self.0) };

		if len > 0 {
			let first = self.0.cast::<T>();

			self.0 = unsafe {
				let data = first.add(min(self.1, len));
				let len = len.saturating_sub(self.1);
				slice_from_raw_parts(data, len)
			};

			Some(first)
		} else {
			None
		}
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<T> DoubleEndedIterator for IterPtr<T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		let len = unsafe { slice_ptr_len(self.0) };

		if len > 0 {
			let first = self.0.cast::<T>();

			self.0 = {
				let data = first;
				let len = len.saturating_sub(self.1);
				slice_from_raw_parts(data, len)
			};

			Some(unsafe { first.add(len - 1) })
		} else {
			None
		}
	}
}

impl<T> ExactSizeIterator for IterPtr<T> {
	#[inline]
	fn len(&self) -> usize {
		let len = unsafe { slice_ptr_len(self.0) };
		(len + (self.1 - 1)) / self.1
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct IterPtrMut<T>(*mut [T], usize);

unsafe impl<T: Send> Send for IterPtrMut<T> {}

unsafe impl<T> Sync for IterPtrMut<T> {}

impl IterPtrMut<()> {
	#[doc(hidden)]
	#[inline(always)]
	pub(crate) fn assert_slice_enough<T>(img: Img<*mut [T]>) {
		use crate::traits::ImgAsPtr;
		IterPtr::assert_slice_enough(img.as_ptr())
	}
}

impl<T> IterPtrMut<T> {
	/// Creates a new [`IterPtrMut`] over the specified slice and stride.
	///
	/// # Safety
	///
	/// The given slice must outlive this [`IterPtrMut`].
	///
	/// # Panics
	///
	/// Panics if the given slice does not start and end at an element. That is,
	/// both the first and last elements of the slice must be elements that
	/// would be returned by this iterator. Do not include trailing stride.
	#[inline]
	pub unsafe fn new(slice: *mut [T], stride: usize) -> Self {
		assert!(IterPtr::is_slice_perfect(slice_ptr_len_mut(slice), stride));
		Self::new_unchecked(slice, stride)
	}

	/// Creates a new [`IterPtrMut`] over the specified slice and stride.
	///
	/// # Safety
	///
	/// The given slice must outlive this [`IterPtrMut`].
	///
	/// UB if the given slice does not start and end at an element. That is,
	/// both the first and last elements of the slice must be elements that
	/// would be returned by this iterator. Do not include trailing stride.
	#[inline]
	pub unsafe fn new_unchecked(slice: *mut [T], stride: usize) -> Self {
		Self(slice, stride)
	}

	/// Creates a new [`IterPtrMut`] over the specified slice and stride.
	///
	/// # Safety
	///
	/// The given slice must outlive this [`IterPtrMut`].
	///
	/// # Panics
	///
	/// Panics if the given slice does not start and end at an element. That is,
	/// both the first and last elements of the slice must be elements that
	/// would be returned by this iterator. Do not include trailing stride.
	#[inline]
	pub unsafe fn new_slice(slice: &mut [T], stride: usize) -> Self {
		Self::new(slice as *mut [T], stride)
	}

	/// Creates a new [`IterPtrMut`] over the specified slice and stride.
	///
	/// # Safety
	///
	/// The given slice must outlive this [`IterPtrMut`].
	///
	/// UB if the given slice does not start and end at an element. That is,
	/// both the first and last elements of the slice must be elements that
	/// would be returned by this iterator. Do not include trailing stride.
	#[inline]
	pub unsafe fn new_slice_unchecked(slice: &mut [T], stride: usize) -> Self {
		Self::new_unchecked(slice as *mut [T], stride)
	}

	/// Creates a new [`IterPtrMut`] over the specified buffer row.
	///
	/// # Safety
	///
	/// The given buffer must outlive this [`IterPtrMut`].
	///
	/// # Panics
	///
	/// Panics if the given row is out of bounds.
	#[inline]
	pub unsafe fn row<S: AsMut<[T]>>(buf: &mut Img<S>, row: usize) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf_mut().as_mut() as *mut [T];
		Self::row_ptr(Img::new_stride(buf, width, height, stride), row)
	}

	/// Creates a new [`IterPtrMut`] over the specified buffer row.
	///
	/// # Safety
	///
	/// The given buffer must outlive this [`IterPtrMut`].
	///
	/// # Panics
	///
	/// Panics if the provided buffer has a width and height too large to fit in
	/// its backing store.
	///
	/// Panics if the given row is out of bounds.
	#[inline]
	pub unsafe fn row_ptr(buf: Img<*mut [T]>, row: usize) -> Self {
		IterPtrMut::assert_slice_enough(buf);
		assert!(row < buf.height());
		Self::row_ptr_unchecked(buf, row)
	}

	/// Creates a new [`IterPtrMut`] over the specified buffer row.
	///
	/// # Safety
	///
	/// The given slice must outlive this [`IterPtrMut`].
	///
	/// The given row must not be out of bounds.
	#[inline]
	pub unsafe fn row_ptr_unchecked(buf: Img<*mut [T]>, row: usize) -> Self {
		let slice = {
			let data = buf.buf().cast::<T>().add(row * buf.stride());
			let len = buf.width();
			slice_from_raw_parts_mut(data, len)
		};

		Self::new_unchecked(slice, 1)
	}

	/// Creates a new [`IterPtrMut`] over the specified buffer col.
	///
	/// # Safety
	///
	/// The given buffer must outlive this [`IterPtrMut`].
	///
	/// # Panics
	///
	/// Panics if the given col is out of bounds.
	#[inline]
	pub unsafe fn col<S: AsMut<[T]>>(buf: &mut Img<S>, col: usize) -> Self {
		let (width, height, stride) = (buf.width(), buf.height(), buf.stride());
		let buf = buf.buf_mut().as_mut() as *mut [T];
		Self::col_ptr(Img::new_stride(buf, width, height, stride), col)
	}

	/// Creates a new [`IterPtrMut`] over the specified buffer col.
	///
	/// # Safety
	///
	/// The given slice must outlive this [`IterPtrMut`].
	///
	/// # Panics
	///
	/// Panics if the provided buffer has a width and height too large to fit in
	/// its backing store.
	///
	/// Panics if the given col is out of bounds.
	#[inline]
	pub unsafe fn col_ptr(buf: Img<*mut [T]>, col: usize) -> Self {
		IterPtrMut::assert_slice_enough(buf);
		assert!(col < buf.width());
		Self::col_ptr_unchecked(buf, col)
	}

	/// Creates a new [`IterPtrMut`] over the specified buffer col.
	///
	/// # Safety
	///
	/// The given slice must outlive this [`IterPtrMut`].
	///
	/// The given col must not be out of bounds.
	#[inline]
	pub unsafe fn col_ptr_unchecked(buf: Img<*mut [T]>, col: usize) -> Self {
		let slice = {
			let data = buf.buf().cast::<T>().add(col);
			let len = buf.stride() * (buf.height() - 1) + 1;
			slice_from_raw_parts_mut(data, len)
		};

		Self::new_unchecked(slice, buf.stride())
	}
}

impl<T> Iterator for IterPtrMut<T> {
	type Item = *mut T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		let len = unsafe { slice_ptr_len_mut(self.0) };

		if len > 0 {
			let first = self.0.cast::<T>();

			self.0 = unsafe {
				let data = first.add(min(self.1, len));
				let len = len.saturating_sub(self.1);
				slice_from_raw_parts_mut(data, len)
			};

			Some(first)
		} else {
			None
		}
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<T> DoubleEndedIterator for IterPtrMut<T> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		let len = unsafe { slice_ptr_len_mut(self.0) };

		if len > 0 {
			let first = self.0.cast::<T>();

			self.0 = {
				let data = first;
				let len = len.saturating_sub(self.1);
				slice_from_raw_parts_mut(data, len)
			};

			Some(unsafe { first.add(len - 1) })
		} else {
			None
		}
	}
}

impl<T> ExactSizeIterator for IterPtrMut<T> {
	#[inline]
	fn len(&self) -> usize {
		let len = unsafe { slice_ptr_len_mut(self.0) };
		(len + (self.1 - 1)) / self.1
	}
}
