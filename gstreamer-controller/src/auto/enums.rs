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
#[doc(alias = "GstInterpolationMode")]
pub enum InterpolationMode {
    #[doc(alias = "GST_INTERPOLATION_MODE_NONE")]
    None,
    #[doc(alias = "GST_INTERPOLATION_MODE_LINEAR")]
    Linear,
    #[doc(alias = "GST_INTERPOLATION_MODE_CUBIC")]
    Cubic,
    #[doc(alias = "GST_INTERPOLATION_MODE_CUBIC_MONOTONIC")]
    CubicMonotonic,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for InterpolationMode {
    type GlibType = ffi::GstInterpolationMode;

    fn to_glib(&self) -> ffi::GstInterpolationMode {
        match *self {
            InterpolationMode::None => ffi::GST_INTERPOLATION_MODE_NONE,
            InterpolationMode::Linear => ffi::GST_INTERPOLATION_MODE_LINEAR,
            InterpolationMode::Cubic => ffi::GST_INTERPOLATION_MODE_CUBIC,
            InterpolationMode::CubicMonotonic => ffi::GST_INTERPOLATION_MODE_CUBIC_MONOTONIC,
            InterpolationMode::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstInterpolationMode> for InterpolationMode {
    unsafe fn from_glib(value: ffi::GstInterpolationMode) -> Self {
        skip_assert_initialized!();
        match value {
            0 => InterpolationMode::None,
            1 => InterpolationMode::Linear,
            2 => InterpolationMode::Cubic,
            3 => InterpolationMode::CubicMonotonic,
            value => InterpolationMode::__Unknown(value),
        }
    }
}

impl StaticType for InterpolationMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_interpolation_mode_get_type()) }
    }
}

impl glib::value::ValueType for InterpolationMode {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for InterpolationMode {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for InterpolationMode {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<InterpolationMode>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstLFOWaveform")]
pub enum LFOWaveform {
    #[doc(alias = "GST_LFO_WAVEFORM_SINE")]
    Sine,
    #[doc(alias = "GST_LFO_WAVEFORM_SQUARE")]
    Square,
    #[doc(alias = "GST_LFO_WAVEFORM_SAW")]
    Saw,
    #[doc(alias = "GST_LFO_WAVEFORM_REVERSE_SAW")]
    ReverseSaw,
    #[doc(alias = "GST_LFO_WAVEFORM_TRIANGLE")]
    Triangle,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for LFOWaveform {
    type GlibType = ffi::GstLFOWaveform;

    fn to_glib(&self) -> ffi::GstLFOWaveform {
        match *self {
            LFOWaveform::Sine => ffi::GST_LFO_WAVEFORM_SINE,
            LFOWaveform::Square => ffi::GST_LFO_WAVEFORM_SQUARE,
            LFOWaveform::Saw => ffi::GST_LFO_WAVEFORM_SAW,
            LFOWaveform::ReverseSaw => ffi::GST_LFO_WAVEFORM_REVERSE_SAW,
            LFOWaveform::Triangle => ffi::GST_LFO_WAVEFORM_TRIANGLE,
            LFOWaveform::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstLFOWaveform> for LFOWaveform {
    unsafe fn from_glib(value: ffi::GstLFOWaveform) -> Self {
        skip_assert_initialized!();
        match value {
            0 => LFOWaveform::Sine,
            1 => LFOWaveform::Square,
            2 => LFOWaveform::Saw,
            3 => LFOWaveform::ReverseSaw,
            4 => LFOWaveform::Triangle,
            value => LFOWaveform::__Unknown(value),
        }
    }
}

impl StaticType for LFOWaveform {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_lfo_waveform_get_type()) }
    }
}

impl glib::value::ValueType for LFOWaveform {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for LFOWaveform {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for LFOWaveform {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<LFOWaveform>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
