// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::WebRTCRTPReceiver;
use crate::WebRTCRTPSender;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use crate::WebRTCRTPTransceiverDirection;
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use std::boxed::Box as Box_;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use std::mem::transmute;

glib::wrapper! {
    pub struct WebRTCRTPTransceiver(Object<ffi::GstWebRTCRTPTransceiver, ffi::GstWebRTCRTPTransceiverClass>);

    match fn {
        type_ => || ffi::gst_webrtc_rtp_transceiver_get_type(),
    }
}

impl WebRTCRTPTransceiver {
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "get_property_direction")]
    pub fn direction(&self) -> WebRTCRTPTransceiverDirection {
        unsafe {
            let mut value = glib::Value::from_type(
                <WebRTCRTPTransceiverDirection as StaticType>::static_type(),
            );
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"direction\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `direction` getter")
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "set_property_direction")]
    pub fn set_direction(&self, direction: WebRTCRTPTransceiverDirection) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"direction\0".as_ptr() as *const _,
                direction.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_mlineindex")]
    pub fn mlineindex(&self) -> u32 {
        unsafe {
            let mut value = glib::Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"mlineindex\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `mlineindex` getter")
        }
    }

    #[doc(alias = "get_property_receiver")]
    pub fn receiver(&self) -> Option<WebRTCRTPReceiver> {
        unsafe {
            let mut value =
                glib::Value::from_type(<WebRTCRTPReceiver as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"receiver\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `receiver` getter")
        }
    }

    #[doc(alias = "get_property_sender")]
    pub fn sender(&self) -> Option<WebRTCRTPSender> {
        unsafe {
            let mut value = glib::Value::from_type(<WebRTCRTPSender as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"sender\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `sender` getter")
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn connect_property_direction_notify<
        F: Fn(&WebRTCRTPTransceiver) + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_direction_trampoline<
            F: Fn(&WebRTCRTPTransceiver) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCRTPTransceiver,
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
                b"notify::direction\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_direction_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for WebRTCRTPTransceiver {}
unsafe impl Sync for WebRTCRTPTransceiver {}
