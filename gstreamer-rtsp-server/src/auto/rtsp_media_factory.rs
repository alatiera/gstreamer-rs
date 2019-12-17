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
use gst;
use gst_rtsp;
use gst_rtsp_server_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;
use RTSPAddressPool;
use RTSPMedia;
use RTSPPublishClockMode;
use RTSPSuspendMode;
use RTSPTransportMode;

glib_wrapper! {
    pub struct RTSPMediaFactory(Object<gst_rtsp_server_sys::GstRTSPMediaFactory, gst_rtsp_server_sys::GstRTSPMediaFactoryClass, RTSPMediaFactoryClass>);

    match fn {
        get_type => || gst_rtsp_server_sys::gst_rtsp_media_factory_get_type(),
    }
}

impl RTSPMediaFactory {
    pub fn new() -> RTSPMediaFactory {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gst_rtsp_server_sys::gst_rtsp_media_factory_new()) }
    }
}

impl Default for RTSPMediaFactory {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPMediaFactory {}
unsafe impl Sync for RTSPMediaFactory {}

pub const NONE_RTSP_MEDIA_FACTORY: Option<&RTSPMediaFactory> = None;

pub trait RTSPMediaFactoryExt: 'static {
    //fn add_role(&self, role: &str, fieldname: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn construct(&self, url: &gst_rtsp::RTSPUrl) -> Result<RTSPMedia, glib::BoolError>;

    fn create_element(&self, url: &gst_rtsp::RTSPUrl) -> Result<gst::Element, glib::BoolError>;

    fn get_address_pool(&self) -> Option<RTSPAddressPool>;

    fn get_buffer_size(&self) -> u32;

    fn get_clock(&self) -> Option<gst::Clock>;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn get_do_retransmission(&self) -> bool;

    fn get_latency(&self) -> u32;

    fn get_launch(&self) -> Option<GString>;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn get_max_mcast_ttl(&self) -> u32;

    fn get_media_gtype(&self) -> glib::types::Type;

    fn get_multicast_iface(&self) -> Option<GString>;

    //fn get_permissions(&self) -> /*Ignored*/Option<RTSPPermissions>;

    fn get_profiles(&self) -> gst_rtsp::RTSPProfile;

    fn get_protocols(&self) -> gst_rtsp::RTSPLowerTrans;

    fn get_publish_clock_mode(&self) -> RTSPPublishClockMode;

    fn get_retransmission_time(&self) -> gst::ClockTime;

    fn get_suspend_mode(&self) -> RTSPSuspendMode;

    fn get_transport_mode(&self) -> RTSPTransportMode;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn is_bind_mcast_address(&self) -> bool;

    fn is_eos_shutdown(&self) -> bool;

    fn is_shared(&self) -> bool;

    fn is_stop_on_disonnect(&self) -> bool;

    fn set_address_pool<P: IsA<RTSPAddressPool>>(&self, pool: Option<&P>);

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_bind_mcast_address(&self, bind_mcast_addr: bool);

    fn set_buffer_size(&self, size: u32);

    fn set_clock<P: IsA<gst::Clock>>(&self, clock: Option<&P>);

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_do_retransmission(&self, do_retransmission: bool);

    fn set_eos_shutdown(&self, eos_shutdown: bool);

    fn set_latency(&self, latency: u32);

    fn set_launch(&self, launch: &str);

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_max_mcast_ttl(&self, ttl: u32) -> bool;

    fn set_media_gtype(&self, media_gtype: glib::types::Type);

    fn set_multicast_iface(&self, multicast_iface: Option<&str>);

    //fn set_permissions(&self, permissions: /*Ignored*/Option<&mut RTSPPermissions>);

    fn set_profiles(&self, profiles: gst_rtsp::RTSPProfile);

    fn set_protocols(&self, protocols: gst_rtsp::RTSPLowerTrans);

    fn set_publish_clock_mode(&self, mode: RTSPPublishClockMode);

    fn set_retransmission_time(&self, time: gst::ClockTime);

    fn set_shared(&self, shared: bool);

    fn set_stop_on_disconnect(&self, stop_on_disconnect: bool);

    fn set_suspend_mode(&self, mode: RTSPSuspendMode);

    fn set_transport_mode(&self, mode: RTSPTransportMode);

    fn get_property_bind_mcast_address(&self) -> bool;

    fn set_property_bind_mcast_address(&self, bind_mcast_address: bool);

    fn get_property_eos_shutdown(&self) -> bool;

    fn get_property_max_mcast_ttl(&self) -> u32;

    fn set_property_max_mcast_ttl(&self, max_mcast_ttl: u32);

    fn get_property_shared(&self) -> bool;

    fn get_property_stop_on_disconnect(&self) -> bool;

    fn connect_media_configure<F: Fn(&Self, &RTSPMedia) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_media_constructed<F: Fn(&Self, &RTSPMedia) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_bind_mcast_address_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_buffer_size_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_clock_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_eos_shutdown_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_launch_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_max_mcast_ttl_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_profiles_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_protocols_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_shared_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_stop_on_disconnect_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_suspend_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_transport_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<RTSPMediaFactory>> RTSPMediaFactoryExt for O {
    //fn add_role(&self, role: &str, fieldname: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_media_factory_add_role() }
    //}

    fn construct(&self, url: &gst_rtsp::RTSPUrl) -> Result<RTSPMedia, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(gst_rtsp_server_sys::gst_rtsp_media_factory_construct(
                self.as_ref().to_glib_none().0,
                url.to_glib_none().0,
            ))
            .ok_or_else(|| glib_bool_error!("Failed to construct media"))
        }
    }

    fn create_element(&self, url: &gst_rtsp::RTSPUrl) -> Result<gst::Element, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_none(gst_rtsp_server_sys::gst_rtsp_media_factory_create_element(
                self.as_ref().to_glib_none().0,
                url.to_glib_none().0,
            ))
            .ok_or_else(|| glib_bool_error!("Failed to create media element"))
        }
    }

    fn get_address_pool(&self) -> Option<RTSPAddressPool> {
        unsafe {
            from_glib_full(
                gst_rtsp_server_sys::gst_rtsp_media_factory_get_address_pool(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_buffer_size(&self) -> u32 {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_get_buffer_size(
                self.as_ref().to_glib_none().0,
            )
        }
    }

    fn get_clock(&self) -> Option<gst::Clock> {
        unsafe {
            from_glib_full(gst_rtsp_server_sys::gst_rtsp_media_factory_get_clock(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn get_do_retransmission(&self) -> bool {
        unsafe {
            from_glib(
                gst_rtsp_server_sys::gst_rtsp_media_factory_get_do_retransmission(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_latency(&self) -> u32 {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_get_latency(self.as_ref().to_glib_none().0)
        }
    }

    fn get_launch(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gst_rtsp_server_sys::gst_rtsp_media_factory_get_launch(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn get_max_mcast_ttl(&self) -> u32 {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_get_max_mcast_ttl(
                self.as_ref().to_glib_none().0,
            )
        }
    }

    fn get_media_gtype(&self) -> glib::types::Type {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_factory_get_media_gtype(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_multicast_iface(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                gst_rtsp_server_sys::gst_rtsp_media_factory_get_multicast_iface(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    //fn get_permissions(&self) -> /*Ignored*/Option<RTSPPermissions> {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_media_factory_get_permissions() }
    //}

    fn get_profiles(&self) -> gst_rtsp::RTSPProfile {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_factory_get_profiles(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_protocols(&self) -> gst_rtsp::RTSPLowerTrans {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_factory_get_protocols(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_publish_clock_mode(&self) -> RTSPPublishClockMode {
        unsafe {
            from_glib(
                gst_rtsp_server_sys::gst_rtsp_media_factory_get_publish_clock_mode(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_retransmission_time(&self) -> gst::ClockTime {
        unsafe {
            from_glib(
                gst_rtsp_server_sys::gst_rtsp_media_factory_get_retransmission_time(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_suspend_mode(&self) -> RTSPSuspendMode {
        unsafe {
            from_glib(
                gst_rtsp_server_sys::gst_rtsp_media_factory_get_suspend_mode(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_transport_mode(&self) -> RTSPTransportMode {
        unsafe {
            from_glib(
                gst_rtsp_server_sys::gst_rtsp_media_factory_get_transport_mode(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn is_bind_mcast_address(&self) -> bool {
        unsafe {
            from_glib(
                gst_rtsp_server_sys::gst_rtsp_media_factory_is_bind_mcast_address(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn is_eos_shutdown(&self) -> bool {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_factory_is_eos_shutdown(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_shared(&self) -> bool {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_media_factory_is_shared(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_stop_on_disonnect(&self) -> bool {
        unsafe {
            from_glib(
                gst_rtsp_server_sys::gst_rtsp_media_factory_is_stop_on_disonnect(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn set_address_pool<P: IsA<RTSPAddressPool>>(&self, pool: Option<&P>) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_set_address_pool(
                self.as_ref().to_glib_none().0,
                pool.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_bind_mcast_address(&self, bind_mcast_addr: bool) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_set_bind_mcast_address(
                self.as_ref().to_glib_none().0,
                bind_mcast_addr.to_glib(),
            );
        }
    }

    fn set_buffer_size(&self, size: u32) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_set_buffer_size(
                self.as_ref().to_glib_none().0,
                size,
            );
        }
    }

    fn set_clock<P: IsA<gst::Clock>>(&self, clock: Option<&P>) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_set_clock(
                self.as_ref().to_glib_none().0,
                clock.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_do_retransmission(&self, do_retransmission: bool) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_set_do_retransmission(
                self.as_ref().to_glib_none().0,
                do_retransmission.to_glib(),
            );
        }
    }

    fn set_eos_shutdown(&self, eos_shutdown: bool) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_set_eos_shutdown(
                self.as_ref().to_glib_none().0,
                eos_shutdown.to_glib(),
            );
        }
    }

    fn set_latency(&self, latency: u32) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_set_latency(
                self.as_ref().to_glib_none().0,
                latency,
            );
        }
    }

    fn set_launch(&self, launch: &str) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_set_launch(
                self.as_ref().to_glib_none().0,
                launch.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_max_mcast_ttl(&self, ttl: u32) -> bool {
        unsafe {
            from_glib(
                gst_rtsp_server_sys::gst_rtsp_media_factory_set_max_mcast_ttl(
                    self.as_ref().to_glib_none().0,
                    ttl,
                ),
            )
        }
    }

    fn set_media_gtype(&self, media_gtype: glib::types::Type) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_set_media_gtype(
                self.as_ref().to_glib_none().0,
                media_gtype.to_glib(),
            );
        }
    }

    fn set_multicast_iface(&self, multicast_iface: Option<&str>) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_set_multicast_iface(
                self.as_ref().to_glib_none().0,
                multicast_iface.to_glib_none().0,
            );
        }
    }

    //fn set_permissions(&self, permissions: /*Ignored*/Option<&mut RTSPPermissions>) {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_media_factory_set_permissions() }
    //}

    fn set_profiles(&self, profiles: gst_rtsp::RTSPProfile) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_set_profiles(
                self.as_ref().to_glib_none().0,
                profiles.to_glib(),
            );
        }
    }

    fn set_protocols(&self, protocols: gst_rtsp::RTSPLowerTrans) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_set_protocols(
                self.as_ref().to_glib_none().0,
                protocols.to_glib(),
            );
        }
    }

    fn set_publish_clock_mode(&self, mode: RTSPPublishClockMode) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_set_publish_clock_mode(
                self.as_ref().to_glib_none().0,
                mode.to_glib(),
            );
        }
    }

    fn set_retransmission_time(&self, time: gst::ClockTime) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_set_retransmission_time(
                self.as_ref().to_glib_none().0,
                time.to_glib(),
            );
        }
    }

    fn set_shared(&self, shared: bool) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_set_shared(
                self.as_ref().to_glib_none().0,
                shared.to_glib(),
            );
        }
    }

    fn set_stop_on_disconnect(&self, stop_on_disconnect: bool) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_set_stop_on_disconnect(
                self.as_ref().to_glib_none().0,
                stop_on_disconnect.to_glib(),
            );
        }
    }

    fn set_suspend_mode(&self, mode: RTSPSuspendMode) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_set_suspend_mode(
                self.as_ref().to_glib_none().0,
                mode.to_glib(),
            );
        }
    }

    fn set_transport_mode(&self, mode: RTSPTransportMode) {
        unsafe {
            gst_rtsp_server_sys::gst_rtsp_media_factory_set_transport_mode(
                self.as_ref().to_glib_none().0,
                mode.to_glib(),
            );
        }
    }

    fn get_property_bind_mcast_address(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"bind-mcast-address\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `bind-mcast-address` getter")
                .unwrap()
        }
    }

    fn set_property_bind_mcast_address(&self, bind_mcast_address: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"bind-mcast-address\0".as_ptr() as *const _,
                Value::from(&bind_mcast_address).to_glib_none().0,
            );
        }
    }

    fn get_property_eos_shutdown(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"eos-shutdown\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `eos-shutdown` getter")
                .unwrap()
        }
    }

    fn get_property_max_mcast_ttl(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"max-mcast-ttl\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `max-mcast-ttl` getter")
                .unwrap()
        }
    }

    fn set_property_max_mcast_ttl(&self, max_mcast_ttl: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"max-mcast-ttl\0".as_ptr() as *const _,
                Value::from(&max_mcast_ttl).to_glib_none().0,
            );
        }
    }

    fn get_property_shared(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"shared\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `shared` getter")
                .unwrap()
        }
    }

    fn get_property_stop_on_disconnect(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"stop-on-disconnect\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `stop-on-disconnect` getter")
                .unwrap()
        }
    }

    fn connect_media_configure<F: Fn(&Self, &RTSPMedia) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn media_configure_trampoline<
            P,
            F: Fn(&P, &RTSPMedia) + Send + Sync + 'static,
        >(
            this: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
            object: *mut gst_rtsp_server_sys::GstRTSPMedia,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RTSPMediaFactory>,
        {
            let f: &F = &*(f as *const F);
            f(
                &RTSPMediaFactory::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"media-configure\0".as_ptr() as *const _,
                Some(transmute(media_configure_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_media_constructed<F: Fn(&Self, &RTSPMedia) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn media_constructed_trampoline<
            P,
            F: Fn(&P, &RTSPMedia) + Send + Sync + 'static,
        >(
            this: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
            object: *mut gst_rtsp_server_sys::GstRTSPMedia,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RTSPMediaFactory>,
        {
            let f: &F = &*(f as *const F);
            f(
                &RTSPMediaFactory::from_glib_borrow(this).unsafe_cast(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"media-constructed\0".as_ptr() as *const _,
                Some(transmute(media_constructed_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_bind_mcast_address_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_bind_mcast_address_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RTSPMediaFactory>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMediaFactory::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::bind-mcast-address\0".as_ptr() as *const _,
                Some(transmute(
                    notify_bind_mcast_address_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_buffer_size_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffer_size_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RTSPMediaFactory>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMediaFactory::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::buffer-size\0".as_ptr() as *const _,
                Some(transmute(notify_buffer_size_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_clock_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_clock_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RTSPMediaFactory>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMediaFactory::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::clock\0".as_ptr() as *const _,
                Some(transmute(notify_clock_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_eos_shutdown_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_eos_shutdown_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RTSPMediaFactory>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMediaFactory::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::eos-shutdown\0".as_ptr() as *const _,
                Some(transmute(
                    notify_eos_shutdown_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_latency_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RTSPMediaFactory>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMediaFactory::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::latency\0".as_ptr() as *const _,
                Some(transmute(notify_latency_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_launch_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_launch_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RTSPMediaFactory>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMediaFactory::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::launch\0".as_ptr() as *const _,
                Some(transmute(notify_launch_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_max_mcast_ttl_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_mcast_ttl_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RTSPMediaFactory>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMediaFactory::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-mcast-ttl\0".as_ptr() as *const _,
                Some(transmute(
                    notify_max_mcast_ttl_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_profiles_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_profiles_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RTSPMediaFactory>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMediaFactory::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::profiles\0".as_ptr() as *const _,
                Some(transmute(notify_profiles_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_protocols_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_protocols_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RTSPMediaFactory>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMediaFactory::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::protocols\0".as_ptr() as *const _,
                Some(transmute(notify_protocols_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_shared_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_shared_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RTSPMediaFactory>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMediaFactory::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::shared\0".as_ptr() as *const _,
                Some(transmute(notify_shared_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_stop_on_disconnect_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_stop_on_disconnect_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RTSPMediaFactory>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMediaFactory::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::stop-on-disconnect\0".as_ptr() as *const _,
                Some(transmute(
                    notify_stop_on_disconnect_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_suspend_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_suspend_mode_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RTSPMediaFactory>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMediaFactory::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::suspend-mode\0".as_ptr() as *const _,
                Some(transmute(
                    notify_suspend_mode_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_transport_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_transport_mode_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<RTSPMediaFactory>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPMediaFactory::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::transport-mode\0".as_ptr() as *const _,
                Some(transmute(
                    notify_transport_mode_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }
}
