// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::GLContext;
use glib::object::IsA;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use glib::StaticType;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use std::boxed::Box as Box_;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct GLOverlayCompositor(Object<ffi::GstGLOverlayCompositor, ffi::GstGLOverlayCompositorClass>) @extends gst::Object;

    match fn {
        get_type => || ffi::gst_gl_overlay_compositor_get_type(),
    }
}

impl GLOverlayCompositor {
    pub fn new<P: IsA<GLContext>>(context: &P) -> GLOverlayCompositor {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gst_gl_overlay_compositor_new(
                context.as_ref().to_glib_none().0,
            ))
        }
    }

    pub fn draw_overlays(&self) {
        unsafe {
            ffi::gst_gl_overlay_compositor_draw_overlays(self.to_glib_none().0);
        }
    }

    pub fn free_overlays(&self) {
        unsafe {
            ffi::gst_gl_overlay_compositor_free_overlays(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn get_property_yinvert(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"yinvert\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `yinvert` getter")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn set_property_yinvert(&self, yinvert: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"yinvert\0".as_ptr() as *const _,
                glib::Value::from(&yinvert).to_glib_none().0,
            );
        }
    }

    pub fn add_caps(caps: &gst::Caps) -> Option<gst::Caps> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_gl_overlay_compositor_add_caps(
                caps.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn connect_property_yinvert_notify<F: Fn(&GLOverlayCompositor) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_yinvert_trampoline<
            F: Fn(&GLOverlayCompositor) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLOverlayCompositor,
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
                b"notify::yinvert\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_yinvert_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for GLOverlayCompositor {}
unsafe impl Sync for GLOverlayCompositor {}
