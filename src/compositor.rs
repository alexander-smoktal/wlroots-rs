//! Main entry point to the library.
//! See examples for documentation on how to use this struct.

use std::any::Any;
use std::cell::UnsafeCell;
use std::env;
use std::ffi::CStr;

use extensions::server_decoration::ServerDecorationManager;
use manager::{InputManager, InputManagerHandler, OutputManager, OutputManagerHandler};
use render::GLES2;

use wayland_sys::server::{WAYLAND_SERVER_HANDLE, wl_display, wl_event_loop};
use wayland_sys::server::signal::wl_signal_add;
use wlroots_sys::{wlr_backend, wlr_backend_autocreate, wlr_backend_destroy, wlr_backend_start};

/// Global compositor pointer, used to refer to the compositor state unsafely.
pub static mut COMPOSITOR_PTR: *mut Compositor = 0 as *mut _;

pub struct CompositorBuilder {
    gles2: bool,
    server_decoration_manager: bool
}

impl CompositorBuilder {
    pub fn new() -> Self {
        CompositorBuilder {
            gles2: false,
            server_decoration_manager: false
        }
    }

    pub fn gles2(mut self, gles2_renderer: bool) -> Self {
        self.gles2 = gles2_renderer;
        self
    }

    pub fn server_decoration_manager(mut self, server_decoration_manager: bool) -> Self {
        self.server_decoration_manager = server_decoration_manager;
        self
    }

    /// Makes a new compositor that handles the setup of the graphical backend
    /// (e.g, Wayland, X11, or DRM).
    ///
    /// Also automatically opens the socket for clients to communicate to the
    /// compositor with.
    pub fn build_auto<T: Any + 'static>(self,
                                        data: T,
                                        input_manager_handler: Box<InputManagerHandler>,
                                        output_manager_handler: Box<OutputManagerHandler>)
                                        -> Compositor {
        unsafe {
            let display = ffi_dispatch!(WAYLAND_SERVER_HANDLE, wl_display_create,) as
                *mut wl_display;
            let event_loop =
                ffi_dispatch!(WAYLAND_SERVER_HANDLE, wl_display_get_event_loop, display);
            let backend = wlr_backend_autocreate(display as *mut _);
            if backend.is_null() {
                // NOTE Rationale for panicking:
                // * Won't be in C land just yet, so it's safe to panic
                // * Can always be returned in a Result instead, but for now
                //   if you auto create it's assumed you can't recover.
                panic!("Could not auto-create backend");
            }
            let mut input_manager = InputManager::new((vec![], input_manager_handler));
            let mut output_manager = OutputManager::new((vec![], output_manager_handler));
            wl_signal_add(&mut (*backend).events.input_add as *mut _ as _,
                          input_manager.add_listener() as *mut _ as _);
            wl_signal_add(&mut (*backend).events.input_remove as *mut _ as _,
                          input_manager.remove_listener() as *mut _ as _);
            wl_signal_add(&mut (*backend).events.output_add as *mut _ as _,
                          output_manager.add_listener() as *mut _ as _);
            wl_signal_add(&mut (*backend).events.output_remove as *mut _ as _,
                          output_manager.remove_listener() as *mut _ as _);

            let server_decoration_manager = if self.server_decoration_manager {
                ServerDecorationManager::new(display)
            } else {
                None
            };
            let gles2 = if self.gles2 {
                GLES2::new(backend)
            } else {
                None
            };

            let socket = ffi_dispatch!(WAYLAND_SERVER_HANDLE, wl_display_add_socket_auto, display);
            if socket.is_null() {
                // NOTE Rationale for panicking:
                // * Won't be in C land just yet, so it's safe to panic
                // * Can always be returned in a Result instead, but for now
                //   if you auto create it's assumed you can't recover.
                panic!("Unable to open wayland socket");
            }
            let socket_name = CStr::from_ptr(socket).to_string_lossy().into_owned();
            wlr_log!(L_DEBUG,
                     "Running compositor on wayland display {}",
                     socket_name);
            env::set_var("_WAYLAND_DISPLAY", socket_name);
            Compositor {
                data: Box::new(data),
                input_manager,
                output_manager,
                backend,
                display,
                event_loop,
                server_decoration_manager,
                gles2
            }
        }
    }
}

#[allow(dead_code)]
pub struct Compositor {
    pub data: Box<Any>,
    input_manager: Box<InputManager>,
    output_manager: Box<OutputManager>,
    backend: *mut wlr_backend,
    display: *mut wl_display,
    event_loop: *mut wl_event_loop,
    pub server_decoration_manager: Option<ServerDecorationManager>,
    pub gles2: Option<GLES2>
}

impl Compositor {
    /// Enters the wayland event loop. Won't return until the compositor is
    /// shut off
    pub fn run(self) {
        unsafe {
            let compositor = UnsafeCell::new(self);
            if COMPOSITOR_PTR != 0 as _ {
                // NOTE Rationale for panicking:
                // * Nicer than an abort
                // * Not yet in C land
                panic!("A compositor is already running!")
            }
            COMPOSITOR_PTR = compositor.get();
            wlr_log!(L_INFO, "Starting compositor");
            if !wlr_backend_start((*compositor.get()).backend) {
                wlr_backend_destroy((*compositor.get()).backend);
                // NOTE Rationale for panicking:
                // * Won't be in C land just yet, so it's safe to panic
                // * Can always be returned in a Result instead, but for now
                //   if you auto create it's assumed you can't recover.
                panic!("Failed to start backend");
            }
            ffi_dispatch!(WAYLAND_SERVER_HANDLE,
                          wl_display_run,
                          (*compositor.get()).display);
        }
        // TODO Clean up
    }

    pub fn terminate(&mut self) {
        unsafe {
            ffi_dispatch!(WAYLAND_SERVER_HANDLE, wl_display_terminate, self.display);
        }
    }
}

/// Terminates the compositor.
/// If one is not running, does nothing
pub fn terminate() {
    unsafe {
        if COMPOSITOR_PTR != 0 as _ {
            (*COMPOSITOR_PTR).terminate();
            COMPOSITOR_PTR = 0 as _
        }
    }
}
