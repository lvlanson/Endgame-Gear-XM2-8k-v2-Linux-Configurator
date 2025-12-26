use std::time::Duration;

use crate::Profile;
use rusb::{self, Context, DeviceHandle, UsbContext};
/// Header for SET_REPORT requests:
/// predefines all the headerfields for the usb interface call
/// over the handler
struct SetHeader;
impl SetHeader {
    const bmRequestType: u8 = 0x21;
    const bRequest: u8 = 0x09;
    const wValue: u16 = 0x03a1;
    const wIndex: u16 = 0x0001;
}

/// Header for GET_REPORT requests:
/// predefines all the headerfields for the usb interface vall
/// over the handler
struct GetHeader;
impl GetHeader {
    const bmRequestType: u8 = 0xa1;
    const bRequest: u8 = 0x01;
    const wValue: u16 = 0x03a1;
    const wIndex: u16 = 0x0001;
}

/// Handler object for the connection to the mouse
/// provides functions for handling the connection such as
/// - init: establishing the connection and initializing handle
/// - read_profile: reading a profile from the device and return it
/// - write_profile: write a profile to the device
/// handles closing connections automatically on destruction of the
/// object
///
/// * `handle`: handle to the usb device
pub struct Handler {
    handle: DeviceHandle<Context>,
}
impl Handler {
    const VID: u16 = 0x3367;
    const PID: u16 = 0x1980;
    const INTERFACE: u8 = 0x01;

    pub fn init() -> Self {
        //  creating USB device context
        let context: Context = Context::new().expect("failed to create context");

        // creating a device handle for usb device
        let device_handle: DeviceHandle<Context> = context
            .open_device_with_vid_pid(Self::VID, Self::PID)
            .expect("failed to open device");

        // if kernel driver is active detach it
        if device_handle
            .kernel_driver_active(Self::INTERFACE)
            .expect("failed to determine if kernel driver is active")
        {
            device_handle
                .detach_kernel_driver(Self::INTERFACE)
                .expect("failed to detach kernel driver");
        }

        // claim interface
        if device_handle.claim_interface(Self::INTERFACE).is_err() {
            device_handle
                .detach_kernel_driver(Self::INTERFACE)
                .expect("failed to detach kernel driver claiming device");
            panic!("failed claiming device");
        }
        Self {
            handle: device_handle,
        }
    }

    pub fn read_profile(&self) -> Profile {
        // initializing empty profile
        let mut profile: Profile = Profile::init();

        // reading profile to `profile`
        self.handle
            .read_control(
                GetHeader::bmRequestType,
                GetHeader::bRequest,
                GetHeader::wValue,
                GetHeader::wIndex,
                profile.profile_buf.as_mut_slice(),
                Duration::new(1000, 0),
            )
            .expect("failed reading profile");
        // returning profile
        profile
    }

    fn write_profile(&self) {
        // to do
    }
}

/// destructor closing down open connections on free
impl Drop for Handler {
    fn drop(&mut self) {
        // releasing interface
        self.handle
            .release_interface(self::Handler::INTERFACE)
            .expect("failed to release interface");
        // reattaching kernel
        self.handle
            .attach_kernel_driver(self::Handler::INTERFACE)
            .expect("failed to attach to kernel driver when closing");
    }
}
