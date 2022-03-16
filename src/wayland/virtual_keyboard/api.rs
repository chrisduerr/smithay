#![allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables)]
#![allow(non_upper_case_globals, non_snake_case, unused_imports)]
#![allow(clippy::all)]
use std::os::raw::{c_char, c_void};
use wayland_commons::map::*;
use wayland_commons::wire::*;
use wayland_server::protocol::*;
use wayland_server::*;
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 4] = [
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
];
#[doc = "virtual keyboard\n\nThe virtual keyboard provides an application with requests which emulate\nthe behaviour of a physical keyboard.\n\nThis interface can be used by clients on its own to provide raw input\nevents, or it can accompany the input method protocol."]
pub mod zwp_virtual_keyboard_v1 {
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::sys::server::*;
    use super::{
        types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message, MessageDesc,
        MessageGroup, Object, ObjectMetadata, Resource, NULLPTR,
    };
    use std::os::raw::c_char;
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum Error {
        #[doc = "No keymap was set"]
        NoKeymap = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::NoKeymap),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Request {
        #[doc = "keyboard mapping\n\nProvide a file descriptor to the compositor which can be\nmemory-mapped to provide a keyboard mapping description.\n\nFormat carries a value from the keymap_format enumeration."]
        Keymap {
            format: u32,
            fd: ::std::os::unix::io::RawFd,
            size: u32,
        },
        #[doc = "key event\n\nA key was pressed or released.\nThe time argument is a timestamp with millisecond granularity, with an\nundefined base. All requests regarding a single object must share the\nsame clock.\n\nKeymap must be set before issuing this request.\n\nState carries a value from the key_state enumeration."]
        Key { time: u32, key: u32, state: u32 },
        #[doc = "modifier and group state\n\nNotifies the compositor that the modifier and/or group state has\nchanged, and it should update state.\n\nThe client should use wl_keyboard.modifiers event to synchronize its\ninternal state with seat state.\n\nKeymap must be set before issuing this request."]
        Modifiers {
            mods_depressed: u32,
            mods_latched: u32,
            mods_locked: u32,
            group: u32,
        },
        #[doc = "destroy the virtual keyboard keyboard object\n\n\n\nThis is a destructor, once received this object cannot be used any longer."]
        Destroy,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "keymap",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Fd,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "key",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "modifiers",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[],
                destructor: true,
            },
        ];
        type Map = super::ResourceMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::Keymap { .. } => 0,
                Request::Key { .. } => 1,
                Request::Modifiers { .. } => 2,
                Request::Destroy => 3,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Keymap { .. } => 1,
                Request::Key { .. } => 1,
                Request::Modifiers { .. } => 1,
                Request::Destroy => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(opcode: u16, version: u32, meta: &Meta) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Request::Keymap {
                        format: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        fd: {
                            if let Some(Argument::Fd(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        size: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Request::Key {
                        time: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        key: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        state: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Request::Modifiers {
                        mods_depressed: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        mods_latched: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        mods_locked: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        group: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                3 => Ok(Request::Destroy),
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Request::into_raw can not be used Server-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Request::Keymap {
                        format: _args[0].u,
                        fd: _args[1].h,
                        size: _args[2].u,
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Request::Key {
                        time: _args[0].u,
                        key: _args[1].u,
                        state: _args[2].u,
                    })
                }
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 4);
                    Ok(Request::Modifiers {
                        mods_depressed: _args[0].u,
                        mods_latched: _args[1].u,
                        mods_locked: _args[2].u,
                        group: _args[3].u,
                    })
                }
                3 => Ok(Request::Destroy),
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Request::as_raw_c_in can not be used Server-side.")
        }
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Event {}
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[];
        type Map = super::ResourceMap;
        fn is_destructor(&self) -> bool {
            match *self {}
        }
        fn opcode(&self) -> u16 {
            match *self {}
        }
        fn since(&self) -> u32 {
            match *self {}
        }
        fn child<Meta: ObjectMetadata>(opcode: u16, version: u32, meta: &Meta) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Event::from_raw can not be used Server-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {}
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            panic!("Event::from_raw_c can not be used Server-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {}
        }
    }
    #[derive(Clone, Eq, PartialEq)]
    pub struct ZwpVirtualKeyboardV1(Resource<ZwpVirtualKeyboardV1>);
    impl AsRef<Resource<ZwpVirtualKeyboardV1>> for ZwpVirtualKeyboardV1 {
        #[inline]
        fn as_ref(&self) -> &Resource<Self> {
            &self.0
        }
    }
    impl From<Resource<ZwpVirtualKeyboardV1>> for ZwpVirtualKeyboardV1 {
        #[inline]
        fn from(value: Resource<Self>) -> Self {
            ZwpVirtualKeyboardV1(value)
        }
    }
    impl From<ZwpVirtualKeyboardV1> for Resource<ZwpVirtualKeyboardV1> {
        #[inline]
        fn from(value: ZwpVirtualKeyboardV1) -> Self {
            value.0
        }
    }
    impl std::fmt::Debug for ZwpVirtualKeyboardV1 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}", self.0))
        }
    }
    impl Interface for ZwpVirtualKeyboardV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_virtual_keyboard_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_virtual_keyboard_v1_interface }
        }
    }
    impl ZwpVirtualKeyboardV1 {}
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_KEYMAP_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_KEY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_MODIFIERS_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_virtual_keyboard_v1_requests: [wl_message; 4] = [
        wl_message {
            name: b"keymap\0" as *const u8 as *const c_char,
            signature: b"uhu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"key\0" as *const u8 as *const c_char,
            signature: b"uuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"modifiers\0" as *const u8 as *const c_char,
            signature: b"uuuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_virtual_keyboard_v1_interface: wl_interface = wl_interface {
        name: b"zwp_virtual_keyboard_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 4,
        requests: unsafe { &zwp_virtual_keyboard_v1_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "virtual keyboard manager\n\nA virtual keyboard manager allows an application to provide keyboard\ninput events as if they came from a physical keyboard."]
pub mod zwp_virtual_keyboard_manager_v1 {
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::sys::server::*;
    use super::{
        types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message, MessageDesc,
        MessageGroup, Object, ObjectMetadata, Resource, NULLPTR,
    };
    use std::os::raw::c_char;
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum Error {
        #[doc = "client not authorized to use the interface"]
        Unauthorized = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::Unauthorized),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Request {
        #[doc = "Create a new virtual keyboard\n\nCreates a new virtual keyboard associated to a seat.\n\nIf the compositor enables a keyboard to perform arbitrary actions, it\nshould present an error when an untrusted client requests a new\nkeyboard."]
        CreateVirtualKeyboard {
            seat: super::wl_seat::WlSeat,
            id: Main<super::zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1>,
        },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "create_virtual_keyboard",
            since: 1,
            signature: &[super::ArgumentType::Object, super::ArgumentType::NewId],
            destructor: false,
        }];
        type Map = super::ResourceMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::CreateVirtualKeyboard { .. } => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::CreateVirtualKeyboard { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(opcode: u16, version: u32, meta: &Meta) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<
                    super::zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
                >(version, meta.child())),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Request::CreateVirtualKeyboard {
                        seat: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?.into()
                            } else {
                                return Err(());
                            }
                        },
                        id: {
                            if let Some(Argument::NewId(val)) = args.next() {
                                map.get_new(val).ok_or(())?
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Request::into_raw can not be used Server-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Request::CreateVirtualKeyboard {
                        seat: Resource::<super::wl_seat::WlSeat>::from_c_ptr(_args[0].o as *mut _).into(),
                        id: {
                            let me = Resource::<ZwpVirtualKeyboardManagerV1>::from_c_ptr(obj as *mut _);
                            me.make_child_for::<super::zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1>(
                                _args[1].n,
                            )
                            .unwrap()
                        },
                    })
                }
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Request::as_raw_c_in can not be used Server-side.")
        }
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Event {}
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[];
        type Map = super::ResourceMap;
        fn is_destructor(&self) -> bool {
            match *self {}
        }
        fn opcode(&self) -> u16 {
            match *self {}
        }
        fn since(&self) -> u32 {
            match *self {}
        }
        fn child<Meta: ObjectMetadata>(opcode: u16, version: u32, meta: &Meta) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Event::from_raw can not be used Server-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {}
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            panic!("Event::from_raw_c can not be used Server-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {}
        }
    }
    #[derive(Clone, Eq, PartialEq)]
    pub struct ZwpVirtualKeyboardManagerV1(Resource<ZwpVirtualKeyboardManagerV1>);
    impl AsRef<Resource<ZwpVirtualKeyboardManagerV1>> for ZwpVirtualKeyboardManagerV1 {
        #[inline]
        fn as_ref(&self) -> &Resource<Self> {
            &self.0
        }
    }
    impl From<Resource<ZwpVirtualKeyboardManagerV1>> for ZwpVirtualKeyboardManagerV1 {
        #[inline]
        fn from(value: Resource<Self>) -> Self {
            ZwpVirtualKeyboardManagerV1(value)
        }
    }
    impl From<ZwpVirtualKeyboardManagerV1> for Resource<ZwpVirtualKeyboardManagerV1> {
        #[inline]
        fn from(value: ZwpVirtualKeyboardManagerV1) -> Self {
            value.0
        }
    }
    impl std::fmt::Debug for ZwpVirtualKeyboardManagerV1 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}", self.0))
        }
    }
    impl Interface for ZwpVirtualKeyboardManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_virtual_keyboard_manager_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_virtual_keyboard_manager_v1_interface }
        }
    }
    impl ZwpVirtualKeyboardManagerV1 {}
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_CREATE_VIRTUAL_KEYBOARD_SINCE: u32 = 1u32;
    static mut zwp_virtual_keyboard_manager_v1_requests_create_virtual_keyboard_types: [*const wl_interface;
        2] = [
        unsafe { &super::wl_seat::wl_seat_interface as *const wl_interface },
        unsafe { &super::zwp_virtual_keyboard_v1::zwp_virtual_keyboard_v1_interface as *const wl_interface },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_virtual_keyboard_manager_v1_requests: [wl_message; 1] = [wl_message {
        name: b"create_virtual_keyboard\0" as *const u8 as *const c_char,
        signature: b"on\0" as *const u8 as *const c_char,
        types: unsafe { &zwp_virtual_keyboard_manager_v1_requests_create_virtual_keyboard_types as *const _ },
    }];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_virtual_keyboard_manager_v1_interface: wl_interface = wl_interface {
        name: b"zwp_virtual_keyboard_manager_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 1,
        requests: unsafe { &zwp_virtual_keyboard_manager_v1_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
