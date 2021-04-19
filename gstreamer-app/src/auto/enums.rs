// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstAppStreamType")]
pub enum AppStreamType {
    #[doc(alias = "GST_APP_STREAM_TYPE_STREAM")]
    Stream,
    #[doc(alias = "GST_APP_STREAM_TYPE_SEEKABLE")]
    Seekable,
    #[doc(alias = "GST_APP_STREAM_TYPE_RANDOM_ACCESS")]
    RandomAccess,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for AppStreamType {
    type GlibType = ffi::GstAppStreamType;

    fn to_glib(&self) -> ffi::GstAppStreamType {
        match *self {
            AppStreamType::Stream => ffi::GST_APP_STREAM_TYPE_STREAM,
            AppStreamType::Seekable => ffi::GST_APP_STREAM_TYPE_SEEKABLE,
            AppStreamType::RandomAccess => ffi::GST_APP_STREAM_TYPE_RANDOM_ACCESS,
            AppStreamType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAppStreamType> for AppStreamType {
    unsafe fn from_glib(value: ffi::GstAppStreamType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => AppStreamType::Stream,
            1 => AppStreamType::Seekable,
            2 => AppStreamType::RandomAccess,
            value => AppStreamType::__Unknown(value),
        }
    }
}

impl StaticType for AppStreamType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_app_stream_type_get_type()) }
    }
}

impl glib::value::ValueType for AppStreamType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AppStreamType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for AppStreamType {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<AppStreamType>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
