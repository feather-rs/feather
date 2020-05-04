use bstr::BStr;
use std::alloc::Layout;
use std::marker::PhantomData;
use std::slice;

/// A string reference which may be passed over FFI boundaries.
///
/// Does not necessarily point to valid UTF8.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FStr<'a> {
    ptr: *const u8,
    len: usize,
    _lifetime: PhantomData<&'a [u8]>,
}

impl<'a> From<&'a BStr> for FStr<'a> {
    fn from(s: &'a BStr) -> Self {
        let slice: &[u8] = s.as_ref();
        Self::from(slice)
    }
}

impl<'a> From<&'a [u8]> for FStr<'a> {
    fn from(slice: &'a [u8]) -> Self {
        Self {
            ptr: slice.as_ptr(),
            len: slice.len(),
            _lifetime: PhantomData,
        }
    }
}

impl<'a> From<&'a str> for FStr<'a> {
    fn from(string: &'a str) -> Self {
        Self {
            ptr: string.as_ptr(),
            len: string.len(),
            _lifetime: PhantomData,
        }
    }
}

impl<'a> From<FStr<'a>> for &'a BStr {
    fn from(string: FStr<'a>) -> Self {
        let slice = <&[u8]>::from(string);
        slice.into()
    }
}

impl<'a> From<FStr<'a>> for &'a [u8] {
    fn from(string: FStr<'a>) -> Self {
        unsafe { slice::from_raw_parts(string.ptr, string.len) }
    }
}

/// An FFI-safe version of `std::allloc::Layout`.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FLayout {
    size: usize,
    align: usize,
}

impl From<Layout> for FLayout {
    fn from(layout: Layout) -> Self {
        Self {
            size: layout.size(),
            align: layout.align(),
        }
    }
}

impl From<FLayout> for Layout {
    fn from(layout: FLayout) -> Self {
        Layout::from_size_align(layout.size, layout.align).unwrap()
    }
}
