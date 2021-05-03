// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::GLContext;
use crate::GLDisplay;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    pub struct GLWindow(Object<ffi::GstGLWindow, ffi::GstGLWindowClass>) @extends gst::Object;

    match fn {
        type_ => || ffi::gst_gl_window_get_type(),
    }
}

impl GLWindow {
    #[doc(alias = "gst_gl_window_new")]
    pub fn new<P: IsA<GLDisplay>>(display: &P) -> GLWindow {
        skip_assert_initialized!();
        unsafe { from_glib_full(ffi::gst_gl_window_new(display.as_ref().to_glib_none().0)) }
    }
}

unsafe impl Send for GLWindow {}
unsafe impl Sync for GLWindow {}

pub const NONE_GL_WINDOW: Option<&GLWindow> = None;

pub trait GLWindowExt: 'static {
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_gl_window_controls_viewport")]
    fn controls_viewport(&self) -> bool;

    #[doc(alias = "gst_gl_window_draw")]
    fn draw(&self);

    #[doc(alias = "gst_gl_window_get_context")]
    #[doc(alias = "get_context")]
    fn context(&self) -> Option<GLContext>;

    #[doc(alias = "gst_gl_window_get_surface_dimensions")]
    #[doc(alias = "get_surface_dimensions")]
    fn surface_dimensions(&self) -> (u32, u32);

    #[doc(alias = "gst_gl_window_handle_events")]
    fn handle_events(&self, handle_events: bool);

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_gl_window_has_output_surface")]
    fn has_output_surface(&self) -> bool;

    #[doc(alias = "gst_gl_window_queue_resize")]
    fn queue_resize(&self);

    #[doc(alias = "gst_gl_window_quit")]
    fn quit(&self);

    #[doc(alias = "gst_gl_window_resize")]
    fn resize(&self, width: u32, height: u32);

    #[doc(alias = "gst_gl_window_run")]
    fn run(&self);

    #[doc(alias = "gst_gl_window_send_key_event")]
    fn send_key_event(&self, event_type: &str, key_str: &str);

    #[doc(alias = "gst_gl_window_send_mouse_event")]
    fn send_mouse_event(&self, event_type: &str, button: i32, posx: f64, posy: f64);

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_gl_window_send_scroll_event")]
    fn send_scroll_event(&self, posx: f64, posy: f64, delta_x: f64, delta_y: f64);

    #[doc(alias = "gst_gl_window_set_preferred_size")]
    fn set_preferred_size(&self, width: i32, height: i32);

    #[doc(alias = "gst_gl_window_set_render_rectangle")]
    fn set_render_rectangle(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_gl_window_show")]
    fn show(&self);

    #[doc(alias = "key-event")]
    fn connect_key_event<F: Fn(&Self, &str, &str) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "mouse-event")]
    fn connect_mouse_event<F: Fn(&Self, &str, i32, f64, f64) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "scroll-event")]
    fn connect_scroll_event<F: Fn(&Self, f64, f64, f64, f64) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<GLWindow>> GLWindowExt for O {
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn controls_viewport(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_window_controls_viewport(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn draw(&self) {
        unsafe {
            ffi::gst_gl_window_draw(self.as_ref().to_glib_none().0);
        }
    }

    fn context(&self) -> Option<GLContext> {
        unsafe {
            from_glib_full(ffi::gst_gl_window_get_context(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn surface_dimensions(&self) -> (u32, u32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::gst_gl_window_get_surface_dimensions(
                self.as_ref().to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            let width = width.assume_init();
            let height = height.assume_init();
            (width, height)
        }
    }

    fn handle_events(&self, handle_events: bool) {
        unsafe {
            ffi::gst_gl_window_handle_events(
                self.as_ref().to_glib_none().0,
                handle_events.into_glib(),
            );
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn has_output_surface(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_window_has_output_surface(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn queue_resize(&self) {
        unsafe {
            ffi::gst_gl_window_queue_resize(self.as_ref().to_glib_none().0);
        }
    }

    fn quit(&self) {
        unsafe {
            ffi::gst_gl_window_quit(self.as_ref().to_glib_none().0);
        }
    }

    fn resize(&self, width: u32, height: u32) {
        unsafe {
            ffi::gst_gl_window_resize(self.as_ref().to_glib_none().0, width, height);
        }
    }

    fn run(&self) {
        unsafe {
            ffi::gst_gl_window_run(self.as_ref().to_glib_none().0);
        }
    }

    fn send_key_event(&self, event_type: &str, key_str: &str) {
        unsafe {
            ffi::gst_gl_window_send_key_event(
                self.as_ref().to_glib_none().0,
                event_type.to_glib_none().0,
                key_str.to_glib_none().0,
            );
        }
    }

    fn send_mouse_event(&self, event_type: &str, button: i32, posx: f64, posy: f64) {
        unsafe {
            ffi::gst_gl_window_send_mouse_event(
                self.as_ref().to_glib_none().0,
                event_type.to_glib_none().0,
                button,
                posx,
                posy,
            );
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn send_scroll_event(&self, posx: f64, posy: f64, delta_x: f64, delta_y: f64) {
        unsafe {
            ffi::gst_gl_window_send_scroll_event(
                self.as_ref().to_glib_none().0,
                posx,
                posy,
                delta_x,
                delta_y,
            );
        }
    }

    fn set_preferred_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gst_gl_window_set_preferred_size(self.as_ref().to_glib_none().0, width, height);
        }
    }

    fn set_render_rectangle(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_gl_window_set_render_rectangle(
                    self.as_ref().to_glib_none().0,
                    x,
                    y,
                    width,
                    height
                ),
                "Failed to set the specified region"
            )
        }
    }

    fn show(&self) {
        unsafe {
            ffi::gst_gl_window_show(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "key-event")]
    fn connect_key_event<F: Fn(&Self, &str, &str) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn key_event_trampoline<
            P,
            F: Fn(&P, &str, &str) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLWindow,
            id: *mut libc::c_char,
            key: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<GLWindow>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GLWindow::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(id),
                &glib::GString::from_glib_borrow(key),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"key-event\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    key_event_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mouse-event")]
    fn connect_mouse_event<F: Fn(&Self, &str, i32, f64, f64) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn mouse_event_trampoline<
            P,
            F: Fn(&P, &str, i32, f64, f64) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLWindow,
            id: *mut libc::c_char,
            button: libc::c_int,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<GLWindow>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GLWindow::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(id),
                button,
                x,
                y,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"mouse-event\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    mouse_event_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "scroll-event")]
    fn connect_scroll_event<F: Fn(&Self, f64, f64, f64, f64) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn scroll_event_trampoline<
            P,
            F: Fn(&P, f64, f64, f64, f64) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLWindow,
            x: libc::c_double,
            y: libc::c_double,
            delta_x: libc::c_double,
            delta_y: libc::c_double,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<GLWindow>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GLWindow::from_glib_borrow(this).unsafe_cast_ref(),
                x,
                y,
                delta_x,
                delta_y,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"scroll-event\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    scroll_event_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
