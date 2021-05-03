// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    pub struct GLDisplayWayland(Object<ffi::GstGLDisplayWayland, ffi::GstGLDisplayWaylandClass>) @extends gst_gl::GLDisplay, gst::Object;

    match fn {
        type_ => || ffi::gst_gl_display_wayland_get_type(),
    }
}

impl GLDisplayWayland {
    #[doc(alias = "gst_gl_display_wayland_new")]
    pub fn new(name: Option<&str>) -> GLDisplayWayland {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_gl_display_wayland_new(name.to_glib_none().0)) }
    }

    //#[doc(alias = "gst_gl_display_wayland_new_with_display")]
    //#[doc(alias = "new_with_display")]
    //pub fn with_display(display: /*Unimplemented*/Option<Fundamental: Pointer>) -> GLDisplayWayland {
    //    unsafe { TODO: call ffi:gst_gl_display_wayland_new_with_display() }
    //}
}

unsafe impl Send for GLDisplayWayland {}
unsafe impl Sync for GLDisplayWayland {}

pub const NONE_GL_DISPLAY_WAYLAND: Option<&GLDisplayWayland> = None;
