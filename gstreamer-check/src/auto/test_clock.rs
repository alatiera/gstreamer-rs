// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    pub struct TestClock(Object<ffi::GstTestClock, ffi::GstTestClockClass>) @extends gst::Clock, gst::Object;

    match fn {
        type_ => || ffi::gst_test_clock_get_type(),
    }
}

impl TestClock {
    #[doc(alias = "gst_test_clock_new")]
    pub fn new() -> TestClock {
        assert_initialized_main_thread!();
        unsafe { gst::Clock::from_glib_full(ffi::gst_test_clock_new()).unsafe_cast() }
    }

    #[doc(alias = "gst_test_clock_new_with_start_time")]
    pub fn with_start_time(start_time: gst::ClockTime) -> TestClock {
        assert_initialized_main_thread!();
        unsafe {
            gst::Clock::from_glib_full(ffi::gst_test_clock_new_with_start_time(
                start_time.to_glib(),
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gst_test_clock_advance_time")]
    pub fn advance_time(&self, delta: gst::ClockTimeDiff) {
        unsafe {
            ffi::gst_test_clock_advance_time(self.to_glib_none().0, delta);
        }
    }

    #[doc(alias = "gst_test_clock_crank")]
    pub fn crank(&self) -> bool {
        unsafe { from_glib(ffi::gst_test_clock_crank(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_test_clock_get_next_entry_time")]
    pub fn next_entry_time(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_test_clock_get_next_entry_time(
                self.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "gst_test_clock_has_id")]
    //pub fn has_id(&self, id: /*Ignored*/gst::ClockID) -> bool {
    //    unsafe { TODO: call ffi:gst_test_clock_has_id() }
    //}

    #[doc(alias = "gst_test_clock_peek_id_count")]
    pub fn peek_id_count(&self) -> u32 {
        unsafe { ffi::gst_test_clock_peek_id_count(self.to_glib_none().0) }
    }

    //#[doc(alias = "gst_test_clock_peek_next_pending_id")]
    //pub fn peek_next_pending_id(&self, pending_id: /*Ignored*/&mut gst::ClockID) -> bool {
    //    unsafe { TODO: call ffi:gst_test_clock_peek_next_pending_id() }
    //}

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    //#[doc(alias = "gst_test_clock_process_id")]
    //pub fn process_id(&self, pending_id: /*Ignored*/gst::ClockID) -> bool {
    //    unsafe { TODO: call ffi:gst_test_clock_process_id() }
    //}

    //#[doc(alias = "gst_test_clock_process_id_list")]
    //pub fn process_id_list(&self, pending_list: /*Ignored*/&[&gst::ClockID]) -> u32 {
    //    unsafe { TODO: call ffi:gst_test_clock_process_id_list() }
    //}

    //#[doc(alias = "gst_test_clock_process_next_clock_id")]
    //pub fn process_next_clock_id(&self) -> /*Ignored*/Option<gst::ClockID> {
    //    unsafe { TODO: call ffi:gst_test_clock_process_next_clock_id() }
    //}

    #[doc(alias = "gst_test_clock_set_time")]
    pub fn set_time(&self, new_time: gst::ClockTime) {
        unsafe {
            ffi::gst_test_clock_set_time(self.to_glib_none().0, new_time.to_glib());
        }
    }

    //#[cfg(any(feature = "v1_16", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    //#[doc(alias = "gst_test_clock_timed_wait_for_multiple_pending_ids")]
    //pub fn timed_wait_for_multiple_pending_ids(&self, count: u32, timeout_ms: u32, pending_list: /*Unimplemented*/Vec<gst::ClockID>) -> bool {
    //    unsafe { TODO: call ffi:gst_test_clock_timed_wait_for_multiple_pending_ids() }
    //}

    //#[doc(alias = "gst_test_clock_wait_for_multiple_pending_ids")]
    //pub fn wait_for_multiple_pending_ids(&self, count: u32, pending_list: /*Unimplemented*/Vec<gst::ClockID>) {
    //    unsafe { TODO: call ffi:gst_test_clock_wait_for_multiple_pending_ids() }
    //}

    //#[doc(alias = "gst_test_clock_wait_for_next_pending_id")]
    //pub fn wait_for_next_pending_id(&self, pending_id: /*Ignored*/&mut gst::ClockID) {
    //    unsafe { TODO: call ffi:gst_test_clock_wait_for_next_pending_id() }
    //}

    #[doc(alias = "gst_test_clock_wait_for_pending_id_count")]
    pub fn wait_for_pending_id_count(&self, count: u32) {
        unsafe {
            ffi::gst_test_clock_wait_for_pending_id_count(self.to_glib_none().0, count);
        }
    }

    #[doc(alias = "get_property_clock_type")]
    pub fn clock_type(&self) -> gst::ClockType {
        unsafe {
            let mut value = glib::Value::from_type(<gst::ClockType as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"clock-type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `clock-type` getter")
        }
    }

    #[doc(alias = "set_property_clock_type")]
    pub fn set_clock_type(&self, clock_type: gst::ClockType) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"clock-type\0".as_ptr() as *const _,
                clock_type.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_start_time")]
    pub fn start_time(&self) -> u64 {
        unsafe {
            let mut value = glib::Value::from_type(<u64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"start-time\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `start-time` getter")
        }
    }

    //#[doc(alias = "gst_test_clock_id_list_get_latest_time")]
    //pub fn id_list_get_latest_time(pending_list: /*Ignored*/&[&gst::ClockID]) -> gst::ClockTime {
    //    unsafe { TODO: call ffi:gst_test_clock_id_list_get_latest_time() }
    //}

    pub fn connect_property_clock_type_notify<F: Fn(&TestClock) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_clock_type_trampoline<
            F: Fn(&TestClock) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstTestClock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::clock-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_clock_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for TestClock {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for TestClock {}
unsafe impl Sync for TestClock {}
