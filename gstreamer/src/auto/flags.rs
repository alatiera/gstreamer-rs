// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use glib::StaticType;
use glib::Type;
use gobject_sys;
use gst_sys;

bitflags! {
    pub struct BinFlags: u32 {
        const NO_RESYNC = 16384;
        const STREAMS_AWARE = 32768;
        const LAST = 524288;
    }
}

#[doc(hidden)]
impl ToGlib for BinFlags {
    type GlibType = gst_sys::GstBinFlags;

    fn to_glib(&self) -> gst_sys::GstBinFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstBinFlags> for BinFlags {
    fn from_glib(value: gst_sys::GstBinFlags) -> BinFlags {
        skip_assert_initialized!();
        BinFlags::from_bits_truncate(value)
    }
}

impl StaticType for BinFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_bin_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for BinFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for BinFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for BinFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct BufferCopyFlags: u32 {
        const NONE = 0;
        const FLAGS = 1;
        const TIMESTAMPS = 2;
        const META = 4;
        const MEMORY = 8;
        const MERGE = 16;
        const DEEP = 32;
    }
}

#[doc(hidden)]
impl ToGlib for BufferCopyFlags {
    type GlibType = gst_sys::GstBufferCopyFlags;

    fn to_glib(&self) -> gst_sys::GstBufferCopyFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstBufferCopyFlags> for BufferCopyFlags {
    fn from_glib(value: gst_sys::GstBufferCopyFlags) -> BufferCopyFlags {
        skip_assert_initialized!();
        BufferCopyFlags::from_bits_truncate(value)
    }
}

impl StaticType for BufferCopyFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_buffer_copy_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for BufferCopyFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for BufferCopyFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for BufferCopyFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    #[cfg_attr(feature = "ser_de", derive(Serialize, Deserialize))]
    pub struct BufferFlags: u32 {
        const LIVE = 16;
        const DECODE_ONLY = 32;
        const DISCONT = 64;
        const RESYNC = 128;
        const CORRUPTED = 256;
        const MARKER = 512;
        const HEADER = 1024;
        const GAP = 2048;
        const DROPPABLE = 4096;
        const DELTA_UNIT = 8192;
        const TAG_MEMORY = 16384;
        const SYNC_AFTER = 32768;
        const NON_DROPPABLE = 65536;
        const LAST = 1048576;
    }
}

#[doc(hidden)]
impl ToGlib for BufferFlags {
    type GlibType = gst_sys::GstBufferFlags;

    fn to_glib(&self) -> gst_sys::GstBufferFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstBufferFlags> for BufferFlags {
    fn from_glib(value: gst_sys::GstBufferFlags) -> BufferFlags {
        skip_assert_initialized!();
        BufferFlags::from_bits_truncate(value)
    }
}

impl StaticType for BufferFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_buffer_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for BufferFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for BufferFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for BufferFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct BufferPoolAcquireFlags: u32 {
        const NONE = 0;
        const KEY_UNIT = 1;
        const DONTWAIT = 2;
        const DISCONT = 4;
        const LAST = 65536;
    }
}

#[doc(hidden)]
impl ToGlib for BufferPoolAcquireFlags {
    type GlibType = gst_sys::GstBufferPoolAcquireFlags;

    fn to_glib(&self) -> gst_sys::GstBufferPoolAcquireFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstBufferPoolAcquireFlags> for BufferPoolAcquireFlags {
    fn from_glib(value: gst_sys::GstBufferPoolAcquireFlags) -> BufferPoolAcquireFlags {
        skip_assert_initialized!();
        BufferPoolAcquireFlags::from_bits_truncate(value)
    }
}

impl StaticType for BufferPoolAcquireFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_buffer_pool_acquire_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for BufferPoolAcquireFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for BufferPoolAcquireFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for BufferPoolAcquireFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct ClockFlags: u32 {
        const CAN_DO_SINGLE_SYNC = 16;
        const CAN_DO_SINGLE_ASYNC = 32;
        const CAN_DO_PERIODIC_SYNC = 64;
        const CAN_DO_PERIODIC_ASYNC = 128;
        const CAN_SET_RESOLUTION = 256;
        const CAN_SET_MASTER = 512;
        const NEEDS_STARTUP_SYNC = 1024;
        const LAST = 4096;
    }
}

#[doc(hidden)]
impl ToGlib for ClockFlags {
    type GlibType = gst_sys::GstClockFlags;

    fn to_glib(&self) -> gst_sys::GstClockFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstClockFlags> for ClockFlags {
    fn from_glib(value: gst_sys::GstClockFlags) -> ClockFlags {
        skip_assert_initialized!();
        ClockFlags::from_bits_truncate(value)
    }
}

impl StaticType for ClockFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_clock_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ClockFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ClockFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for ClockFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct DebugColorFlags: u32 {
        const FG_BLACK = 0;
        const FG_RED = 1;
        const FG_GREEN = 2;
        const FG_YELLOW = 3;
        const FG_BLUE = 4;
        const FG_MAGENTA = 5;
        const FG_CYAN = 6;
        const FG_WHITE = 7;
        const BG_BLACK = 0;
        const BG_RED = 16;
        const BG_GREEN = 32;
        const BG_YELLOW = 48;
        const BG_BLUE = 64;
        const BG_MAGENTA = 80;
        const BG_CYAN = 96;
        const BG_WHITE = 112;
        const BOLD = 256;
        const UNDERLINE = 512;
    }
}

#[doc(hidden)]
impl ToGlib for DebugColorFlags {
    type GlibType = gst_sys::GstDebugColorFlags;

    fn to_glib(&self) -> gst_sys::GstDebugColorFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstDebugColorFlags> for DebugColorFlags {
    fn from_glib(value: gst_sys::GstDebugColorFlags) -> DebugColorFlags {
        skip_assert_initialized!();
        DebugColorFlags::from_bits_truncate(value)
    }
}

impl StaticType for DebugColorFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_debug_color_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DebugColorFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DebugColorFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for DebugColorFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct DebugGraphDetails: u32 {
        const MEDIA_TYPE = 1;
        const CAPS_DETAILS = 2;
        const NON_DEFAULT_PARAMS = 4;
        const STATES = 8;
        const FULL_PARAMS = 16;
        const ALL = 15;
        const VERBOSE = 4294967295;
    }
}

#[doc(hidden)]
impl ToGlib for DebugGraphDetails {
    type GlibType = gst_sys::GstDebugGraphDetails;

    fn to_glib(&self) -> gst_sys::GstDebugGraphDetails {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstDebugGraphDetails> for DebugGraphDetails {
    fn from_glib(value: gst_sys::GstDebugGraphDetails) -> DebugGraphDetails {
        skip_assert_initialized!();
        DebugGraphDetails::from_bits_truncate(value)
    }
}

impl StaticType for DebugGraphDetails {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_debug_graph_details_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DebugGraphDetails {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DebugGraphDetails {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for DebugGraphDetails {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct ElementFlags: u32 {
        const LOCKED_STATE = 16;
        const SINK = 32;
        const SOURCE = 64;
        const PROVIDE_CLOCK = 128;
        const REQUIRE_CLOCK = 256;
        const INDEXABLE = 512;
        const LAST = 16384;
    }
}

#[doc(hidden)]
impl ToGlib for ElementFlags {
    type GlibType = gst_sys::GstElementFlags;

    fn to_glib(&self) -> gst_sys::GstElementFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstElementFlags> for ElementFlags {
    fn from_glib(value: gst_sys::GstElementFlags) -> ElementFlags {
        skip_assert_initialized!();
        ElementFlags::from_bits_truncate(value)
    }
}

impl StaticType for ElementFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_element_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ElementFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ElementFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for ElementFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct MemoryFlags: u32 {
        const READONLY = 2;
        const NO_SHARE = 16;
        const ZERO_PREFIXED = 32;
        const ZERO_PADDED = 64;
        const PHYSICALLY_CONTIGUOUS = 128;
        const NOT_MAPPABLE = 256;
        const LAST = 1048576;
    }
}

#[doc(hidden)]
impl ToGlib for MemoryFlags {
    type GlibType = gst_sys::GstMemoryFlags;

    fn to_glib(&self) -> gst_sys::GstMemoryFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstMemoryFlags> for MemoryFlags {
    fn from_glib(value: gst_sys::GstMemoryFlags) -> MemoryFlags {
        skip_assert_initialized!();
        MemoryFlags::from_bits_truncate(value)
    }
}

impl StaticType for MemoryFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_memory_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for MemoryFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for MemoryFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for MemoryFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct ObjectFlags: u32 {
        const MAY_BE_LEAKED = 1;
        const LAST = 16;
    }
}

#[doc(hidden)]
impl ToGlib for ObjectFlags {
    type GlibType = gst_sys::GstObjectFlags;

    fn to_glib(&self) -> gst_sys::GstObjectFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstObjectFlags> for ObjectFlags {
    fn from_glib(value: gst_sys::GstObjectFlags) -> ObjectFlags {
        skip_assert_initialized!();
        ObjectFlags::from_bits_truncate(value)
    }
}

impl StaticType for ObjectFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_object_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ObjectFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ObjectFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for ObjectFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct PadFlags: u32 {
        const BLOCKED = 16;
        const FLUSHING = 32;
        const EOS = 64;
        const BLOCKING = 128;
        const NEED_PARENT = 256;
        const NEED_RECONFIGURE = 512;
        const PENDING_EVENTS = 1024;
        const FIXED_CAPS = 2048;
        const PROXY_CAPS = 4096;
        const PROXY_ALLOCATION = 8192;
        const PROXY_SCHEDULING = 16384;
        const ACCEPT_INTERSECT = 32768;
        const ACCEPT_TEMPLATE = 65536;
        const LAST = 1048576;
    }
}

#[doc(hidden)]
impl ToGlib for PadFlags {
    type GlibType = gst_sys::GstPadFlags;

    fn to_glib(&self) -> gst_sys::GstPadFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstPadFlags> for PadFlags {
    fn from_glib(value: gst_sys::GstPadFlags) -> PadFlags {
        skip_assert_initialized!();
        PadFlags::from_bits_truncate(value)
    }
}

impl StaticType for PadFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_pad_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PadFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PadFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for PadFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct PadLinkCheck: u32 {
        const NOTHING = 0;
        const HIERARCHY = 1;
        const TEMPLATE_CAPS = 2;
        const CAPS = 4;
        const NO_RECONFIGURE = 8;
        const DEFAULT = 5;
    }
}

#[doc(hidden)]
impl ToGlib for PadLinkCheck {
    type GlibType = gst_sys::GstPadLinkCheck;

    fn to_glib(&self) -> gst_sys::GstPadLinkCheck {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstPadLinkCheck> for PadLinkCheck {
    fn from_glib(value: gst_sys::GstPadLinkCheck) -> PadLinkCheck {
        skip_assert_initialized!();
        PadLinkCheck::from_bits_truncate(value)
    }
}

impl StaticType for PadLinkCheck {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_pad_link_check_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PadLinkCheck {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PadLinkCheck {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for PadLinkCheck {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct PadProbeType: u32 {
        const INVALID = 0;
        const IDLE = 1;
        const BLOCK = 2;
        const BUFFER = 16;
        const BUFFER_LIST = 32;
        const EVENT_DOWNSTREAM = 64;
        const EVENT_UPSTREAM = 128;
        const EVENT_FLUSH = 256;
        const QUERY_DOWNSTREAM = 512;
        const QUERY_UPSTREAM = 1024;
        const PUSH = 4096;
        const PULL = 8192;
        const BLOCKING = 3;
        const DATA_DOWNSTREAM = 112;
        const DATA_UPSTREAM = 128;
        const DATA_BOTH = 240;
        const BLOCK_DOWNSTREAM = 114;
        const BLOCK_UPSTREAM = 130;
        const EVENT_BOTH = 192;
        const QUERY_BOTH = 1536;
        const ALL_BOTH = 1776;
        const SCHEDULING = 12288;
    }
}

#[doc(hidden)]
impl ToGlib for PadProbeType {
    type GlibType = gst_sys::GstPadProbeType;

    fn to_glib(&self) -> gst_sys::GstPadProbeType {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstPadProbeType> for PadProbeType {
    fn from_glib(value: gst_sys::GstPadProbeType) -> PadProbeType {
        skip_assert_initialized!();
        PadProbeType::from_bits_truncate(value)
    }
}

impl StaticType for PadProbeType {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_pad_probe_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PadProbeType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PadProbeType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for PadProbeType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct ParseFlags: u32 {
        const NONE = 0;
        const FATAL_ERRORS = 1;
        const NO_SINGLE_ELEMENT_BINS = 2;
        const PLACE_IN_BIN = 4;
    }
}

#[doc(hidden)]
impl ToGlib for ParseFlags {
    type GlibType = gst_sys::GstParseFlags;

    fn to_glib(&self) -> gst_sys::GstParseFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstParseFlags> for ParseFlags {
    fn from_glib(value: gst_sys::GstParseFlags) -> ParseFlags {
        skip_assert_initialized!();
        ParseFlags::from_bits_truncate(value)
    }
}

impl StaticType for ParseFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_parse_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ParseFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ParseFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for ParseFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct PipelineFlags: u32 {
        const FIXED_CLOCK = 524288;
        const LAST = 8388608;
    }
}

#[doc(hidden)]
impl ToGlib for PipelineFlags {
    type GlibType = gst_sys::GstPipelineFlags;

    fn to_glib(&self) -> gst_sys::GstPipelineFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstPipelineFlags> for PipelineFlags {
    fn from_glib(value: gst_sys::GstPipelineFlags) -> PipelineFlags {
        skip_assert_initialized!();
        PipelineFlags::from_bits_truncate(value)
    }
}

impl StaticType for PipelineFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_pipeline_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PipelineFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PipelineFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for PipelineFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct PluginDependencyFlags: u32 {
        const NONE = 0;
        const RECURSE = 1;
        const PATHS_ARE_DEFAULT_ONLY = 2;
        const FILE_NAME_IS_SUFFIX = 4;
        const FILE_NAME_IS_PREFIX = 8;
        const PATHS_ARE_RELATIVE_TO_EXE = 16;
    }
}

#[doc(hidden)]
impl ToGlib for PluginDependencyFlags {
    type GlibType = gst_sys::GstPluginDependencyFlags;

    fn to_glib(&self) -> gst_sys::GstPluginDependencyFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstPluginDependencyFlags> for PluginDependencyFlags {
    fn from_glib(value: gst_sys::GstPluginDependencyFlags) -> PluginDependencyFlags {
        skip_assert_initialized!();
        PluginDependencyFlags::from_bits_truncate(value)
    }
}

impl StaticType for PluginDependencyFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_plugin_dependency_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PluginDependencyFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PluginDependencyFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for PluginDependencyFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct PluginFlags: u32 {
        const CACHED = 16;
        const BLACKLISTED = 32;
    }
}

#[doc(hidden)]
impl ToGlib for PluginFlags {
    type GlibType = gst_sys::GstPluginFlags;

    fn to_glib(&self) -> gst_sys::GstPluginFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstPluginFlags> for PluginFlags {
    fn from_glib(value: gst_sys::GstPluginFlags) -> PluginFlags {
        skip_assert_initialized!();
        PluginFlags::from_bits_truncate(value)
    }
}

impl StaticType for PluginFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_plugin_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PluginFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PluginFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for PluginFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct SchedulingFlags: u32 {
        const SEEKABLE = 1;
        const SEQUENTIAL = 2;
        const BANDWIDTH_LIMITED = 4;
    }
}

#[doc(hidden)]
impl ToGlib for SchedulingFlags {
    type GlibType = gst_sys::GstSchedulingFlags;

    fn to_glib(&self) -> gst_sys::GstSchedulingFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstSchedulingFlags> for SchedulingFlags {
    fn from_glib(value: gst_sys::GstSchedulingFlags) -> SchedulingFlags {
        skip_assert_initialized!();
        SchedulingFlags::from_bits_truncate(value)
    }
}

impl StaticType for SchedulingFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_scheduling_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SchedulingFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SchedulingFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for SchedulingFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct SeekFlags: u32 {
        const NONE = 0;
        const FLUSH = 1;
        const ACCURATE = 2;
        const KEY_UNIT = 4;
        const SEGMENT = 8;
        const TRICKMODE = 16;
        const SKIP = 16;
        const SNAP_BEFORE = 32;
        const SNAP_AFTER = 64;
        const SNAP_NEAREST = 96;
        const TRICKMODE_KEY_UNITS = 128;
        const TRICKMODE_NO_AUDIO = 256;
    }
}

#[doc(hidden)]
impl ToGlib for SeekFlags {
    type GlibType = gst_sys::GstSeekFlags;

    fn to_glib(&self) -> gst_sys::GstSeekFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstSeekFlags> for SeekFlags {
    fn from_glib(value: gst_sys::GstSeekFlags) -> SeekFlags {
        skip_assert_initialized!();
        SeekFlags::from_bits_truncate(value)
    }
}

impl StaticType for SeekFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_seek_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SeekFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SeekFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for SeekFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    #[cfg_attr(feature = "ser_de", derive(Serialize, Deserialize))]
    pub struct SegmentFlags: u32 {
        const NONE = 0;
        const RESET = 1;
        const TRICKMODE = 16;
        const SKIP = 16;
        const SEGMENT = 8;
        const TRICKMODE_KEY_UNITS = 128;
        const TRICKMODE_NO_AUDIO = 256;
    }
}

#[doc(hidden)]
impl ToGlib for SegmentFlags {
    type GlibType = gst_sys::GstSegmentFlags;

    fn to_glib(&self) -> gst_sys::GstSegmentFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstSegmentFlags> for SegmentFlags {
    fn from_glib(value: gst_sys::GstSegmentFlags) -> SegmentFlags {
        skip_assert_initialized!();
        SegmentFlags::from_bits_truncate(value)
    }
}

impl StaticType for SegmentFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_segment_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SegmentFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SegmentFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for SegmentFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
bitflags! {
    pub struct StackTraceFlags: u32 {
        const FULL = 1;
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for StackTraceFlags {
    type GlibType = gst_sys::GstStackTraceFlags;

    fn to_glib(&self) -> gst_sys::GstStackTraceFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<gst_sys::GstStackTraceFlags> for StackTraceFlags {
    fn from_glib(value: gst_sys::GstStackTraceFlags) -> StackTraceFlags {
        skip_assert_initialized!();
        StackTraceFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl StaticType for StackTraceFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_stack_trace_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl<'a> FromValueOptional<'a> for StackTraceFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl<'a> FromValue<'a> for StackTraceFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl SetValue for StackTraceFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct StreamFlags: u32 {
        const NONE = 0;
        const SPARSE = 1;
        const SELECT = 2;
        const UNSELECT = 4;
    }
}

#[doc(hidden)]
impl ToGlib for StreamFlags {
    type GlibType = gst_sys::GstStreamFlags;

    fn to_glib(&self) -> gst_sys::GstStreamFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_sys::GstStreamFlags> for StreamFlags {
    fn from_glib(value: gst_sys::GstStreamFlags) -> StreamFlags {
        skip_assert_initialized!();
        StreamFlags::from_bits_truncate(value)
    }
}

impl StaticType for StreamFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_stream_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for StreamFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for StreamFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for StreamFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
bitflags! {
    pub struct StreamType: u32 {
        const UNKNOWN = 1;
        const AUDIO = 2;
        const VIDEO = 4;
        const CONTAINER = 8;
        const TEXT = 16;
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for StreamType {
    type GlibType = gst_sys::GstStreamType;

    fn to_glib(&self) -> gst_sys::GstStreamType {
        self.bits()
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<gst_sys::GstStreamType> for StreamType {
    fn from_glib(value: gst_sys::GstStreamType) -> StreamType {
        skip_assert_initialized!();
        StreamType::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl StaticType for StreamType {
    fn static_type() -> Type {
        unsafe { from_glib(gst_sys::gst_stream_type_get_type()) }
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl<'a> FromValueOptional<'a> for StreamType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl<'a> FromValue<'a> for StreamType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl SetValue for StreamType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}
