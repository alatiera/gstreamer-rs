// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
use crate::VideoCodecFrame;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct VideoEncoder(Object<ffi::GstVideoEncoder, ffi::GstVideoEncoderClass>) @extends gst::Element, gst::Object;

    match fn {
        get_type => || ffi::gst_video_encoder_get_type(),
    }
}

unsafe impl Send for VideoEncoder {}
unsafe impl Sync for VideoEncoder {}

pub const NONE_VIDEO_ENCODER: Option<&VideoEncoder> = None;

pub trait VideoEncoderExt: 'static {
    fn allocate_output_buffer(&self, size: usize) -> Result<gst::Buffer, glib::BoolError>;

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    fn get_max_encode_time(&self, frame: &VideoCodecFrame) -> gst::ClockTimeDiff;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn get_min_force_key_unit_interval(&self) -> gst::ClockTime;

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    fn is_qos_enabled(&self) -> bool;

    fn merge_tags(&self, tags: Option<&gst::TagList>, mode: gst::TagMergeMode);

    fn proxy_getcaps(&self, caps: Option<&gst::Caps>, filter: Option<&gst::Caps>) -> gst::Caps;

    fn set_headers(&self, headers: &[&gst::Buffer]);

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_min_force_key_unit_interval(&self, interval: gst::ClockTime);

    fn set_min_pts(&self, min_pts: gst::ClockTime);

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    fn set_qos_enabled(&self, enabled: bool);

    fn get_property_qos(&self) -> bool;

    fn set_property_qos(&self, qos: bool);

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_property_min_force_key_unit_interval_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<VideoEncoder>> VideoEncoderExt for O {
    fn allocate_output_buffer(&self, size: usize) -> Result<gst::Buffer, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_video_encoder_allocate_output_buffer(
                self.as_ref().to_glib_none().0,
                size,
            ))
            .ok_or_else(|| glib::glib_bool_error!("Failed to allocate output buffer"))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    fn get_max_encode_time(&self, frame: &VideoCodecFrame) -> gst::ClockTimeDiff {
        unsafe {
            ffi::gst_video_encoder_get_max_encode_time(
                self.as_ref().to_glib_none().0,
                frame.to_glib_none().0,
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn get_min_force_key_unit_interval(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_video_encoder_get_min_force_key_unit_interval(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    fn is_qos_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_video_encoder_is_qos_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn merge_tags(&self, tags: Option<&gst::TagList>, mode: gst::TagMergeMode) {
        unsafe {
            ffi::gst_video_encoder_merge_tags(
                self.as_ref().to_glib_none().0,
                tags.to_glib_none().0,
                mode.to_glib(),
            );
        }
    }

    fn proxy_getcaps(&self, caps: Option<&gst::Caps>, filter: Option<&gst::Caps>) -> gst::Caps {
        unsafe {
            from_glib_full(ffi::gst_video_encoder_proxy_getcaps(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
                filter.to_glib_none().0,
            ))
        }
    }

    fn set_headers(&self, headers: &[&gst::Buffer]) {
        unsafe {
            ffi::gst_video_encoder_set_headers(
                self.as_ref().to_glib_none().0,
                headers.to_glib_full(),
            );
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_min_force_key_unit_interval(&self, interval: gst::ClockTime) {
        unsafe {
            ffi::gst_video_encoder_set_min_force_key_unit_interval(
                self.as_ref().to_glib_none().0,
                interval.to_glib(),
            );
        }
    }

    fn set_min_pts(&self, min_pts: gst::ClockTime) {
        unsafe {
            ffi::gst_video_encoder_set_min_pts(self.as_ref().to_glib_none().0, min_pts.to_glib());
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    fn set_qos_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gst_video_encoder_set_qos_enabled(
                self.as_ref().to_glib_none().0,
                enabled.to_glib(),
            );
        }
    }

    fn get_property_qos(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"qos\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `qos` getter")
                .unwrap()
        }
    }

    fn set_property_qos(&self, qos: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"qos\0".as_ptr() as *const _,
                glib::Value::from(&qos).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_property_min_force_key_unit_interval_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_force_key_unit_interval_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstVideoEncoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<VideoEncoder>,
        {
            let f: &F = &*(f as *const F);
            f(&VideoEncoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-force-key-unit-interval\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_force_key_unit_interval_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_qos_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstVideoEncoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<VideoEncoder>,
        {
            let f: &F = &*(f as *const F);
            f(&VideoEncoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::qos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_qos_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
