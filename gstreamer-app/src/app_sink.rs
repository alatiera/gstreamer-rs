// Take a look at the license at the top of the repository in the LICENSE file.

use crate::AppSink;
use glib::ffi::gpointer;
use glib::prelude::*;
use glib::translate::*;
use std::mem;
use std::panic;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};

use {
    futures_core::Stream,
    std::{
        pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, Poll, Waker},
    },
};

#[allow(clippy::type_complexity)]
pub struct AppSinkCallbacks {
    eos: Option<Box<dyn FnMut(&AppSink) + Send + 'static>>,
    new_preroll: Option<
        Box<dyn FnMut(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static>,
    >,
    new_sample: Option<
        Box<dyn FnMut(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static>,
    >,
    new_event: Option<Box<dyn FnMut(&AppSink) -> bool + Send + 'static>>,
    panicked: AtomicBool,
    callbacks: ffi::GstAppSinkCallbacks,
}

unsafe impl Send for AppSinkCallbacks {}
unsafe impl Sync for AppSinkCallbacks {}

impl AppSinkCallbacks {
    pub fn builder() -> AppSinkCallbacksBuilder {
        skip_assert_initialized!();
        AppSinkCallbacksBuilder {
            eos: None,
            new_preroll: None,
            new_sample: None,
            new_event: None,
        }
    }
}

#[allow(clippy::type_complexity)]
#[must_use = "The builder must be built to be used"]
pub struct AppSinkCallbacksBuilder {
    eos: Option<Box<dyn FnMut(&AppSink) + Send + 'static>>,
    new_preroll: Option<
        Box<dyn FnMut(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static>,
    >,
    new_sample: Option<
        Box<dyn FnMut(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static>,
    >,
    new_event: Option<Box<dyn FnMut(&AppSink) -> bool + Send + 'static>>,
}

impl AppSinkCallbacksBuilder {
    pub fn eos<F: FnMut(&AppSink) + Send + 'static>(self, eos: F) -> Self {
        Self {
            eos: Some(Box::new(eos)),
            ..self
        }
    }

    pub fn new_preroll<
        F: FnMut(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static,
    >(
        self,
        new_preroll: F,
    ) -> Self {
        Self {
            new_preroll: Some(Box::new(new_preroll)),
            ..self
        }
    }

    pub fn new_sample<
        F: FnMut(&AppSink) -> Result<gst::FlowSuccess, gst::FlowError> + Send + 'static,
    >(
        self,
        new_sample: F,
    ) -> Self {
        Self {
            new_sample: Some(Box::new(new_sample)),
            ..self
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn new_event<F: FnMut(&AppSink) -> bool + Send + 'static>(self, new_event: F) -> Self {
        Self {
            new_event: Some(Box::new(new_event)),
            ..self
        }
    }

    #[must_use = "Building the callbacks without using them has no effect"]
    pub fn build(self) -> AppSinkCallbacks {
        let have_eos = self.eos.is_some();
        let have_new_preroll = self.new_preroll.is_some();
        let have_new_sample = self.new_sample.is_some();
        let have_new_event = self.new_event.is_some();

        AppSinkCallbacks {
            eos: self.eos,
            new_preroll: self.new_preroll,
            new_sample: self.new_sample,
            new_event: self.new_event,
            panicked: AtomicBool::new(false),
            callbacks: ffi::GstAppSinkCallbacks {
                eos: if have_eos { Some(trampoline_eos) } else { None },
                new_preroll: if have_new_preroll {
                    Some(trampoline_new_preroll)
                } else {
                    None
                },
                new_sample: if have_new_sample {
                    Some(trampoline_new_sample)
                } else {
                    None
                },
                new_event: if have_new_event {
                    Some(trampoline_new_event)
                } else {
                    None
                },
                _gst_reserved: [ptr::null_mut(), ptr::null_mut(), ptr::null_mut()],
            },
        }
    }
}

fn post_panic_error_message(element: &AppSink, err: &dyn std::any::Any) {
    skip_assert_initialized!();
    if let Some(cause) = err.downcast_ref::<&str>() {
        gst::element_error!(element, gst::LibraryError::Failed, ["Panicked: {}", cause]);
    } else if let Some(cause) = err.downcast_ref::<String>() {
        gst::element_error!(element, gst::LibraryError::Failed, ["Panicked: {}", cause]);
    } else {
        gst::element_error!(element, gst::LibraryError::Failed, ["Panicked"]);
    }
}

unsafe extern "C" fn trampoline_eos(appsink: *mut ffi::GstAppSink, callbacks: gpointer) {
    let callbacks = callbacks as *mut AppSinkCallbacks;
    let element: Borrowed<AppSink> = from_glib_borrow(appsink);

    if (*callbacks).panicked.load(Ordering::Relaxed) {
        let element: Borrowed<AppSink> = from_glib_borrow(appsink);
        gst::element_error!(element, gst::LibraryError::Failed, ["Panicked"]);
        return;
    }

    if let Some(ref mut eos) = (*callbacks).eos {
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| eos(&element)));
        match result {
            Ok(result) => result,
            Err(err) => {
                (*callbacks).panicked.store(true, Ordering::Relaxed);
                post_panic_error_message(&element, &err);
            }
        }
    }
}

unsafe extern "C" fn trampoline_new_preroll(
    appsink: *mut ffi::GstAppSink,
    callbacks: gpointer,
) -> gst::ffi::GstFlowReturn {
    let callbacks = callbacks as *mut AppSinkCallbacks;
    let element: Borrowed<AppSink> = from_glib_borrow(appsink);

    if (*callbacks).panicked.load(Ordering::Relaxed) {
        let element: Borrowed<AppSink> = from_glib_borrow(appsink);
        gst::element_error!(element, gst::LibraryError::Failed, ["Panicked"]);
        return gst::FlowReturn::Error.into_glib();
    }

    let ret = if let Some(ref mut new_preroll) = (*callbacks).new_preroll {
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| new_preroll(&element).into()));
        match result {
            Ok(result) => result,
            Err(err) => {
                (*callbacks).panicked.store(true, Ordering::Relaxed);
                post_panic_error_message(&element, &err);

                gst::FlowReturn::Error
            }
        }
    } else {
        gst::FlowReturn::Error
    };

    ret.into_glib()
}

unsafe extern "C" fn trampoline_new_sample(
    appsink: *mut ffi::GstAppSink,
    callbacks: gpointer,
) -> gst::ffi::GstFlowReturn {
    let callbacks = callbacks as *mut AppSinkCallbacks;
    let element: Borrowed<AppSink> = from_glib_borrow(appsink);

    if (*callbacks).panicked.load(Ordering::Relaxed) {
        let element: Borrowed<AppSink> = from_glib_borrow(appsink);
        gst::element_error!(element, gst::LibraryError::Failed, ["Panicked"]);
        return gst::FlowReturn::Error.into_glib();
    }

    let ret = if let Some(ref mut new_sample) = (*callbacks).new_sample {
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| new_sample(&element).into()));
        match result {
            Ok(result) => result,
            Err(err) => {
                (*callbacks).panicked.store(true, Ordering::Relaxed);
                post_panic_error_message(&element, &err);

                gst::FlowReturn::Error
            }
        }
    } else {
        gst::FlowReturn::Error
    };

    ret.into_glib()
}

unsafe extern "C" fn trampoline_new_event(
    appsink: *mut ffi::GstAppSink,
    callbacks: gpointer,
) -> glib::ffi::gboolean {
    let callbacks = callbacks as *mut AppSinkCallbacks;
    let element: Borrowed<AppSink> = from_glib_borrow(appsink);

    if (*callbacks).panicked.load(Ordering::Relaxed) {
        let element: Borrowed<AppSink> = from_glib_borrow(appsink);
        gst::element_error!(element, gst::LibraryError::Failed, ["Panicked"]);
        return false.into_glib();
    }

    let ret = if let Some(ref mut new_event) = (*callbacks).new_event {
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| new_event(&element)));
        match result {
            Ok(result) => result,
            Err(err) => {
                (*callbacks).panicked.store(true, Ordering::Relaxed);
                post_panic_error_message(&element, &err);

                false
            }
        }
    } else {
        false
    };

    ret.into_glib()
}

unsafe extern "C" fn destroy_callbacks(ptr: gpointer) {
    let _ = Box::<AppSinkCallbacks>::from_raw(ptr as *mut _);
}

impl AppSink {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`AppSink`] objects.
    ///
    /// This method returns an instance of [`AppSinkBuilder`](crate::builders::AppSinkBuilder) which can be used to create [`AppSink`] objects.
    pub fn builder() -> AppSinkBuilder {
        AppSinkBuilder::default()
    }

    #[doc(alias = "gst_app_sink_set_callbacks")]
    pub fn set_callbacks(&self, callbacks: AppSinkCallbacks) {
        use once_cell::sync::Lazy;
        static SET_ONCE_QUARK: Lazy<glib::Quark> =
            Lazy::new(|| glib::Quark::from_str("gstreamer-rs-app-sink-callbacks"));

        unsafe {
            let sink = self.to_glib_none().0;

            // This is not thread-safe before 1.16.3, see
            // https://gitlab.freedesktop.org/gstreamer/gst-plugins-base/merge_requests/570
            if gst::version() < (1, 16, 3, 0) {
                if !glib::gobject_ffi::g_object_get_qdata(
                    sink as *mut _,
                    SET_ONCE_QUARK.into_glib(),
                )
                .is_null()
                {
                    panic!("AppSink callbacks can only be set once");
                }

                glib::gobject_ffi::g_object_set_qdata(
                    sink as *mut _,
                    SET_ONCE_QUARK.into_glib(),
                    1 as *mut _,
                );
            }

            ffi::gst_app_sink_set_callbacks(
                sink,
                mut_override(&callbacks.callbacks),
                Box::into_raw(Box::new(callbacks)) as *mut _,
                Some(destroy_callbacks),
            );
        }
    }

    #[doc(alias = "drop-out-of-segment")]
    pub fn drops_out_of_segment(&self) -> bool {
        unsafe {
            from_glib(gst_base::ffi::gst_base_sink_get_drop_out_of_segment(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink,
            ))
        }
    }

    #[doc(alias = "max-bitrate")]
    #[doc(alias = "gst_base_sink_get_max_bitrate")]
    pub fn max_bitrate(&self) -> u64 {
        unsafe {
            gst_base::ffi::gst_base_sink_get_max_bitrate(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink
            )
        }
    }

    #[doc(alias = "max-lateness")]
    #[doc(alias = "gst_base_sink_get_max_lateness")]
    pub fn max_lateness(&self) -> i64 {
        unsafe {
            gst_base::ffi::gst_base_sink_get_max_lateness(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink
            )
        }
    }

    #[doc(alias = "processing-deadline")]
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_base_sink_get_processing_deadline")]
    pub fn processing_deadline(&self) -> gst::ClockTime {
        unsafe {
            try_from_glib(gst_base::ffi::gst_base_sink_get_processing_deadline(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink,
            ))
            .expect("undefined processing_deadline")
        }
    }

    #[doc(alias = "render-delay")]
    #[doc(alias = "gst_base_sink_get_render_delay")]
    pub fn render_delay(&self) -> gst::ClockTime {
        unsafe {
            try_from_glib(gst_base::ffi::gst_base_sink_get_render_delay(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink
            ))
            .expect("undefined render_delay")
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_base_sink_get_stats")]
    pub fn stats(&self) -> gst::Structure {
        unsafe {
            from_glib_full(gst_base::ffi::gst_base_sink_get_stats(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink
            ))
        }
    }

    #[doc(alias = "sync")]
    pub fn is_sync(&self) -> bool {
        unsafe {
            from_glib(gst_base::ffi::gst_base_sink_get_sync(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink
            ))
        }
    }

    #[doc(alias = "throttle-time")]
    #[doc(alias = "gst_base_sink_get_throttle_time")]
    pub fn throttle_time(&self) -> u64 {
        unsafe {
            gst_base::ffi::gst_base_sink_get_throttle_time(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink
            )
        }
    }

    #[doc(alias = "ts-offset")]
    #[doc(alias = "gst_base_sink_get_ts_offset")]
    pub fn ts_offset(&self) -> gst::ClockTimeDiff {
        unsafe {
            gst_base::ffi::gst_base_sink_get_ts_offset(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink
            )
        }
    }

    #[doc(alias = "async")]
    #[doc(alias = "gst_base_sink_is_async_enabled")]
    pub fn is_async(&self) -> bool {
        unsafe {
            from_glib(gst_base::ffi::gst_base_sink_is_async_enabled(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink
            ))
        }
    }

    #[doc(alias = "last-sample")]
    pub fn enables_last_sample(&self) -> bool {
        unsafe {
            from_glib(gst_base::ffi::gst_base_sink_is_last_sample_enabled(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink,
            ))
        }
    }

    #[doc(alias = "qos")]
    #[doc(alias = "gst_base_sink_is_qos_enabled")]
    pub fn is_qos(&self) -> bool {
        unsafe {
            from_glib(gst_base::ffi::gst_base_sink_is_qos_enabled(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink
            ))
        }
    }

    #[doc(alias = "async")]
    #[doc(alias = "gst_base_sink_set_async_enabled")]
    pub fn set_async(&self, enabled: bool) {
        unsafe {
            gst_base::ffi::gst_base_sink_set_async_enabled(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink,
                enabled.into_glib(),
            );
        }
    }

    #[doc(alias = "drop-out-of-segment")]
    #[doc(alias = "gst_base_sink_set_drop_out_of_segment")]
    pub fn set_drop_out_of_segment(&self, drop_out_of_segment: bool) {
        unsafe {
            gst_base::ffi::gst_base_sink_set_drop_out_of_segment(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink,
                drop_out_of_segment.into_glib(),
            );
        }
    }

    #[doc(alias = "last-sample")]
    pub fn set_enable_last_sample(&self, enabled: bool) {
        unsafe {
            gst_base::ffi::gst_base_sink_set_last_sample_enabled(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink,
                enabled.into_glib(),
            );
        }
    }

    #[doc(alias = "max-bitrate")]
    #[doc(alias = "gst_base_sink_set_max_bitrate")]
    pub fn set_max_bitrate(&self, max_bitrate: u64) {
        unsafe {
            gst_base::ffi::gst_base_sink_set_max_bitrate(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink,
                max_bitrate,
            );
        }
    }

    #[doc(alias = "max-lateness")]
    #[doc(alias = "gst_base_sink_set_max_lateness")]
    pub fn set_max_lateness(&self, max_lateness: i64) {
        unsafe {
            gst_base::ffi::gst_base_sink_set_max_lateness(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink,
                max_lateness,
            );
        }
    }

    #[doc(alias = "processing-deadline")]
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_base_sink_set_processing_deadline")]
    pub fn set_processing_deadline(&self, processing_deadline: gst::ClockTime) {
        unsafe {
            gst_base::ffi::gst_base_sink_set_processing_deadline(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink,
                processing_deadline.into_glib(),
            );
        }
    }

    #[doc(alias = "qos")]
    #[doc(alias = "gst_base_sink_set_qos_enabled")]
    pub fn set_qos(&self, enabled: bool) {
        unsafe {
            gst_base::ffi::gst_base_sink_set_qos_enabled(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink,
                enabled.into_glib(),
            );
        }
    }

    #[doc(alias = "render-delay")]
    #[doc(alias = "gst_base_sink_set_render_delay")]
    pub fn set_render_delay(&self, delay: gst::ClockTime) {
        unsafe {
            gst_base::ffi::gst_base_sink_set_render_delay(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink,
                delay.into_glib(),
            );
        }
    }

    #[doc(alias = "sync")]
    #[doc(alias = "gst_base_sink_set_sync")]
    pub fn set_sync(&self, sync: bool) {
        unsafe {
            gst_base::ffi::gst_base_sink_set_sync(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink,
                sync.into_glib(),
            );
        }
    }

    #[doc(alias = "throttle-time")]
    #[doc(alias = "gst_base_sink_set_throttle_time")]
    pub fn set_throttle_time(&self, throttle: u64) {
        unsafe {
            gst_base::ffi::gst_base_sink_set_throttle_time(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink,
                throttle,
            );
        }
    }

    #[doc(alias = "ts-offset")]
    #[doc(alias = "gst_base_sink_set_ts_offset")]
    pub fn set_ts_offset(&self, offset: gst::ClockTimeDiff) {
        unsafe {
            gst_base::ffi::gst_base_sink_set_ts_offset(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSink,
                offset,
            );
        }
    }

    #[doc(alias = "async")]
    pub fn connect_async_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        unsafe extern "C" fn notify_async_trampoline<F: Fn(&AppSink) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"notify::async\0".as_ptr() as *const _,
                Some(mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_async_trampoline::<F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    #[doc(alias = "blocksize")]
    pub fn connect_blocksize_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        unsafe extern "C" fn notify_blocksize_trampoline<
            F: Fn(&AppSink) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"notify::blocksize\0".as_ptr() as *const _,
                Some(mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_blocksize_trampoline::<F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    #[doc(alias = "enable-last-sample")]
    pub fn connect_enable_last_sample_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        unsafe extern "C" fn notify_enable_last_sample_trampoline<
            F: Fn(&AppSink) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enable-last-sample\0".as_ptr() as *const _,
                Some(mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_enable_last_sample_trampoline::<F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    #[doc(alias = "last-sample")]
    pub fn connect_last_sample_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        unsafe extern "C" fn notify_last_sample_trampoline<
            F: Fn(&AppSink) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"notify::last-sample\0".as_ptr() as *const _,
                Some(mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_last_sample_trampoline::<F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-bitrate")]
    pub fn connect_max_bitrate_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        unsafe extern "C" fn notify_max_bitrate_trampoline<
            F: Fn(&AppSink) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-bitrate\0".as_ptr() as *const _,
                Some(mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_max_bitrate_trampoline::<F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-lateness")]
    pub fn connect_max_lateness_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        unsafe extern "C" fn notify_max_lateness_trampoline<
            F: Fn(&AppSink) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-lateness\0".as_ptr() as *const _,
                Some(mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_max_lateness_trampoline::<F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "processing-deadline")]
    pub fn connect_processing_deadline_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        unsafe extern "C" fn notify_processing_deadline_trampoline<
            F: Fn(&AppSink) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"notify::processing-deadline\0".as_ptr() as *const _,
                Some(mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_processing_deadline_trampoline::<F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    #[doc(alias = "qos")]
    pub fn connect_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        unsafe extern "C" fn notify_qos_trampoline<F: Fn(&AppSink) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"notify::qos\0".as_ptr() as *const _,
                Some(mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_qos_trampoline::<F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    #[doc(alias = "render-delay")]
    pub fn connect_render_delay_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        unsafe extern "C" fn notify_render_delay_trampoline<
            F: Fn(&AppSink) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"notify::render-delay\0".as_ptr() as *const _,
                Some(mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_render_delay_trampoline::<F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "stats")]
    pub fn connect_stats_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        unsafe extern "C" fn notify_stats_trampoline<F: Fn(&AppSink) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"notify::stats\0".as_ptr() as *const _,
                Some(mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_stats_trampoline::<F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    #[doc(alias = "sync")]
    pub fn connect_sync_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        unsafe extern "C" fn notify_sync_trampoline<F: Fn(&AppSink) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"notify::sync\0".as_ptr() as *const _,
                Some(mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_sync_trampoline::<F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    #[doc(alias = "throttle-time")]
    pub fn connect_throttle_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        unsafe extern "C" fn notify_throttle_time_trampoline<
            F: Fn(&AppSink) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"notify::throttle-time\0".as_ptr() as *const _,
                Some(mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_throttle_time_trampoline::<F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    #[doc(alias = "ts-offset")]
    pub fn connect_ts_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        unsafe extern "C" fn notify_ts_offset_trampoline<
            F: Fn(&AppSink) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSink,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ts-offset\0".as_ptr() as *const _,
                Some(mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_ts_offset_trampoline::<F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    pub fn stream(&self) -> AppSinkStream {
        AppSinkStream::new(self)
    }
}

#[derive(Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`AppSink`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct AppSinkBuilder {
    async_: Option<bool>,
    buffer_list: Option<bool>,
    callbacks: Option<AppSinkCallbacks>,
    caps: Option<gst::Caps>,
    drop: Option<bool>,
    drop_out_of_segment: Option<bool>,
    enable_last_sample: Option<bool>,
    max_bitrate: Option<u64>,
    max_buffers: Option<u32>,
    max_lateness: Option<i64>,
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    processing_deadline: Option<i64>,
    qos: Option<bool>,
    render_delay: Option<Option<gst::ClockTime>>,
    sync: Option<bool>,
    throttle_time: Option<u64>,
    ts_offset: Option<gst::ClockTimeDiff>,
    wait_on_eos: Option<bool>,
    name: Option<String>,
}

impl AppSinkBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`AppSinkBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`AppSink`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> AppSink {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref async_) = self.async_ {
            properties.push(("async", async_));
        }
        if let Some(ref buffer_list) = self.buffer_list {
            properties.push(("buffer-list", buffer_list));
        }
        if let Some(ref caps) = self.caps {
            properties.push(("caps", caps));
        }
        if let Some(ref drop) = self.drop {
            properties.push(("drop", drop));
        }
        if let Some(ref enable_last_sample) = self.enable_last_sample {
            properties.push(("enable-last-sample", enable_last_sample));
        }
        if let Some(ref max_bitrate) = self.max_bitrate {
            properties.push(("max-bitrate", max_bitrate));
        }
        if let Some(ref max_buffers) = self.max_buffers {
            properties.push(("max-buffers", max_buffers));
        }
        if let Some(ref max_lateness) = self.max_lateness {
            properties.push(("max-lateness", max_lateness));
        }
        #[cfg(any(feature = "v1_16", feature = "dox"))]
        if let Some(ref processing_deadline) = self.processing_deadline {
            properties.push(("processing-deadline", processing_deadline));
        }
        if let Some(ref qos) = self.qos {
            properties.push(("qos", qos));
        }
        if let Some(ref render_delay) = self.render_delay {
            properties.push(("render-delay", render_delay));
        }
        if let Some(ref sync) = self.sync {
            properties.push(("sync", sync));
        }
        if let Some(ref throttle_time) = self.throttle_time {
            properties.push(("throttle-time", throttle_time));
        }
        if let Some(ref ts_offset) = self.ts_offset {
            properties.push(("ts-offset", ts_offset));
        }
        if let Some(ref qos) = self.qos {
            properties.push(("qos", qos));
        }
        if let Some(ref wait_on_eos) = self.wait_on_eos {
            properties.push(("wait-on-eos", wait_on_eos));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }

        let appsink = glib::Object::new::<AppSink>(&properties);

        if let Some(callbacks) = self.callbacks {
            appsink.set_callbacks(callbacks);
        }

        if let Some(drop_out_of_segment) = self.drop_out_of_segment {
            appsink.set_drop_out_of_segment(drop_out_of_segment);
        }

        appsink
    }

    pub fn async_(mut self, async_: bool) -> Self {
        self.async_ = Some(async_);
        self
    }

    pub fn buffer_list(mut self, buffer_list: bool) -> Self {
        self.buffer_list = Some(buffer_list);
        self
    }

    pub fn callbacks(mut self, callbacks: AppSinkCallbacks) -> Self {
        self.callbacks = Some(callbacks);
        self
    }

    pub fn caps(mut self, caps: &gst::Caps) -> Self {
        self.caps = Some(caps.clone());
        self
    }

    pub fn drop(mut self, drop: bool) -> Self {
        self.drop = Some(drop);
        self
    }

    pub fn drop_out_of_segment(mut self, drop_out_of_segment: bool) -> Self {
        self.drop_out_of_segment = Some(drop_out_of_segment);
        self
    }

    pub fn enable_last_sample(mut self, enable_last_sample: bool) -> Self {
        self.enable_last_sample = Some(enable_last_sample);
        self
    }

    pub fn max_bitrate(mut self, max_bitrate: u64) -> Self {
        self.max_bitrate = Some(max_bitrate);
        self
    }

    pub fn max_buffers(mut self, max_buffers: u32) -> Self {
        self.max_buffers = Some(max_buffers);
        self
    }

    pub fn max_lateness(mut self, max_lateness: i64) -> Self {
        self.max_lateness = Some(max_lateness);
        self
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn processing_deadline(mut self, processing_deadline: i64) -> Self {
        self.processing_deadline = Some(processing_deadline);
        self
    }

    pub fn qos(mut self, qos: bool) -> Self {
        self.qos = Some(qos);
        self
    }

    pub fn render_delay(mut self, render_delay: Option<gst::ClockTime>) -> Self {
        self.render_delay = Some(render_delay);
        self
    }

    pub fn sync(mut self, sync: bool) -> Self {
        self.sync = Some(sync);
        self
    }

    pub fn throttle_time(mut self, throttle_time: u64) -> Self {
        self.throttle_time = Some(throttle_time);
        self
    }

    pub fn ts_offset(mut self, ts_offset: gst::ClockTimeDiff) -> Self {
        self.ts_offset = Some(ts_offset);
        self
    }

    pub fn wait_on_eos(mut self, wait_on_eos: bool) -> Self {
        self.wait_on_eos = Some(wait_on_eos);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
}

#[derive(Debug)]
pub struct AppSinkStream {
    app_sink: glib::WeakRef<AppSink>,
    waker_reference: Arc<Mutex<Option<Waker>>>,
}

impl AppSinkStream {
    fn new(app_sink: &AppSink) -> Self {
        skip_assert_initialized!();

        let waker_reference = Arc::new(Mutex::new(None as Option<Waker>));

        app_sink.set_callbacks(
            AppSinkCallbacks::builder()
                .new_sample({
                    let waker_reference = Arc::clone(&waker_reference);

                    move |_| {
                        if let Some(waker) = waker_reference.lock().unwrap().take() {
                            waker.wake();
                        }

                        Ok(gst::FlowSuccess::Ok)
                    }
                })
                .eos({
                    let waker_reference = Arc::clone(&waker_reference);

                    move |_| {
                        if let Some(waker) = waker_reference.lock().unwrap().take() {
                            waker.wake();
                        }
                    }
                })
                .build(),
        );

        Self {
            app_sink: app_sink.downgrade(),
            waker_reference,
        }
    }
}

impl Drop for AppSinkStream {
    fn drop(&mut self) {
        // This is not thread-safe before 1.16.3, see
        // https://gitlab.freedesktop.org/gstreamer/gst-plugins-base/merge_requests/570
        if gst::version() >= (1, 16, 3, 0) {
            if let Some(app_sink) = self.app_sink.upgrade() {
                app_sink.set_callbacks(AppSinkCallbacks::builder().build());
            }
        }
    }
}

impl Stream for AppSinkStream {
    type Item = gst::Sample;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context) -> Poll<Option<Self::Item>> {
        let mut waker = self.waker_reference.lock().unwrap();

        let app_sink = match self.app_sink.upgrade() {
            Some(app_sink) => app_sink,
            None => return Poll::Ready(None),
        };

        app_sink
            .try_pull_sample(gst::ClockTime::ZERO)
            .map(|sample| Poll::Ready(Some(sample)))
            .unwrap_or_else(|| {
                if app_sink.is_eos() {
                    return Poll::Ready(None);
                }

                waker.replace(context.waker().to_owned());

                Poll::Pending
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;
    use gst::prelude::*;

    #[test]
    fn test_app_sink_stream() {
        gst::init().unwrap();

        let videotestsrc = gst::ElementFactory::make("videotestsrc")
            .property("num-buffers", 5)
            .build()
            .unwrap();
        let appsink = gst::ElementFactory::make("appsink").build().unwrap();

        let pipeline = gst::Pipeline::new(None);
        pipeline.add(&videotestsrc).unwrap();
        pipeline.add(&appsink).unwrap();

        videotestsrc.link(&appsink).unwrap();

        let app_sink_stream = appsink.dynamic_cast::<AppSink>().unwrap().stream();
        let samples_future = app_sink_stream.collect::<Vec<gst::Sample>>();

        pipeline.set_state(gst::State::Playing).unwrap();
        let samples = futures_executor::block_on(samples_future);
        pipeline.set_state(gst::State::Null).unwrap();

        assert_eq!(samples.len(), 5);
    }
}
