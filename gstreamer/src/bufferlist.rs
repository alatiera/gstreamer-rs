// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::{from_glib, from_glib_full, from_glib_none, IntoGlib, IntoGlibPtr};
use std::fmt;
use std::ops::ControlFlow;
use std::ptr;

use crate::Buffer;
use crate::BufferRef;

mini_object_wrapper!(BufferList, BufferListRef, ffi::GstBufferList, || {
    ffi::gst_buffer_list_get_type()
});

impl BufferList {
    #[doc(alias = "gst_buffer_list_new")]
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_buffer_list_new()) }
    }

    #[doc(alias = "gst_buffer_list_new_sized")]
    pub fn new_sized(size: usize) -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_buffer_list_new_sized(size as u32)) }
    }
}

impl BufferListRef {
    #[doc(alias = "gst_buffer_list_insert")]
    pub fn insert(&mut self, idx: i32, buffer: Buffer) {
        unsafe {
            ffi::gst_buffer_list_insert(self.as_mut_ptr(), idx, buffer.into_glib_ptr());
        }
    }

    #[doc(alias = "gst_buffer_list_add")]
    pub fn add(&mut self, buffer: Buffer) {
        self.insert(-1, buffer);
    }

    #[doc(alias = "gst_buffer_list_copy_deep")]
    pub fn copy_deep(&self) -> BufferList {
        unsafe { from_glib_full(ffi::gst_buffer_list_copy_deep(self.as_ptr())) }
    }

    #[doc(alias = "gst_buffer_list_remove")]
    pub fn remove(&mut self, idx: u32, len: u32) {
        unsafe { ffi::gst_buffer_list_remove(self.as_mut_ptr(), idx, len) }
    }

    #[doc(alias = "gst_buffer_list_get")]
    pub fn get(&self, idx: u32) -> Option<&BufferRef> {
        unsafe {
            let ptr = ffi::gst_buffer_list_get(self.as_mut_ptr(), idx);
            if ptr.is_null() {
                None
            } else {
                Some(BufferRef::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "gst_buffer_list_get")]
    pub fn get_owned(&self, idx: u32) -> Option<Buffer> {
        unsafe {
            let ptr = ffi::gst_buffer_list_get(self.as_mut_ptr(), idx);
            from_glib_none(ptr)
        }
    }

    #[doc(alias = "gst_buffer_list_get_writable")]
    #[doc(alias = "get_writable")]
    pub fn get_mut(&mut self, idx: u32) -> Option<&mut BufferRef> {
        unsafe {
            let ptr = ffi::gst_buffer_list_get_writable(self.as_mut_ptr(), idx);
            if ptr.is_null() {
                None
            } else {
                Some(BufferRef::from_mut_ptr(ptr))
            }
        }
    }

    #[doc(alias = "gst_buffer_list_length")]
    pub fn len(&self) -> usize {
        unsafe { ffi::gst_buffer_list_length(self.as_mut_ptr()) as usize }
    }

    #[doc(alias = "gst_buffer_list_calculate_size")]
    pub fn calculate_size(&self) -> usize {
        unsafe { ffi::gst_buffer_list_calculate_size(self.as_mut_ptr()) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    pub fn iter_owned(&self) -> IterOwned {
        IterOwned::new(self)
    }

    #[doc(alias = "gst_buffer_list_foreach")]
    pub fn foreach<F: FnMut(&Buffer, u32) -> ControlFlow<(), ()>>(&self, func: F) -> bool {
        unsafe extern "C" fn trampoline<F: FnMut(&Buffer, u32) -> ControlFlow<(), ()>>(
            buffer: *mut *mut ffi::GstBuffer,
            idx: u32,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let func = user_data as *const _ as usize as *mut F;
            let res = (*func)(&Buffer::from_glib_borrow(*buffer), idx);

            matches!(res, ControlFlow::Continue(_)).into_glib()
        }

        unsafe {
            let func_ptr: &F = &func;

            from_glib(ffi::gst_buffer_list_foreach(
                self.as_ptr() as *mut _,
                Some(trampoline::<F>),
                func_ptr as *const _ as usize as *mut _,
            ))
        }
    }

    #[doc(alias = "gst_buffer_list_foreach")]
    pub fn foreach_mut<F: FnMut(Buffer, u32) -> ControlFlow<Option<Buffer>, Option<Buffer>>>(
        &mut self,
        func: F,
    ) -> bool {
        unsafe extern "C" fn trampoline<
            F: FnMut(Buffer, u32) -> ControlFlow<Option<Buffer>, Option<Buffer>>,
        >(
            buffer: *mut *mut ffi::GstBuffer,
            idx: u32,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let func = user_data as *const _ as usize as *mut F;
            let res = (*func)(
                Buffer::from_glib_full(
                    ptr::replace(buffer, ptr::null_mut::<ffi::GstBuffer>()) as *mut ffi::GstBuffer
                ),
                idx,
            );

            let (cont, res_buffer) = match res {
                ControlFlow::Continue(res_buffer) => (true, res_buffer),
                ControlFlow::Break(res_buffer) => (false, res_buffer),
            };

            match res_buffer {
                None => {
                    *buffer = ptr::null_mut();
                }
                Some(new_buffer) => {
                    *buffer = new_buffer.into_glib_ptr();
                }
            }

            cont.into_glib()
        }

        unsafe {
            let func_ptr: &F = &func;

            from_glib(ffi::gst_buffer_list_foreach(
                self.as_ptr() as *mut _,
                Some(trampoline::<F>),
                func_ptr as *const _ as usize as *mut _,
            ))
        }
    }
}

impl Default for BufferList {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for BufferList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        BufferListRef::fmt(self, f)
    }
}

impl fmt::Debug for BufferListRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::utils::Displayable;
        use crate::ClockTime;

        let size = self.iter().map(|b| b.size()).sum::<usize>();
        let (pts, dts) = self
            .get(0)
            .map(|b| (b.pts(), b.dts()))
            .unwrap_or((ClockTime::NONE, ClockTime::NONE));

        f.debug_struct("BufferList")
            .field("ptr", &self.as_ptr())
            .field("buffers", &self.len())
            .field("pts", &pts.display())
            .field("dts", &dts.display())
            .field("size", &size)
            .finish()
    }
}

macro_rules! define_iter(
    ($name:ident, $styp:ty, $get_item:expr) => {
    #[derive(Debug)]
    pub struct $name<'a> {
        list: &'a BufferListRef,
        idx: usize,
        size: usize,
    }

    impl<'a> $name<'a> {
        fn new(list: &'a BufferListRef) -> $name<'a> {
            skip_assert_initialized!();
            $name {
                list,
                idx: 0,
                size: list.len() as usize,
            }
        }
    }

    impl<'a> Iterator for $name<'a> {
        type Item = $styp;

        fn next(&mut self) -> Option<Self::Item> {
            if self.idx >= self.size {
                return None;
            }

            let item = $get_item(self.list, self.idx as u32).unwrap();
            self.idx += 1;

            Some(item)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let remaining = self.size - self.idx;

            (remaining, Some(remaining))
        }

        fn count(self) -> usize {
            self.size - self.idx
        }

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            let (end, overflow) = self.idx.overflowing_add(n);
            if end >= self.size || overflow {
                self.idx = self.size;
                None
            } else {
                self.idx = end + 1;
                Some($get_item(self.list, end as u32).unwrap())
            }
        }

        fn last(self) -> Option<Self::Item> {
            if self.idx == self.size {
                None
            } else {
                Some($get_item(self.list, self.size as u32 - 1).unwrap())
            }
        }
    }

    impl<'a> DoubleEndedIterator for $name<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.idx == self.size {
                return None;
            }

            self.size -= 1;
            Some($get_item(self.list, self.size as u32).unwrap())
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            let (end, overflow) = self.size.overflowing_sub(n);
            if end <= self.idx || overflow {
                self.idx = self.size;
                None
            } else {
                self.size = end - 1;
                Some($get_item(self.list, self.size as u32).unwrap())
            }
        }
    }

    impl<'a> ExactSizeIterator for $name<'a> {}
    impl<'a> std::iter::FusedIterator for $name<'a> {}
    }
);

define_iter!(Iter, &'a BufferRef, |list: &'a BufferListRef, idx| {
    list.get(idx)
});

define_iter!(IterOwned, Buffer, |list: &BufferListRef, idx| {
    list.get_owned(idx)
});

impl<'a> IntoIterator for &'a BufferListRef {
    type IntoIter = Iter<'a>;
    type Item = &'a BufferRef;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl std::iter::FromIterator<Buffer> for BufferList {
    fn from_iter<T: IntoIterator<Item = Buffer>>(iter: T) -> Self {
        assert_initialized_main_thread!();

        let iter = iter.into_iter();

        let mut list = BufferList::new_sized(iter.size_hint().0);

        {
            let list = list.get_mut().unwrap();
            iter.for_each(|b| list.add(b));
        }

        list
    }
}

impl std::iter::Extend<Buffer> for BufferListRef {
    fn extend<T: IntoIterator<Item = Buffer>>(&mut self, iter: T) {
        iter.into_iter().for_each(|b| self.add(b));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClockTime;

    #[test]
    fn test_foreach() {
        crate::init().unwrap();

        let mut buffer_list = BufferList::new();
        {
            let buffer_list = buffer_list.get_mut().unwrap();
            let mut buffer = Buffer::new();
            buffer.get_mut().unwrap().set_pts(ClockTime::ZERO);
            buffer_list.add(buffer);

            let mut buffer = Buffer::new();
            buffer.get_mut().unwrap().set_pts(ClockTime::SECOND);
            buffer_list.add(buffer);
        }

        let mut res = vec![];
        buffer_list.foreach(|buffer, idx| {
            res.push((buffer.pts(), idx));
            ControlFlow::Continue(())
        });

        assert_eq!(
            res,
            &[(Some(ClockTime::ZERO), 0), (Some(ClockTime::SECOND), 1)]
        );
    }

    #[test]
    fn test_foreach_mut() {
        crate::init().unwrap();

        let mut buffer_list = BufferList::new();
        {
            let buffer_list = buffer_list.get_mut().unwrap();
            let mut buffer = Buffer::new();
            buffer.get_mut().unwrap().set_pts(ClockTime::ZERO);
            buffer_list.add(buffer);

            let mut buffer = Buffer::new();
            buffer.get_mut().unwrap().set_pts(ClockTime::SECOND);
            buffer_list.add(buffer);

            let mut buffer = Buffer::new();
            buffer.get_mut().unwrap().set_pts(2 * ClockTime::SECOND);
            buffer_list.add(buffer);
        }

        let mut res = vec![];
        buffer_list.get_mut().unwrap().foreach_mut(|buffer, idx| {
            res.push((buffer.pts(), idx));

            if let Some(ClockTime::ZERO) = buffer.pts() {
                ControlFlow::Continue(Some(buffer))
            } else if let Some(ClockTime::SECOND) = buffer.pts() {
                ControlFlow::Continue(None)
            } else {
                let mut new_buffer = Buffer::new();
                new_buffer.get_mut().unwrap().set_pts(3 * ClockTime::SECOND);
                ControlFlow::Continue(Some(new_buffer))
            }
        });

        assert_eq!(
            res,
            &[
                (Some(ClockTime::ZERO), 0),
                (Some(ClockTime::SECOND), 1),
                (Some(2 * ClockTime::SECOND), 1)
            ]
        );

        let mut res = vec![];
        buffer_list.foreach(|buffer, idx| {
            res.push((buffer.pts(), idx));
            ControlFlow::Continue(())
        });

        assert_eq!(
            res,
            &[(Some(ClockTime::ZERO), 0), (Some(3 * ClockTime::SECOND), 1)]
        );
    }
}
