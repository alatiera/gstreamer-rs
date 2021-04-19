// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;

bitflags! {
    pub struct DiscovererSerializeFlags: u32 {
        const CAPS = 1;
        const TAGS = 2;
        const MISC = 4;
    }
}

#[doc(hidden)]
impl ToGlib for DiscovererSerializeFlags {
    type GlibType = ffi::GstDiscovererSerializeFlags;

    fn to_glib(&self) -> ffi::GstDiscovererSerializeFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstDiscovererSerializeFlags> for DiscovererSerializeFlags {
    unsafe fn from_glib(value: ffi::GstDiscovererSerializeFlags) -> DiscovererSerializeFlags {
        skip_assert_initialized!();
        DiscovererSerializeFlags::from_bits_truncate(value)
    }
}

impl StaticType for DiscovererSerializeFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_discoverer_serialize_flags_get_type()) }
    }
}

impl glib::value::ValueType for DiscovererSerializeFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for DiscovererSerializeFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for DiscovererSerializeFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<DiscovererSerializeFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
