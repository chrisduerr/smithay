//! TODO

use wayland_backend::server::{ClientId, ObjectId};
use wayland_protocols_misc::zwp_virtual_keyboard_v1::server::zwp_virtual_keyboard_v1::{
    self, ZwpVirtualKeyboardV1,
};
use wayland_server::protocol::wl_keyboard::{KeyState, KeymapFormat};
use wayland_server::protocol::wl_seat::WlSeat;
use wayland_server::{Client, DataInit, Dispatch, DisplayHandle};

use crate::input::{Seat, SeatHandler};
use crate::utils::SERIAL_COUNTER;
use crate::wayland::seat::keyboard;

use crate::wayland::virtual_keyboard::VirtualKeyboardManagerState;

/// User data of ZwpVirtualKeyboardV1 object
#[derive(Debug)]
pub struct VirtualKeyboardUserData<D: SeatHandler> {
    seat: Seat<D>,
}

impl<D: SeatHandler + 'static> VirtualKeyboardUserData<D> {
    pub(crate) fn new(seat: &WlSeat) -> Self {
        let seat = Seat::<D>::from_resource(&seat).unwrap();
        Self { seat }
    }
}

impl<D> Dispatch<ZwpVirtualKeyboardV1, VirtualKeyboardUserData<D>, D> for VirtualKeyboardManagerState
where
    D: Dispatch<ZwpVirtualKeyboardV1, VirtualKeyboardUserData<D>>,
    D: SeatHandler,
    D: 'static,
{
    fn request(
        _state: &mut D,
        _client: &Client,
        _: &ZwpVirtualKeyboardV1,
        request: zwp_virtual_keyboard_v1::Request,
        data: &VirtualKeyboardUserData<D>,
        _dh: &DisplayHandle,
        _data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            zwp_virtual_keyboard_v1::Request::Keymap { format, fd, size } => {
                let format = if format == 0 {
                    KeymapFormat::NoKeymap
                } else {
                    KeymapFormat::XkbV1
                };

                keyboard::with_focused_kbds(&data.seat, |kbd| {
                    kbd.keymap(format, fd, size);
                });
            }
            zwp_virtual_keyboard_v1::Request::Key { time, key, state } => {
                let state = if state == 1 {
                    KeyState::Pressed
                } else {
                    KeyState::Released
                };

                let serial = SERIAL_COUNTER.next_serial().into();
                keyboard::with_focused_kbds(&data.seat, |kbd| {
                    kbd.key(serial, time, key, state);
                });
            }
            zwp_virtual_keyboard_v1::Request::Modifiers {
                mods_depressed,
                mods_latched,
                mods_locked,
                group,
            } => {
                let serial = SERIAL_COUNTER.next_serial().into();
                keyboard::with_focused_kbds(&data.seat, |kbd| {
                    kbd.modifiers(serial, mods_depressed, mods_latched, mods_locked, group);
                });
            }
            zwp_virtual_keyboard_v1::Request::Destroy => {
                // Nothing to do
            }
            _ => todo!(),
        }
    }

    fn destroyed(
        _state: &mut D,
        _client: ClientId,
        _virtual_keyboard: ObjectId,
        _data: &VirtualKeyboardUserData<D>,
    ) {
    }
}
