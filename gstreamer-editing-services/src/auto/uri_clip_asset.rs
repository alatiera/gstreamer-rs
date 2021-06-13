// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Asset;
use crate::UriSourceAsset;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GESUriClipAsset")]
    pub struct UriClipAsset(Object<ffi::GESUriClipAsset, ffi::GESUriClipAssetClass>) @extends Asset;

    match fn {
        type_ => || ffi::ges_uri_clip_asset_get_type(),
    }
}

impl UriClipAsset {
    //#[cfg(any(feature = "v1_16", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    //#[doc(alias = "ges_uri_clip_asset_finish")]
    //pub fn finish(res: /*Ignored*/&gio::AsyncResult) -> Result<UriClipAsset, glib::Error> {
    //    unsafe { TODO: call ffi:ges_uri_clip_asset_finish() }
    //}

    //#[doc(alias = "ges_uri_clip_asset_new")]
    //pub fn new<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(uri: &str, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ffi:ges_uri_clip_asset_new() }
    //}

    #[doc(alias = "ges_uri_clip_asset_request_sync")]
    pub fn request_sync(uri: &str) -> Result<UriClipAsset, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ges_uri_clip_asset_request_sync(uri.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

pub const NONE_URI_CLIP_ASSET: Option<&UriClipAsset> = None;

pub trait UriClipAssetExt: 'static {
    #[doc(alias = "ges_uri_clip_asset_get_duration")]
    #[doc(alias = "get_duration")]
    fn duration(&self) -> Option<gst::ClockTime>;

    #[doc(alias = "ges_uri_clip_asset_get_info")]
    #[doc(alias = "get_info")]
    fn info(&self) -> Option<gst_pbutils::DiscovererInfo>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_uri_clip_asset_get_max_duration")]
    #[doc(alias = "get_max_duration")]
    fn max_duration(&self) -> Option<gst::ClockTime>;

    #[doc(alias = "ges_uri_clip_asset_get_stream_assets")]
    #[doc(alias = "get_stream_assets")]
    fn stream_assets(&self) -> Vec<UriSourceAsset>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_uri_clip_asset_is_image")]
    fn is_image(&self) -> bool;

    fn set_duration(&self, duration: u64);

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "is-nested-timeline")]
    fn is_nested_timeline(&self) -> bool;

    #[doc(alias = "duration")]
    fn connect_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "is-nested-timeline")]
    fn connect_is_nested_timeline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<UriClipAsset>> UriClipAssetExt for O {
    fn duration(&self) -> Option<gst::ClockTime> {
        unsafe {
            from_glib(ffi::ges_uri_clip_asset_get_duration(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn info(&self) -> Option<gst_pbutils::DiscovererInfo> {
        unsafe {
            from_glib_none(ffi::ges_uri_clip_asset_get_info(const_override(
                self.as_ref().to_glib_none().0,
            )))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn max_duration(&self) -> Option<gst::ClockTime> {
        unsafe {
            from_glib(ffi::ges_uri_clip_asset_get_max_duration(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn stream_assets(&self) -> Vec<UriSourceAsset> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::ges_uri_clip_asset_get_stream_assets(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn is_image(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_uri_clip_asset_is_image(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_duration(&self, duration: u64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"duration\0".as_ptr() as *const _,
                duration.to_value().to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn is_nested_timeline(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"is-nested-timeline\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `is-nested-timeline` getter")
        }
    }

    fn connect_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_duration_trampoline<
            P: IsA<UriClipAsset>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESUriClipAsset,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&UriClipAsset::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_duration_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_is_nested_timeline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_nested_timeline_trampoline<
            P: IsA<UriClipAsset>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESUriClipAsset,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&UriClipAsset::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-nested-timeline\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_nested_timeline_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
