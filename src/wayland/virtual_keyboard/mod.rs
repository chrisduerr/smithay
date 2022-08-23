//! TODO

use wayland_backend::server::GlobalId;
use wayland_protocols_misc::zwp_virtual_keyboard_v1::server::{
    zwp_virtual_keyboard_manager_v1::{self, ZwpVirtualKeyboardManagerV1},
    zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
};
use wayland_server::{Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New};

use crate::input::SeatHandler;
use crate::wayland::virtual_keyboard::virtual_keyboard_handle::VirtualKeyboardUserData;

const MANAGER_VERSION: u32 = 1;

pub mod virtual_keyboard_handle;

/// State of wp misc virtual keyboard protocol
#[derive(Debug)]
pub struct VirtualKeyboardManagerState {
    global: GlobalId,
}

impl VirtualKeyboardManagerState {
    /// Initialize a virtual keyboard manager global.
    pub fn new<D>(display: &DisplayHandle) -> Self
    where
        D: GlobalDispatch<ZwpVirtualKeyboardManagerV1, ()>,
        D: Dispatch<ZwpVirtualKeyboardManagerV1, ()>,
        D: Dispatch<ZwpVirtualKeyboardV1, VirtualKeyboardUserData<D>>,
        D: SeatHandler,
        D: 'static,
    {
        let global = display.create_global::<D, ZwpVirtualKeyboardManagerV1, _>(MANAGER_VERSION, ());

        Self { global }
    }

    /// Get the id of the ZwpVirtualKeyboardManagerV1 global.
    pub fn global(&self) -> GlobalId {
        self.global.clone()
    }
}

impl<D> GlobalDispatch<ZwpVirtualKeyboardManagerV1, (), D> for VirtualKeyboardManagerState
where
    D: GlobalDispatch<ZwpVirtualKeyboardManagerV1, ()>,
    D: Dispatch<ZwpVirtualKeyboardManagerV1, ()>,
    D: Dispatch<ZwpVirtualKeyboardV1, VirtualKeyboardUserData<D>>,
    D: SeatHandler,
    D: 'static,
{
    fn bind(
        _: &mut D,
        _: &DisplayHandle,
        _: &Client,
        resource: New<ZwpVirtualKeyboardManagerV1>,
        _: &(),
        data_init: &mut DataInit<'_, D>,
    ) {
        data_init.init(resource, ());
    }
}

impl<D> Dispatch<ZwpVirtualKeyboardManagerV1, (), D> for VirtualKeyboardManagerState
where
    D: Dispatch<ZwpVirtualKeyboardManagerV1, ()>,
    D: Dispatch<ZwpVirtualKeyboardV1, VirtualKeyboardUserData<D>>,
    D: SeatHandler,
    D: 'static,
{
    fn request(
        _state: &mut D,
        _client: &Client,
        _resource: &ZwpVirtualKeyboardManagerV1,
        request: zwp_virtual_keyboard_manager_v1::Request,
        _data: &(),
        _dhandle: &DisplayHandle,
        data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            zwp_virtual_keyboard_manager_v1::Request::CreateVirtualKeyboard { seat, id } => {
                data_init.init(id, VirtualKeyboardUserData::new(&seat));
            }
            _ => unreachable!(),
        }
    }
}

#[allow(missing_docs)] //TODO
#[macro_export]
macro_rules! delegate_virtual_keyboard_manager {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols_misc::zwp_virtual_keyboard_v1::server::zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1: ()
        ] => $crate::wayland::virtual_keyboard::VirtualKeyboardManagerState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols_misc::zwp_virtual_keyboard_v1::server::zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1: ()
        ] => $crate::wayland::virtual_keyboard::VirtualKeyboardManagerState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols_misc::zwp_virtual_keyboard_v1::server::zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1: $crate::wayland::virtual_keyboard::virtual_keyboard_handle::VirtualKeyboardUserData<$ty>
        ] => $crate::wayland::virtual_keyboard::VirtualKeyboardManagerState);
    };
}
