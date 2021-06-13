// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstAudioBaseSink")]
    pub struct AudioBaseSink(Object<ffi::GstAudioBaseSink, ffi::GstAudioBaseSinkClass>) @extends gst_base::BaseSink, gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_audio_base_sink_get_type(),
    }
}

unsafe impl Send for AudioBaseSink {}
unsafe impl Sync for AudioBaseSink {}

pub const NONE_AUDIO_BASE_SINK: Option<&AudioBaseSink> = None;

pub trait AudioBaseSinkExt: 'static {
    //#[doc(alias = "gst_audio_base_sink_create_ringbuffer")]
    //fn create_ringbuffer(&self) -> /*Ignored*/Option<AudioRingBuffer>;

    #[doc(alias = "gst_audio_base_sink_get_alignment_threshold")]
    #[doc(alias = "get_alignment_threshold")]
    fn alignment_threshold(&self) -> gst::ClockTime;

    #[doc(alias = "gst_audio_base_sink_get_discont_wait")]
    #[doc(alias = "get_discont_wait")]
    fn discont_wait(&self) -> gst::ClockTime;

    #[doc(alias = "gst_audio_base_sink_get_drift_tolerance")]
    #[doc(alias = "get_drift_tolerance")]
    fn drift_tolerance(&self) -> i64;

    #[doc(alias = "gst_audio_base_sink_get_provide_clock")]
    #[doc(alias = "get_provide_clock")]
    fn is_provide_clock(&self) -> bool;

    //#[doc(alias = "gst_audio_base_sink_get_slave_method")]
    //#[doc(alias = "get_slave_method")]
    //fn slave_method(&self) -> /*Ignored*/AudioBaseSinkSlaveMethod;

    #[doc(alias = "gst_audio_base_sink_report_device_failure")]
    fn report_device_failure(&self);

    #[doc(alias = "gst_audio_base_sink_set_alignment_threshold")]
    fn set_alignment_threshold(&self, alignment_threshold: gst::ClockTime);

    //#[doc(alias = "gst_audio_base_sink_set_custom_slaving_callback")]
    //fn set_custom_slaving_callback(&self, callback: /*Unimplemented*/Fn(&AudioBaseSink, impl Into<Option<gst::ClockTime>>, impl Into<Option<gst::ClockTime>>, gst::ClockTimeDiff, /*Ignored*/AudioBaseSinkDiscontReason), user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    #[doc(alias = "gst_audio_base_sink_set_discont_wait")]
    fn set_discont_wait(&self, discont_wait: gst::ClockTime);

    #[doc(alias = "gst_audio_base_sink_set_drift_tolerance")]
    fn set_drift_tolerance(&self, drift_tolerance: i64);

    #[doc(alias = "gst_audio_base_sink_set_provide_clock")]
    fn set_provide_clock(&self, provide: bool);

    //#[doc(alias = "gst_audio_base_sink_set_slave_method")]
    //fn set_slave_method(&self, method: /*Ignored*/AudioBaseSinkSlaveMethod);

    #[doc(alias = "buffer-time")]
    fn buffer_time(&self) -> i64;

    #[doc(alias = "buffer-time")]
    fn set_buffer_time(&self, buffer_time: i64);

    #[doc(alias = "can-activate-pull")]
    fn can_activate_pull(&self) -> bool;

    #[doc(alias = "can-activate-pull")]
    fn set_can_activate_pull(&self, can_activate_pull: bool);

    #[doc(alias = "latency-time")]
    fn latency_time(&self) -> i64;

    #[doc(alias = "latency-time")]
    fn set_latency_time(&self, latency_time: i64);

    #[doc(alias = "alignment-threshold")]
    fn connect_alignment_threshold_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "buffer-time")]
    fn connect_buffer_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "can-activate-pull")]
    fn connect_can_activate_pull_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "discont-wait")]
    fn connect_discont_wait_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "drift-tolerance")]
    fn connect_drift_tolerance_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "latency-time")]
    fn connect_latency_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "provide-clock")]
    fn connect_provide_clock_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "slave-method")]
    fn connect_slave_method_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<AudioBaseSink>> AudioBaseSinkExt for O {
    //fn create_ringbuffer(&self) -> /*Ignored*/Option<AudioRingBuffer> {
    //    unsafe { TODO: call ffi:gst_audio_base_sink_create_ringbuffer() }
    //}

    fn alignment_threshold(&self) -> gst::ClockTime {
        unsafe {
            try_from_glib(ffi::gst_audio_base_sink_get_alignment_threshold(
                self.as_ref().to_glib_none().0,
            ))
            .expect("mandatory glib value is None")
        }
    }

    fn discont_wait(&self) -> gst::ClockTime {
        unsafe {
            try_from_glib(ffi::gst_audio_base_sink_get_discont_wait(
                self.as_ref().to_glib_none().0,
            ))
            .expect("mandatory glib value is None")
        }
    }

    fn drift_tolerance(&self) -> i64 {
        unsafe { ffi::gst_audio_base_sink_get_drift_tolerance(self.as_ref().to_glib_none().0) }
    }

    fn is_provide_clock(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_audio_base_sink_get_provide_clock(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn slave_method(&self) -> /*Ignored*/AudioBaseSinkSlaveMethod {
    //    unsafe { TODO: call ffi:gst_audio_base_sink_get_slave_method() }
    //}

    fn report_device_failure(&self) {
        unsafe {
            ffi::gst_audio_base_sink_report_device_failure(self.as_ref().to_glib_none().0);
        }
    }

    fn set_alignment_threshold(&self, alignment_threshold: gst::ClockTime) {
        unsafe {
            ffi::gst_audio_base_sink_set_alignment_threshold(
                self.as_ref().to_glib_none().0,
                alignment_threshold.into_glib(),
            );
        }
    }

    //fn set_custom_slaving_callback(&self, callback: /*Unimplemented*/Fn(&AudioBaseSink, impl Into<Option<gst::ClockTime>>, impl Into<Option<gst::ClockTime>>, gst::ClockTimeDiff, /*Ignored*/AudioBaseSinkDiscontReason), user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:gst_audio_base_sink_set_custom_slaving_callback() }
    //}

    fn set_discont_wait(&self, discont_wait: gst::ClockTime) {
        unsafe {
            ffi::gst_audio_base_sink_set_discont_wait(
                self.as_ref().to_glib_none().0,
                discont_wait.into_glib(),
            );
        }
    }

    fn set_drift_tolerance(&self, drift_tolerance: i64) {
        unsafe {
            ffi::gst_audio_base_sink_set_drift_tolerance(
                self.as_ref().to_glib_none().0,
                drift_tolerance,
            );
        }
    }

    fn set_provide_clock(&self, provide: bool) {
        unsafe {
            ffi::gst_audio_base_sink_set_provide_clock(
                self.as_ref().to_glib_none().0,
                provide.into_glib(),
            );
        }
    }

    //fn set_slave_method(&self, method: /*Ignored*/AudioBaseSinkSlaveMethod) {
    //    unsafe { TODO: call ffi:gst_audio_base_sink_set_slave_method() }
    //}

    fn buffer_time(&self) -> i64 {
        unsafe {
            let mut value = glib::Value::from_type(<i64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"buffer-time\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `buffer-time` getter")
        }
    }

    fn set_buffer_time(&self, buffer_time: i64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"buffer-time\0".as_ptr() as *const _,
                buffer_time.to_value().to_glib_none().0,
            );
        }
    }

    fn can_activate_pull(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"can-activate-pull\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `can-activate-pull` getter")
        }
    }

    fn set_can_activate_pull(&self, can_activate_pull: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"can-activate-pull\0".as_ptr() as *const _,
                can_activate_pull.to_value().to_glib_none().0,
            );
        }
    }

    fn latency_time(&self) -> i64 {
        unsafe {
            let mut value = glib::Value::from_type(<i64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"latency-time\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `latency-time` getter")
        }
    }

    fn set_latency_time(&self, latency_time: i64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"latency-time\0".as_ptr() as *const _,
                latency_time.to_value().to_glib_none().0,
            );
        }
    }

    fn connect_alignment_threshold_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_alignment_threshold_trampoline<
            P: IsA<AudioBaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AudioBaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::alignment-threshold\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_alignment_threshold_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_buffer_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffer_time_trampoline<
            P: IsA<AudioBaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AudioBaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::buffer-time\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_buffer_time_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_can_activate_pull_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_activate_pull_trampoline<
            P: IsA<AudioBaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AudioBaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-activate-pull\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_can_activate_pull_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_discont_wait_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_discont_wait_trampoline<
            P: IsA<AudioBaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AudioBaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::discont-wait\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_discont_wait_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_drift_tolerance_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_drift_tolerance_trampoline<
            P: IsA<AudioBaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AudioBaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::drift-tolerance\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_drift_tolerance_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_latency_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_latency_time_trampoline<
            P: IsA<AudioBaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AudioBaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::latency-time\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_latency_time_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_provide_clock_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_provide_clock_trampoline<
            P: IsA<AudioBaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AudioBaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::provide-clock\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_provide_clock_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_slave_method_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_slave_method_trampoline<
            P: IsA<AudioBaseSink>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioBaseSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AudioBaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::slave-method\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_slave_method_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
