//! Handler for pointers

use libc;

use compositor::{COMPOSITOR_PTR, Compositor};
use events::pointer_events::{AbsoluteMotionEvent, AxisEvent, ButtonEvent, MotionEvent};
use types::PointerHandle;

use wlroots_sys::{wlr_event_pointer_axis, wlr_event_pointer_button, wlr_event_pointer_motion,
                  wlr_input_device};

pub trait PointerHandler {
    /// Callback that is triggered when the pointer moves.
    fn on_motion(&mut self, &mut Compositor, &mut PointerHandle, &MotionEvent) {}

    fn on_motion_absolute(&mut self, &mut Compositor, &mut PointerHandle, &AbsoluteMotionEvent) {}

    /// Callback that is triggered when the buttons on the pointer are pressed.
    fn on_button(&mut self, &mut Compositor, &mut PointerHandle, &ButtonEvent) {}

    fn on_axis(&mut self, &mut Compositor, &mut PointerHandle, &AxisEvent) {}
}

wayland_listener!(PointerWrapper, (PointerHandle, Box<PointerHandler>), [
    button_listener => key_notify: |this: &mut PointerWrapper, data: *mut libc::c_void,| unsafe {
        let event = ButtonEvent::from_ptr(data as *mut wlr_event_pointer_button);
        let compositor = &mut *COMPOSITOR_PTR;
        this.data.1.on_button(compositor, &mut this.data.0, &event)
    };
    motion_listener => motion_notify:  |this: &mut PointerWrapper, data: *mut libc::c_void,|
    unsafe {
        let event = MotionEvent::from_ptr(data as *mut wlr_event_pointer_motion);
        let compositor = &mut *COMPOSITOR_PTR;
        this.data.1.on_motion(compositor, &mut this.data.0, &event)
    };
    motion_absolute_listener => motion_absolute_notify:
    |this: &mut PointerWrapper, data: *mut libc::c_void,| unsafe {
        let event = AbsoluteMotionEvent::from_ptr(data as *mut _);
        let compositor = &mut *COMPOSITOR_PTR;
        this.data.1.on_motion_absolute(compositor, &mut this.data.0, &event)
    };
    axis_listener => axis_notify:  |this: &mut PointerWrapper, data: *mut libc::c_void,| unsafe {
        let event = AxisEvent::from_ptr(data as *mut wlr_event_pointer_axis);
        let compositor = &mut *COMPOSITOR_PTR;
        this.data.1.on_axis(compositor, &mut this.data.0, &event)
    };
]);

impl PointerWrapper {
    pub unsafe fn input_device(&self) -> *mut wlr_input_device {
        self.data.0.input_device()
    }
}
