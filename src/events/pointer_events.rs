//! Pointers and their events

use types::input_device::InputDevice;

use wlroots_sys::{wlr_button_state, wlr_event_pointer_axis, wlr_event_pointer_button,
                  wlr_event_pointer_motion, wlr_event_pointer_motion_absolute};

pub struct AxisEvent {
    event: *mut wlr_event_pointer_axis
}

pub struct ButtonEvent {
    event: *mut wlr_event_pointer_button
}

pub struct MotionEvent {
    event: *mut wlr_event_pointer_motion
}

pub struct AbsoluteMotionEvent {
    event: *mut wlr_event_pointer_motion_absolute
}

impl ButtonEvent {
    pub unsafe fn from_ptr(event: *mut wlr_event_pointer_button) -> Self {
        ButtonEvent { event }
    }

    pub fn state(&self) -> wlr_button_state {
        unsafe { (*self.event).state }
    }

    pub fn button(&self) -> u32 {
        unsafe { (*self.event).button }
    }
}

impl AxisEvent {
    pub unsafe fn from_ptr(event: *mut wlr_event_pointer_axis) -> Self {
        AxisEvent { event }
    }

    pub fn delta(&self) -> f64 {
        unsafe { (*self.event).delta }
    }
}

impl MotionEvent {
    pub unsafe fn from_ptr(event: *mut wlr_event_pointer_motion) -> Self {
        MotionEvent { event }
    }

    pub fn device(&self) -> InputDevice {
        unsafe { InputDevice::from_ptr((*self.event).device) }
    }

    pub fn delta(&self) -> (f64, f64) {
        unsafe { ((*self.event).delta_x, (*self.event).delta_y) }
    }
}

impl AbsoluteMotionEvent {
    pub unsafe fn from_ptr(event: *mut wlr_event_pointer_motion_absolute) -> Self {
        AbsoluteMotionEvent { event }
    }

    pub fn device(&self) -> InputDevice {
        unsafe { InputDevice::from_ptr((*self.event).device) }
    }
}
