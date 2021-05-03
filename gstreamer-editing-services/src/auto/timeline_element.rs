// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use crate::Edge;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use crate::EditMode;
use crate::Extractable;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use crate::Layer;
use crate::Timeline;
use crate::TrackType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use std::mem;
use std::mem::transmute;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use std::ptr;

glib::wrapper! {
    pub struct TimelineElement(Object<ffi::GESTimelineElement, ffi::GESTimelineElementClass>) @implements Extractable;

    match fn {
        type_ => || ffi::ges_timeline_element_get_type(),
    }
}

pub const NONE_TIMELINE_ELEMENT: Option<&TimelineElement> = None;

pub trait TimelineElementExt: 'static {
    //#[doc(alias = "ges_timeline_element_add_child_property")]
    //fn add_child_property<P: IsA<glib::Object>>(&self, pspec: /*Ignored*/&glib::ParamSpec, child: &P) -> bool;

    #[doc(alias = "ges_timeline_element_copy")]
    fn copy(&self, deep: bool) -> Result<TimelineElement, glib::BoolError>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_timeline_element_edit")]
    fn edit(
        &self,
        layers: &[Layer],
        new_layer_priority: i64,
        mode: EditMode,
        edge: Edge,
        position: u64,
    ) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_timeline_element_edit_full")]
    fn edit_full(
        &self,
        new_layer_priority: i64,
        mode: EditMode,
        edge: Edge,
        position: u64,
    ) -> Result<(), glib::Error>;

    //#[doc(alias = "ges_timeline_element_get_child_properties")]
    //#[doc(alias = "get_child_properties")]
    //fn child_properties(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //#[doc(alias = "ges_timeline_element_get_child_property")]
    //#[doc(alias = "get_child_property")]
    //fn child_property(&self, property_name: &str, value: /*Ignored*/glib::Value) -> bool;

    //#[doc(alias = "ges_timeline_element_get_child_property_by_pspec")]
    //#[doc(alias = "get_child_property_by_pspec")]
    //fn child_property_by_pspec(&self, pspec: /*Ignored*/&glib::ParamSpec, value: /*Ignored*/glib::Value);

    //#[doc(alias = "ges_timeline_element_get_child_property_valist")]
    //#[doc(alias = "get_child_property_valist")]
    //fn child_property_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    #[doc(alias = "ges_timeline_element_get_duration")]
    #[doc(alias = "get_duration")]
    fn duration(&self) -> gst::ClockTime;

    #[doc(alias = "ges_timeline_element_get_inpoint")]
    #[doc(alias = "get_inpoint")]
    fn inpoint(&self) -> gst::ClockTime;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "ges_timeline_element_get_layer_priority")]
    #[doc(alias = "get_layer_priority")]
    fn layer_priority(&self) -> u32;

    #[doc(alias = "ges_timeline_element_get_max_duration")]
    #[doc(alias = "get_max_duration")]
    fn max_duration(&self) -> gst::ClockTime;

    #[doc(alias = "ges_timeline_element_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_timeline_element_get_natural_framerate")]
    #[doc(alias = "get_natural_framerate")]
    fn natural_framerate(&self) -> Option<(i32, i32)>;

    #[doc(alias = "ges_timeline_element_get_parent")]
    #[doc(alias = "get_parent")]
    fn parent(&self) -> Option<TimelineElement>;

    #[doc(alias = "ges_timeline_element_get_priority")]
    #[doc(alias = "get_priority")]
    fn priority(&self) -> u32;

    #[doc(alias = "ges_timeline_element_get_start")]
    #[doc(alias = "get_start")]
    fn start(&self) -> gst::ClockTime;

    #[doc(alias = "ges_timeline_element_get_timeline")]
    #[doc(alias = "get_timeline")]
    fn timeline(&self) -> Option<Timeline>;

    #[doc(alias = "ges_timeline_element_get_toplevel_parent")]
    #[doc(alias = "get_toplevel_parent")]
    fn toplevel_parent(&self) -> Option<TimelineElement>;

    #[doc(alias = "ges_timeline_element_get_track_types")]
    #[doc(alias = "get_track_types")]
    fn track_types(&self) -> TrackType;

    //#[doc(alias = "ges_timeline_element_list_children_properties")]
    //fn list_children_properties(&self) -> /*Ignored*/Vec<glib::ParamSpec>;

    //#[doc(alias = "ges_timeline_element_lookup_child")]
    //fn lookup_child(&self, prop_name: &str, pspec: /*Ignored*/glib::ParamSpec) -> Option<glib::Object>;

    #[doc(alias = "ges_timeline_element_paste")]
    fn paste(&self, paste_position: gst::ClockTime) -> Result<TimelineElement, glib::BoolError>;

    //#[doc(alias = "ges_timeline_element_remove_child_property")]
    //fn remove_child_property(&self, pspec: /*Ignored*/&glib::ParamSpec) -> bool;

    #[doc(alias = "ges_timeline_element_ripple")]
    fn ripple(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "ges_timeline_element_ripple_end")]
    fn ripple_end(&self, end: gst::ClockTime) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "ges_timeline_element_roll_end")]
    fn roll_end(&self, end: gst::ClockTime) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "ges_timeline_element_roll_start")]
    fn roll_start(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError>;

    //#[doc(alias = "ges_timeline_element_set_child_properties")]
    //fn set_child_properties(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //#[doc(alias = "ges_timeline_element_set_child_property")]
    //fn set_child_property(&self, property_name: &str, value: /*Ignored*/&glib::Value) -> bool;

    //#[doc(alias = "ges_timeline_element_set_child_property_by_pspec")]
    //fn set_child_property_by_pspec(&self, pspec: /*Ignored*/&glib::ParamSpec, value: /*Ignored*/&glib::Value);

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    //#[doc(alias = "ges_timeline_element_set_child_property_full")]
    //fn set_child_property_full(&self, property_name: &str, value: /*Ignored*/&glib::Value) -> Result<(), glib::Error>;

    //#[doc(alias = "ges_timeline_element_set_child_property_valist")]
    //fn set_child_property_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    #[doc(alias = "ges_timeline_element_set_duration")]
    fn set_duration(&self, duration: gst::ClockTime) -> bool;

    #[doc(alias = "ges_timeline_element_set_inpoint")]
    fn set_inpoint(&self, inpoint: gst::ClockTime) -> bool;

    #[doc(alias = "ges_timeline_element_set_max_duration")]
    fn set_max_duration(&self, maxduration: gst::ClockTime) -> bool;

    #[doc(alias = "ges_timeline_element_set_name")]
    fn set_name(&self, name: Option<&str>) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "ges_timeline_element_set_parent")]
    fn set_parent<P: IsA<TimelineElement>>(&self, parent: &P)
        -> Result<(), glib::error::BoolError>;

    #[cfg_attr(feature = "v1_10", deprecated = "Since 1.10")]
    #[doc(alias = "ges_timeline_element_set_priority")]
    fn set_priority(&self, priority: u32) -> bool;

    #[doc(alias = "ges_timeline_element_set_start")]
    fn set_start(&self, start: gst::ClockTime) -> bool;

    #[doc(alias = "ges_timeline_element_set_timeline")]
    fn set_timeline<P: IsA<Timeline>>(&self, timeline: &P) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "ges_timeline_element_trim")]
    fn trim(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "in-point")]
    fn in_point(&self) -> u64;

    #[doc(alias = "in-point")]
    fn set_in_point(&self, in_point: u64);

    fn is_serialize(&self) -> bool;

    fn set_serialize(&self, serialize: bool);

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    //#[doc(alias = "child-property-added")]
    //fn connect_child_property_added<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    //#[doc(alias = "child-property-removed")]
    //fn connect_child_property_removed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //#[doc(alias = "deep-notify")]
    //fn connect_deep_notify<Unsupported or ignored types>(&self, detail: Option<&str>, f: F) -> SignalHandlerId;

    #[doc(alias = "duration")]
    fn connect_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "in-point")]
    fn connect_in_point_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "max-duration")]
    fn connect_max_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "parent")]
    fn connect_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v1_10", deprecated = "Since 1.10")]
    #[doc(alias = "priority")]
    fn connect_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "serialize")]
    fn connect_serialize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "start")]
    fn connect_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "timeline")]
    fn connect_timeline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TimelineElement>> TimelineElementExt for O {
    //fn add_child_property<P: IsA<glib::Object>>(&self, pspec: /*Ignored*/&glib::ParamSpec, child: &P) -> bool {
    //    unsafe { TODO: call ffi:ges_timeline_element_add_child_property() }
    //}

    fn copy(&self, deep: bool) -> Result<TimelineElement, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_none(ffi::ges_timeline_element_copy(
                self.as_ref().to_glib_none().0,
                deep.into_glib(),
            ))
            .ok_or_else(|| glib::bool_error!("Failed to copy timeline element"))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn edit(
        &self,
        layers: &[Layer],
        new_layer_priority: i64,
        mode: EditMode,
        edge: Edge,
        position: u64,
    ) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_element_edit(
                self.as_ref().to_glib_none().0,
                layers.to_glib_none().0,
                new_layer_priority,
                mode.into_glib(),
                edge.into_glib(),
                position,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn edit_full(
        &self,
        new_layer_priority: i64,
        mode: EditMode,
        edge: Edge,
        position: u64,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::ges_timeline_element_edit_full(
                self.as_ref().to_glib_none().0,
                new_layer_priority,
                mode.into_glib(),
                edge.into_glib(),
                position,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //fn child_properties(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:ges_timeline_element_get_child_properties() }
    //}

    //fn child_property(&self, property_name: &str, value: /*Ignored*/glib::Value) -> bool {
    //    unsafe { TODO: call ffi:ges_timeline_element_get_child_property() }
    //}

    //fn child_property_by_pspec(&self, pspec: /*Ignored*/&glib::ParamSpec, value: /*Ignored*/glib::Value) {
    //    unsafe { TODO: call ffi:ges_timeline_element_get_child_property_by_pspec() }
    //}

    //fn child_property_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:ges_timeline_element_get_child_property_valist() }
    //}

    fn duration(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::ges_timeline_element_get_duration(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn inpoint(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::ges_timeline_element_get_inpoint(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn layer_priority(&self) -> u32 {
        unsafe { ffi::ges_timeline_element_get_layer_priority(self.as_ref().to_glib_none().0) }
    }

    fn max_duration(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::ges_timeline_element_get_max_duration(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::ges_timeline_element_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn natural_framerate(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut framerate_n = mem::MaybeUninit::uninit();
            let mut framerate_d = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::ges_timeline_element_get_natural_framerate(
                self.as_ref().to_glib_none().0,
                framerate_n.as_mut_ptr(),
                framerate_d.as_mut_ptr(),
            ));
            let framerate_n = framerate_n.assume_init();
            let framerate_d = framerate_d.assume_init();
            if ret {
                Some((framerate_n, framerate_d))
            } else {
                None
            }
        }
    }

    fn parent(&self) -> Option<TimelineElement> {
        unsafe {
            from_glib_full(ffi::ges_timeline_element_get_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn priority(&self) -> u32 {
        unsafe { ffi::ges_timeline_element_get_priority(self.as_ref().to_glib_none().0) }
    }

    fn start(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::ges_timeline_element_get_start(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn timeline(&self) -> Option<Timeline> {
        unsafe {
            from_glib_full(ffi::ges_timeline_element_get_timeline(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn toplevel_parent(&self) -> Option<TimelineElement> {
        unsafe {
            from_glib_full(ffi::ges_timeline_element_get_toplevel_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn track_types(&self) -> TrackType {
        unsafe {
            from_glib(ffi::ges_timeline_element_get_track_types(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn list_children_properties(&self) -> /*Ignored*/Vec<glib::ParamSpec> {
    //    unsafe { TODO: call ffi:ges_timeline_element_list_children_properties() }
    //}

    //fn lookup_child(&self, prop_name: &str, pspec: /*Ignored*/glib::ParamSpec) -> Option<glib::Object> {
    //    unsafe { TODO: call ffi:ges_timeline_element_lookup_child() }
    //}

    fn paste(&self, paste_position: gst::ClockTime) -> Result<TimelineElement, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::ges_timeline_element_paste(
                self.as_ref().to_glib_none().0,
                paste_position.into_glib(),
            ))
            .ok_or_else(|| glib::bool_error!("Failed to paste timeline element"))
        }
    }

    //fn remove_child_property(&self, pspec: /*Ignored*/&glib::ParamSpec) -> bool {
    //    unsafe { TODO: call ffi:ges_timeline_element_remove_child_property() }
    //}

    fn ripple(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_ripple(self.as_ref().to_glib_none().0, start.into_glib()),
                "Failed to ripple"
            )
        }
    }

    fn ripple_end(&self, end: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_ripple_end(
                    self.as_ref().to_glib_none().0,
                    end.into_glib()
                ),
                "Failed to ripple"
            )
        }
    }

    fn roll_end(&self, end: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_roll_end(self.as_ref().to_glib_none().0, end.into_glib()),
                "Failed to roll"
            )
        }
    }

    fn roll_start(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_roll_start(
                    self.as_ref().to_glib_none().0,
                    start.into_glib()
                ),
                "Failed to roll"
            )
        }
    }

    //fn set_child_properties(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:ges_timeline_element_set_child_properties() }
    //}

    //fn set_child_property(&self, property_name: &str, value: /*Ignored*/&glib::Value) -> bool {
    //    unsafe { TODO: call ffi:ges_timeline_element_set_child_property() }
    //}

    //fn set_child_property_by_pspec(&self, pspec: /*Ignored*/&glib::ParamSpec, value: /*Ignored*/&glib::Value) {
    //    unsafe { TODO: call ffi:ges_timeline_element_set_child_property_by_pspec() }
    //}

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    //fn set_child_property_full(&self, property_name: &str, value: /*Ignored*/&glib::Value) -> Result<(), glib::Error> {
    //    unsafe { TODO: call ffi:ges_timeline_element_set_child_property_full() }
    //}

    //fn set_child_property_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:ges_timeline_element_set_child_property_valist() }
    //}

    fn set_duration(&self, duration: gst::ClockTime) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_element_set_duration(
                self.as_ref().to_glib_none().0,
                duration.into_glib(),
            ))
        }
    }

    fn set_inpoint(&self, inpoint: gst::ClockTime) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_element_set_inpoint(
                self.as_ref().to_glib_none().0,
                inpoint.into_glib(),
            ))
        }
    }

    fn set_max_duration(&self, maxduration: gst::ClockTime) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_element_set_max_duration(
                self.as_ref().to_glib_none().0,
                maxduration.into_glib(),
            ))
        }
    }

    fn set_name(&self, name: Option<&str>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_set_name(
                    self.as_ref().to_glib_none().0,
                    name.to_glib_none().0
                ),
                "Failed to set name"
            )
        }
    }

    fn set_parent<P: IsA<TimelineElement>>(
        &self,
        parent: &P,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_set_parent(
                    self.as_ref().to_glib_none().0,
                    parent.as_ref().to_glib_none().0
                ),
                "`TimelineElement` already had a parent or its parent was the same as specified"
            )
        }
    }

    fn set_priority(&self, priority: u32) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_element_set_priority(
                self.as_ref().to_glib_none().0,
                priority,
            ))
        }
    }

    fn set_start(&self, start: gst::ClockTime) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_element_set_start(
                self.as_ref().to_glib_none().0,
                start.into_glib(),
            ))
        }
    }

    fn set_timeline<P: IsA<Timeline>>(&self, timeline: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_set_timeline(
                    self.as_ref().to_glib_none().0,
                    timeline.as_ref().to_glib_none().0
                ),
                "`Failed to set timeline"
            )
        }
    }

    fn trim(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_trim(self.as_ref().to_glib_none().0, start.into_glib()),
                "Failed to trim"
            )
        }
    }

    fn in_point(&self) -> u64 {
        unsafe {
            let mut value = glib::Value::from_type(<u64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"in-point\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `in-point` getter")
        }
    }

    fn set_in_point(&self, in_point: u64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"in-point\0".as_ptr() as *const _,
                in_point.to_value().to_glib_none().0,
            );
        }
    }

    fn is_serialize(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"serialize\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `serialize` getter")
        }
    }

    fn set_serialize(&self, serialize: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"serialize\0".as_ptr() as *const _,
                serialize.to_value().to_glib_none().0,
            );
        }
    }

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    //#[doc(alias = "child-property-added")]
    //fn connect_child_property_added<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored prop: GObject.ParamSpec
    //}

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    //#[doc(alias = "child-property-removed")]
    //fn connect_child_property_removed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored prop: GObject.ParamSpec
    //}

    //#[doc(alias = "deep-notify")]
    //fn connect_deep_notify<Unsupported or ignored types>(&self, detail: Option<&str>, f: F) -> SignalHandlerId {
    //    Ignored prop: GObject.ParamSpec
    //}

    #[doc(alias = "duration")]
    fn connect_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_duration_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_duration_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "in-point")]
    fn connect_in_point_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_in_point_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::in-point\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_in_point_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-duration")]
    fn connect_max_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_duration_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_duration_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "parent")]
    fn connect_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "priority")]
    fn connect_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_priority_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::priority\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_priority_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "serialize")]
    fn connect_serialize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_serialize_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::serialize\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_serialize_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "start")]
    fn connect_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_start_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::start\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_start_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "timeline")]
    fn connect_timeline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeline_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimelineElement>,
        {
            let f: &F = &*(f as *const F);
            f(&TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeline\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_timeline_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
