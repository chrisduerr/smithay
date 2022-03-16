//! TODO: Doc

use std::cell::RefCell;
use std::rc::Rc;

use wayland_server::protocol::wl_keyboard::KeymapFormat;
use wayland_server::protocol::wl_seat::WlSeat;
use wayland_server::protocol::wl_surface::WlSurface;
use wayland_server::{Client, Display, Filter, Global, Main};

use crate::backend::input::KeyState;
use crate::wayland::seat::{GrabStartData, KeyboardGrab, KeyboardHandle, Seat, XkbConfig};
use crate::wayland::virtual_keyboard::api::zwp_virtual_keyboard_manager_v1::{
    Request as ManagerRequest, ZwpVirtualKeyboardManagerV1,
};
use crate::wayland::virtual_keyboard::api::zwp_virtual_keyboard_v1::{
    Request as KeyboardRequest, ZwpVirtualKeyboardV1,
};
use crate::wayland::{Serial, SERIAL_COUNTER};

mod api;

const MANAGER_VERSION: u32 = 1;

type VirtualKeyboard = Rc<RefCell<Option<KeyboardHandle>>>;

/// TODO: Doc
#[derive(Debug)]
pub struct VirtualKeyboardHandle {
    /// TODO: Doc
    pub global: Global<ZwpVirtualKeyboardManagerV1>,
    keyboard: VirtualKeyboard,
}

impl VirtualKeyboardHandle {
    /// TODO: Doc
    /// TODO: XkbConfig
    pub fn new(display: &mut Display, repeat_delay: i32, repeat_rate: i32) -> Self {
        let keyboard = Rc::new(RefCell::new(None));

        let global_keyboard = keyboard.clone();
        let global = display.create_global::<ZwpVirtualKeyboardManagerV1, _>(
            MANAGER_VERSION,
            Filter::new(
                move |(manager, _version): (Main<ZwpVirtualKeyboardManagerV1>, u32), _, _| {
                    let global_keyboard = global_keyboard.clone();
                    manager.quick_assign(move |_manager, req, _| {
                        let ManagerRequest::CreateVirtualKeyboard { seat, id } = req;
                        Self::create_virtual_keyboard(
                            global_keyboard.clone(),
                            repeat_delay,
                            repeat_rate,
                            seat,
                            id,
                        );
                    })
                },
            ),
        );

        VirtualKeyboardHandle { global, keyboard }
    }

    /// Change the current grab on this keyboard to the provided grab
    ///
    /// Overwrites any current grab.
    pub fn set_grab<G: KeyboardGrab + 'static>(&self, grab: G, serial: Serial) {
        if let Some(keyboard) = &*self.keyboard.borrow() {
            keyboard.set_grab(grab, serial);
        }
    }

    /// Remove any current grab on this keyboard, resetting it to the default behavior
    pub fn unset_grab(&self) {
        if let Some(keyboard) = &*self.keyboard.borrow() {
            keyboard.unset_grab();
        }
    }

    /// Check if this keyboard is currently grabbed with this serial
    pub fn has_grab(&self, serial: Serial) -> bool {
        match &*self.keyboard.borrow() {
            Some(keyboard) => keyboard.has_grab(serial),
            _ => false,
        }
    }

    /// Check if this keyboard is currently being grabbed
    pub fn is_grabbed(&self) -> bool {
        match &*self.keyboard.borrow() {
            Some(keyboard) => keyboard.is_grabbed(),
            _ => false,
        }
    }

    /// Returns the start data for the grab, if any.
    pub fn grab_start_data(&self) -> Option<GrabStartData> {
        self.keyboard
            .borrow()
            .as_ref()
            .and_then(|keyboard| keyboard.grab_start_data())
    }

    /// Set the current focus of this keyboard
    ///
    /// If the new focus is different from the previous one, any previous focus
    /// will be sent a [`wl_keyboard::Event::Leave`](wayland_server::protocol::wl_keyboard::Event::Leave)
    /// event, and if the new focus is not `None`,
    /// a [`wl_keyboard::Event::Enter`](wayland_server::protocol::wl_keyboard::Event::Enter) event will be sent.
    pub fn set_focus(&self, focus: Option<&WlSurface>, serial: Serial) {
        if let Some(keyboard) = &*self.keyboard.borrow() {
            keyboard.set_focus(focus, serial);
        }
    }

    /// Check if given client currently has keyboard focus
    pub fn has_focus(&self, client: &Client) -> bool {
        match &*self.keyboard.borrow() {
            Some(keyboard) => keyboard.has_focus(client),
            _ => false,
        }
    }

    /// Check if keyboard has focus
    pub fn is_focused(&self) -> bool {
        match &*self.keyboard.borrow() {
            Some(keyboard) => keyboard.is_focused(),
            _ => false,
        }
    }

    /// Change the repeat info configured for this keyboard
    pub fn change_repeat_info(&self, rate: i32, delay: i32) {
        if let Some(keyboard) = &*self.keyboard.borrow() {
            keyboard.change_repeat_info(rate, delay);
        }
    }

    /// Create a new virtual keyboard from its seat and ID.
    fn create_virtual_keyboard(
        keyboard: VirtualKeyboard,
        repeat_delay: i32,
        repeat_rate: i32,
        seat: WlSeat,
        id: Main<ZwpVirtualKeyboardV1>,
    ) {
        let mut seat = Seat::from_resource(&seat).unwrap();
        let keyboard_handle = seat
            .add_keyboard(
                XkbConfig::default(),
                repeat_delay,
                repeat_rate,
                |_seat, _focused_surface| {
                    // TODO: Focus
                },
            )
            .expect("adding keyboard");

        keyboard.replace(Some(keyboard_handle.clone()));

        // TODO
        id.quick_assign(move |_text_input, req, _| match req {
            KeyboardRequest::Key { time, key, state } => {
                let state = KeyState::from(state);
                let serial = SERIAL_COUNTER.next_serial().into();
                let keyboard = keyboard_handle.arc.internal.borrow();
                keyboard.with_focused_kbds(|keyboard, _| {
                    keyboard.key(serial, time, key, state.into());
                });
            }
            KeyboardRequest::Modifiers {
                mods_depressed,
                mods_latched,
                mods_locked,
                group,
            } => {
                let serial = SERIAL_COUNTER.next_serial().into();
                let keyboard = keyboard_handle.arc.internal.borrow();
                keyboard.with_focused_kbds(|keyboard, _| {
                    keyboard.modifiers(serial, mods_depressed, mods_latched, mods_locked, group);
                });
            }
            KeyboardRequest::Keymap { format, fd, size } => {
                let format = if format == 0 {
                    KeymapFormat::NoKeymap
                } else {
                    KeymapFormat::XkbV1
                };
                let keyboard = keyboard_handle.arc.internal.borrow();
                keyboard.with_focused_kbds(|keyboard, _| {
                    keyboard.keymap(format, fd, size);
                });
            }
            _ => (),
        });
    }
}
