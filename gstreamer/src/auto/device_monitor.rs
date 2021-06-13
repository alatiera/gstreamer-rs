// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Bus;
use crate::Device;
use crate::Object;
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
    #[doc(alias = "GstDeviceMonitor")]
    pub struct DeviceMonitor(Object<ffi::GstDeviceMonitor, ffi::GstDeviceMonitorClass>) @extends Object;

    match fn {
        type_ => || ffi::gst_device_monitor_get_type(),
    }
}

unsafe impl Send for DeviceMonitor {}
unsafe impl Sync for DeviceMonitor {}

pub const NONE_DEVICE_MONITOR: Option<&DeviceMonitor> = None;

pub trait DeviceMonitorExt: 'static {
    #[doc(alias = "gst_device_monitor_get_bus")]
    #[doc(alias = "get_bus")]
    fn bus(&self) -> Bus;

    #[doc(alias = "gst_device_monitor_get_devices")]
    #[doc(alias = "get_devices")]
    fn devices(&self) -> Vec<Device>;

    #[doc(alias = "gst_device_monitor_get_providers")]
    #[doc(alias = "get_providers")]
    fn providers(&self) -> Vec<glib::GString>;

    #[doc(alias = "gst_device_monitor_get_show_all_devices")]
    #[doc(alias = "get_show_all_devices")]
    fn shows_all_devices(&self) -> bool;

    #[doc(alias = "gst_device_monitor_set_show_all_devices")]
    fn set_show_all_devices(&self, show_all: bool);

    #[doc(alias = "gst_device_monitor_start")]
    fn start(&self) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_device_monitor_stop")]
    fn stop(&self);

    #[doc(alias = "show-all")]
    fn shows_all(&self) -> bool;

    #[doc(alias = "show-all")]
    fn set_show_all(&self, show_all: bool);

    #[doc(alias = "show-all")]
    fn connect_show_all_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<DeviceMonitor>> DeviceMonitorExt for O {
    fn bus(&self) -> Bus {
        unsafe {
            from_glib_full(ffi::gst_device_monitor_get_bus(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn devices(&self) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_device_monitor_get_devices(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn providers(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_device_monitor_get_providers(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn shows_all_devices(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_device_monitor_get_show_all_devices(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_show_all_devices(&self, show_all: bool) {
        unsafe {
            ffi::gst_device_monitor_set_show_all_devices(
                self.as_ref().to_glib_none().0,
                show_all.into_glib(),
            );
        }
    }

    fn start(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_device_monitor_start(self.as_ref().to_glib_none().0),
                "Failed to start"
            )
        }
    }

    fn stop(&self) {
        unsafe {
            ffi::gst_device_monitor_stop(self.as_ref().to_glib_none().0);
        }
    }

    fn shows_all(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"show-all\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `show-all` getter")
        }
    }

    fn set_show_all(&self, show_all: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"show-all\0".as_ptr() as *const _,
                show_all.to_value().to_glib_none().0,
            );
        }
    }

    fn connect_show_all_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_all_trampoline<
            P: IsA<DeviceMonitor>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstDeviceMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&DeviceMonitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-all\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_all_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
