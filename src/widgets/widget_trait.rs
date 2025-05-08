use cairo::Context;
use input_linux::{EventKind, Key, SynchronizeKind, UInputHandle};
use std::{os::fd::AsRawFd, time::Instant};

use crate::emit;

pub trait TWidget {
    fn render(
        &mut self,
        c: &Context,
        height: i32,
        button_left_edge: f64,
        button_width: u64,
        y_shift: f64,
    );
    // If widget wants periodic redraw, instant of next draw request
    fn next_draw_time(&self) -> Option<Instant>;
    // Used for active / key up-down events
    fn set_active(&mut self, active: bool) -> bool;
    fn get_action(&self) -> Key;
    fn changed(&self) -> bool;
    fn active(&self) -> bool;
    fn reset_changed(&mut self);
}

pub fn set_widget_active<F>(
    widget: &mut Box<dyn TWidget>,
    uinput: &mut UInputHandle<F>,
    active: bool,
) where
    F: AsRawFd,
{
    if widget.set_active(active) {
        //Active changed
        toggle_key(uinput, widget.get_action(), active as i32);
    }
}

fn toggle_key<F>(uinput: &mut UInputHandle<F>, code: Key, value: i32)
where
    F: AsRawFd,
{
    emit(uinput, EventKind::Key, code as u16, value);
    emit(
        uinput,
        EventKind::Synchronize,
        SynchronizeKind::Report as u16,
        0,
    );
}
