// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct NetTimeProvider(Object<ffi::GstNetTimeProvider, ffi::GstNetTimeProviderClass>) @extends gst::Object;

    match fn {
        get_type => || ffi::gst_net_time_provider_get_type(),
    }
}

impl NetTimeProvider {
    pub fn get_property_active(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"active\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `active` getter")
                .unwrap()
        }
    }

    pub fn set_property_active(&self, active: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"active\0".as_ptr() as *const _,
                glib::Value::from(&active).to_glib_none().0,
            );
        }
    }

    pub fn get_property_address(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"address\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `address` getter")
        }
    }

    pub fn get_property_clock(&self) -> Option<gst::Clock> {
        unsafe {
            let mut value = glib::Value::from_type(<gst::Clock as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"clock\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `clock` getter")
        }
    }

    pub fn get_property_port(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"port\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `port` getter")
                .unwrap()
        }
    }

    pub fn get_property_qos_dscp(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"qos-dscp\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `qos-dscp` getter")
                .unwrap()
        }
    }

    pub fn set_property_qos_dscp(&self, qos_dscp: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"qos-dscp\0".as_ptr() as *const _,
                glib::Value::from(&qos_dscp).to_glib_none().0,
            );
        }
    }

    pub fn connect_property_active_notify<F: Fn(&NetTimeProvider) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<
            F: Fn(&NetTimeProvider) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstNetTimeProvider,
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
                b"notify::active\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_qos_dscp_notify<F: Fn(&NetTimeProvider) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_qos_dscp_trampoline<
            F: Fn(&NetTimeProvider) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstNetTimeProvider,
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
                b"notify::qos-dscp\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_qos_dscp_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for NetTimeProvider {}
unsafe impl Sync for NetTimeProvider {}
