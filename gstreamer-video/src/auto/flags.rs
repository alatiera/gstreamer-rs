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
    pub struct VideoBufferFlags: u32 {
        const INTERLACED = 1048576;
        const TFF = 2097152;
        const RFF = 4194304;
        const ONEFIELD = 8388608;
        const MULTIPLE_VIEW = 16777216;
        const FIRST_IN_BUNDLE = 33554432;
        #[cfg(any(feature = "v1_16", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
        const TOP_FIELD = 10485760;
        #[cfg(any(feature = "v1_16", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
        const BOTTOM_FIELD = 8388608;
        #[cfg(any(feature = "v1_18", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
        const MARKER = 512;
    }
}

#[doc(hidden)]
impl ToGlib for VideoBufferFlags {
    type GlibType = ffi::GstVideoBufferFlags;

    fn to_glib(&self) -> ffi::GstVideoBufferFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoBufferFlags> for VideoBufferFlags {
    unsafe fn from_glib(value: ffi::GstVideoBufferFlags) -> VideoBufferFlags {
        skip_assert_initialized!();
        VideoBufferFlags::from_bits_truncate(value)
    }
}

impl StaticType for VideoBufferFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_buffer_flags_get_type()) }
    }
}

impl glib::value::ValueType for VideoBufferFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for VideoBufferFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoBufferFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<VideoBufferFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct VideoChromaSite: u32 {
        const NONE = 1;
        const H_COSITED = 2;
        const V_COSITED = 4;
        const ALT_LINE = 8;
        const COSITED = 6;
        const JPEG = 1;
        const MPEG2 = 2;
        const DV = 14;
    }
}

#[doc(hidden)]
impl ToGlib for VideoChromaSite {
    type GlibType = ffi::GstVideoChromaSite;

    fn to_glib(&self) -> ffi::GstVideoChromaSite {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoChromaSite> for VideoChromaSite {
    unsafe fn from_glib(value: ffi::GstVideoChromaSite) -> VideoChromaSite {
        skip_assert_initialized!();
        VideoChromaSite::from_bits_truncate(value)
    }
}

impl StaticType for VideoChromaSite {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_chroma_site_get_type()) }
    }
}

impl glib::value::ValueType for VideoChromaSite {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for VideoChromaSite {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoChromaSite {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<VideoChromaSite>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct VideoCodecFrameFlags: u32 {
        const DECODE_ONLY = 1;
        const SYNC_POINT = 2;
        const FORCE_KEYFRAME = 4;
        const FORCE_KEYFRAME_HEADERS = 8;
    }
}

#[doc(hidden)]
impl ToGlib for VideoCodecFrameFlags {
    type GlibType = ffi::GstVideoCodecFrameFlags;

    fn to_glib(&self) -> ffi::GstVideoCodecFrameFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoCodecFrameFlags> for VideoCodecFrameFlags {
    unsafe fn from_glib(value: ffi::GstVideoCodecFrameFlags) -> VideoCodecFrameFlags {
        skip_assert_initialized!();
        VideoCodecFrameFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct VideoFlags: u32 {
        const VARIABLE_FPS = 1;
        const PREMULTIPLIED_ALPHA = 2;
    }
}

#[doc(hidden)]
impl ToGlib for VideoFlags {
    type GlibType = ffi::GstVideoFlags;

    fn to_glib(&self) -> ffi::GstVideoFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoFlags> for VideoFlags {
    unsafe fn from_glib(value: ffi::GstVideoFlags) -> VideoFlags {
        skip_assert_initialized!();
        VideoFlags::from_bits_truncate(value)
    }
}

impl StaticType for VideoFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_flags_get_type()) }
    }
}

impl glib::value::ValueType for VideoFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for VideoFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<VideoFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct VideoFormatFlags: u32 {
        const YUV = 1;
        const RGB = 2;
        const GRAY = 4;
        const ALPHA = 8;
        const LE = 16;
        const PALETTE = 32;
        const COMPLEX = 64;
        const UNPACK = 128;
        const TILED = 256;
    }
}

#[doc(hidden)]
impl ToGlib for VideoFormatFlags {
    type GlibType = ffi::GstVideoFormatFlags;

    fn to_glib(&self) -> ffi::GstVideoFormatFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoFormatFlags> for VideoFormatFlags {
    unsafe fn from_glib(value: ffi::GstVideoFormatFlags) -> VideoFormatFlags {
        skip_assert_initialized!();
        VideoFormatFlags::from_bits_truncate(value)
    }
}

impl StaticType for VideoFormatFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_format_flags_get_type()) }
    }
}

impl glib::value::ValueType for VideoFormatFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for VideoFormatFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoFormatFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<VideoFormatFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct VideoFrameFlags: u32 {
        const INTERLACED = 1;
        const TFF = 2;
        const RFF = 4;
        const ONEFIELD = 8;
        const MULTIPLE_VIEW = 16;
        const FIRST_IN_BUNDLE = 32;
        const TOP_FIELD = 10;
        const BOTTOM_FIELD = 8;
    }
}

#[doc(hidden)]
impl ToGlib for VideoFrameFlags {
    type GlibType = ffi::GstVideoFrameFlags;

    fn to_glib(&self) -> ffi::GstVideoFrameFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoFrameFlags> for VideoFrameFlags {
    unsafe fn from_glib(value: ffi::GstVideoFrameFlags) -> VideoFrameFlags {
        skip_assert_initialized!();
        VideoFrameFlags::from_bits_truncate(value)
    }
}

impl StaticType for VideoFrameFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_frame_flags_get_type()) }
    }
}

impl glib::value::ValueType for VideoFrameFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for VideoFrameFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoFrameFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<VideoFrameFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct VideoMultiviewFlags: u32 {
        const RIGHT_VIEW_FIRST = 1;
        const LEFT_FLIPPED = 2;
        const LEFT_FLOPPED = 4;
        const RIGHT_FLIPPED = 8;
        const RIGHT_FLOPPED = 16;
        const HALF_ASPECT = 16384;
        const MIXED_MONO = 32768;
    }
}

#[doc(hidden)]
impl ToGlib for VideoMultiviewFlags {
    type GlibType = ffi::GstVideoMultiviewFlags;

    fn to_glib(&self) -> ffi::GstVideoMultiviewFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoMultiviewFlags> for VideoMultiviewFlags {
    unsafe fn from_glib(value: ffi::GstVideoMultiviewFlags) -> VideoMultiviewFlags {
        skip_assert_initialized!();
        VideoMultiviewFlags::from_bits_truncate(value)
    }
}

impl StaticType for VideoMultiviewFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_multiview_flags_get_type()) }
    }
}

impl glib::value::ValueType for VideoMultiviewFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for VideoMultiviewFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoMultiviewFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<VideoMultiviewFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct VideoOverlayFormatFlags: u32 {
        const PREMULTIPLIED_ALPHA = 1;
        const GLOBAL_ALPHA = 2;
    }
}

#[doc(hidden)]
impl ToGlib for VideoOverlayFormatFlags {
    type GlibType = ffi::GstVideoOverlayFormatFlags;

    fn to_glib(&self) -> ffi::GstVideoOverlayFormatFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoOverlayFormatFlags> for VideoOverlayFormatFlags {
    unsafe fn from_glib(value: ffi::GstVideoOverlayFormatFlags) -> VideoOverlayFormatFlags {
        skip_assert_initialized!();
        VideoOverlayFormatFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
impl StaticType for VideoOverlayFormatFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_overlay_format_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
impl glib::value::ValueType for VideoOverlayFormatFlags {
    type Type = Self;
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
unsafe impl<'a> FromValue<'a> for VideoOverlayFormatFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
impl ToValue for VideoOverlayFormatFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<VideoOverlayFormatFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct VideoPackFlags: u32 {
        const TRUNCATE_RANGE = 1;
        const INTERLACED = 2;
    }
}

#[doc(hidden)]
impl ToGlib for VideoPackFlags {
    type GlibType = ffi::GstVideoPackFlags;

    fn to_glib(&self) -> ffi::GstVideoPackFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoPackFlags> for VideoPackFlags {
    unsafe fn from_glib(value: ffi::GstVideoPackFlags) -> VideoPackFlags {
        skip_assert_initialized!();
        VideoPackFlags::from_bits_truncate(value)
    }
}

impl StaticType for VideoPackFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_pack_flags_get_type()) }
    }
}

impl glib::value::ValueType for VideoPackFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for VideoPackFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoPackFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<VideoPackFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
bitflags! {
    pub struct VideoTimeCodeFlags: u32 {
        const DROP_FRAME = 1;
        const INTERLACED = 2;
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
#[doc(hidden)]
impl ToGlib for VideoTimeCodeFlags {
    type GlibType = ffi::GstVideoTimeCodeFlags;

    fn to_glib(&self) -> ffi::GstVideoTimeCodeFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
#[doc(hidden)]
impl FromGlib<ffi::GstVideoTimeCodeFlags> for VideoTimeCodeFlags {
    unsafe fn from_glib(value: ffi::GstVideoTimeCodeFlags) -> VideoTimeCodeFlags {
        skip_assert_initialized!();
        VideoTimeCodeFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
impl StaticType for VideoTimeCodeFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_time_code_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
impl glib::value::ValueType for VideoTimeCodeFlags {
    type Type = Self;
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
unsafe impl<'a> FromValue<'a> for VideoTimeCodeFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
impl ToValue for VideoTimeCodeFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<VideoTimeCodeFlags>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.to_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
