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
    pub struct DirectControlBinding(Object<ffi::GstDirectControlBinding, ffi::GstDirectControlBindingClass>) @extends gst::ControlBinding, gst::Object;

    match fn {
        type_ => || ffi::gst_direct_control_binding_get_type(),
    }
}

impl DirectControlBinding {
    #[doc(alias = "gst_direct_control_binding_new")]
    pub fn new<P: IsA<gst::Object>, Q: IsA<gst::ControlSource>>(
        object: &P,
        property_name: &str,
        cs: &Q,
    ) -> DirectControlBinding {
        assert_initialized_main_thread!();
        unsafe {
            gst::ControlBinding::from_glib_none(ffi::gst_direct_control_binding_new(
                object.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                cs.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gst_direct_control_binding_new_absolute")]
    pub fn new_absolute<P: IsA<gst::Object>, Q: IsA<gst::ControlSource>>(
        object: &P,
        property_name: &str,
        cs: &Q,
    ) -> DirectControlBinding {
        assert_initialized_main_thread!();
        unsafe {
            gst::ControlBinding::from_glib_none(ffi::gst_direct_control_binding_new_absolute(
                object.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                cs.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

unsafe impl Send for DirectControlBinding {}
unsafe impl Sync for DirectControlBinding {}

pub const NONE_DIRECT_CONTROL_BINDING: Option<&DirectControlBinding> = None;

pub trait DirectControlBindingExt: 'static {
    fn is_absolute(&self) -> bool;

    #[doc(alias = "control-source")]
    fn control_source(&self) -> Option<gst::ControlSource>;

    #[doc(alias = "control-source")]
    fn set_control_source<P: IsA<gst::ControlSource>>(&self, control_source: Option<&P>);

    #[doc(alias = "control-source")]
    fn connect_control_source_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<DirectControlBinding>> DirectControlBindingExt for O {
    fn is_absolute(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"absolute\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `absolute` getter")
        }
    }

    fn control_source(&self) -> Option<gst::ControlSource> {
        unsafe {
            let mut value =
                glib::Value::from_type(<gst::ControlSource as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"control-source\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `control-source` getter")
        }
    }

    fn set_control_source<P: IsA<gst::ControlSource>>(&self, control_source: Option<&P>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"control-source\0".as_ptr() as *const _,
                control_source.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "control-source")]
    fn connect_control_source_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_control_source_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstDirectControlBinding,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DirectControlBinding>,
        {
            let f: &F = &*(f as *const F);
            f(&DirectControlBinding::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::control-source\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_control_source_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
