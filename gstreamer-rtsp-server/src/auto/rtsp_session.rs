// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gst_rtsp_server_sys;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use RTSPFilterResult;
use RTSPMedia;
use RTSPSessionMedia;

glib_wrapper! {
    pub struct RTSPSession(Object<gst_rtsp_server_sys::GstRTSPSession, gst_rtsp_server_sys::GstRTSPSessionClass, RTSPSessionClass>);

    match fn {
        get_type => || gst_rtsp_server_sys::gst_rtsp_session_get_type(),
    }
}

impl RTSPSession {
    pub fn new(sessionid: &str) -> RTSPSession {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_rtsp_server_sys::gst_rtsp_session_new(
                sessionid.to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for RTSPSession {}
unsafe impl Sync for RTSPSession {}

pub const NONE_RTSP_SESSION: Option<&RTSPSession> = None;

pub trait RTSPSessionExt: 'static {
    fn allow_expire(&self);

    fn filter(
        &self,
        func: Option<&mut dyn (FnMut(&RTSPSession, &RTSPSessionMedia) -> RTSPFilterResult)>,
    ) -> Vec<RTSPSessionMedia>;

    fn get_header(&self) -> Option<GString>;

    fn get_media(&self, path: &str) -> (Option<RTSPSessionMedia>, i32);

    fn get_sessionid(&self) -> Option<GString>;

    fn get_timeout(&self) -> u32;

    //fn is_expired(&self, now: /*Ignored*/&mut glib::TimeVal) -> bool;

    fn is_expired_usec(&self, now: i64) -> bool;

    fn manage_media<P: IsA<RTSPMedia>>(
        &self,
        path: &str,
        media: &P,
    ) -> Result<RTSPSessionMedia, glib::BoolError>;

    //fn next_timeout(&self, now: /*Ignored*/&mut glib::TimeVal) -> i32;

    fn next_timeout_usec(&self, now: i64) -> i32;

    fn prevent_expire(&self);

    fn release_media<P: IsA<RTSPSessionMedia>>(&self, media: &P) -> bool;

    fn set_timeout(&self, timeout: u32);

    fn touch(&self);

    fn get_property_timeout_always_visible(&self) -> bool;

    fn set_property_timeout_always_visible(&self, timeout_always_visible: bool);

    fn connect_property_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_timeout_always_visible_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<RTSPSession>> RTSPSessionExt for O {
    fn allow_expire(&self) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_session_allow_expire(self.as_ref().to_glib_none().0);
        }
    }

    fn filter(
        &self,
        func: Option<&mut dyn (FnMut(&RTSPSession, &RTSPSessionMedia) -> RTSPFilterResult)>,
    ) -> Vec<RTSPSessionMedia> {
        let func_data: Option<
            &mut dyn (FnMut(&RTSPSession, &RTSPSessionMedia) -> RTSPFilterResult),
        > = func;
        unsafe extern "C" fn func_func(
            sess: *mut gst_rtsp_server_sys::GstRTSPSession,
            media: *mut gst_rtsp_server_sys::GstRTSPSessionMedia,
            user_data: glib_sys::gpointer,
        ) -> gst_rtsp_server_sys::GstRTSPFilterResult {
            let sess = from_glib_borrow(sess);
            let media = from_glib_borrow(media);
            let callback: *mut Option<
                &mut dyn (FnMut(&RTSPSession, &RTSPSessionMedia) -> RTSPFilterResult),
            > = user_data as *const _ as usize
                as *mut Option<
                    &mut dyn (FnMut(&RTSPSession, &RTSPSessionMedia) -> RTSPFilterResult),
                >;
            let res = if let Some(ref mut callback) = *callback {
                callback(&sess, &media)
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib()
        }
        let func = if func_data.is_some() {
            Some(func_func as _)
        } else {
            None
        };
        let super_callback0: &Option<
            &mut dyn (FnMut(&RTSPSession, &RTSPSessionMedia) -> RTSPFilterResult),
        > = &func_data;
        unsafe {
            FromGlibPtrContainer::from_glib_full(gst_rtsp_server_sys::gst_rtsp_session_filter(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            ))
        }
    }

    fn get_header(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gst_rtsp_server_sys::gst_rtsp_session_get_header(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_media(&self, path: &str) -> (Option<RTSPSessionMedia>, i32) {
        unsafe {
            let mut matched = mem::MaybeUninit::uninit();
            let ret = from_glib_none(gst_rtsp_server_sys::gst_rtsp_session_get_media(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                matched.as_mut_ptr(),
            ));
            let matched = matched.assume_init();
            (ret, matched)
        }
    }

    fn get_sessionid(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gst_rtsp_server_sys::gst_rtsp_session_get_sessionid(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_timeout(&self) -> u32 {
        unsafe { gst_rtsp_server_sys::gst_rtsp_session_get_timeout(self.as_ref().to_glib_none().0) }
    }

    //fn is_expired(&self, now: /*Ignored*/&mut glib::TimeVal) -> bool {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_session_is_expired() }
    //}

    fn is_expired_usec(&self, now: i64) -> bool {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_session_is_expired_usec(
                self.as_ref().to_glib_none().0,
                now,
            ))
        }
    }

    fn manage_media<P: IsA<RTSPMedia>>(
        &self,
        path: &str,
        media: &P,
    ) -> Result<RTSPSessionMedia, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_none(gst_rtsp_server_sys::gst_rtsp_session_manage_media(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                media.as_ref().to_glib_full(),
            ))
            .ok_or_else(|| glib_bool_error!("Failed to manage media"))
        }
    }

    //fn next_timeout(&self, now: /*Ignored*/&mut glib::TimeVal) -> i32 {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_session_next_timeout() }
    //}

    fn next_timeout_usec(&self, now: i64) -> i32 {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_session_next_timeout_usec(
                self.as_ref().to_glib_none().0,
                now,
            )
        }
    }

    fn prevent_expire(&self) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_session_prevent_expire(self.as_ref().to_glib_none().0);
        }
    }

    fn release_media<P: IsA<RTSPSessionMedia>>(&self, media: &P) -> bool {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_session_release_media(
                self.as_ref().to_glib_none().0,
                media.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_timeout(&self, timeout: u32) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_session_set_timeout(
                self.as_ref().to_glib_none().0,
                timeout,
            );
        }
    }

    fn touch(&self) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_session_touch(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_timeout_always_visible(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"timeout-always-visible\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `timeout-always-visible` getter")
                .unwrap()
        }
    }

    fn set_property_timeout_always_visible(&self, timeout_always_visible: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"timeout-always-visible\0".as_ptr() as *const _,
                Value::from(&timeout_always_visible).to_glib_none().0,
            );
        }
    }

    fn connect_property_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeout_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_rtsp_server_sys::GstRTSPSession,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RTSPSession>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPSession::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeout\0".as_ptr() as *const _,
                Some(transmute(notify_timeout_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_timeout_always_visible_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeout_always_visible_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut gst_rtsp_server_sys::GstRTSPSession,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RTSPSession>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPSession::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeout-always-visible\0".as_ptr() as *const _,
                Some(transmute(
                    notify_timeout_always_visible_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }
}
