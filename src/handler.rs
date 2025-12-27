use std::time::Duration;

use crate::Profile;
use rusb::{self, Context, DeviceHandle, UsbContext};
/// Header for SET_REPORT requests:
/// predefines all the headerfields for the usb interface call
/// over the handler
struct SetHeader;
impl SetHeader {
    const BMREQUESTTYPE: u8 = 0x21;
    const BREQUEST: u8 = 0x09;
    const WVALUE: u16 = 0x03a1;
    const WINDEX: u16 = 0x0001;
}

/// Header for GET_REPORT requests:
/// predefines all the headerfields for the usb interface vall
/// over the handler
struct GetHeader;
impl GetHeader {
    const BMREQUESTTYPE: u8 = 0xa1;
    const BREQUEST: u8 = 0x01;
    const WVALUE: u16 = 0x03a1;
    const WINDEX: u16 = 0x0001;
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
    const PAYLOAD_LENGTH: usize = 64;

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
    pub fn read_profile(&self) -> Result<Profile, String> {
        use GetHeader as GH;
        use SetHeader as SH;

        // initializing empty profile
        let mut profile: Profile = Profile::init();

        let mut payload_handshake: [u8; Self::PAYLOAD_LENGTH] = [0; Self::PAYLOAD_LENGTH];
        let mut payload_get: [u8; Self::PAYLOAD_LENGTH] = [0; Self::PAYLOAD_LENGTH];
        payload_handshake[0] = 0xa1;
        payload_handshake[1] = 0x02;
        let mut payload_read: [u8; Self::PAYLOAD_LENGTH] = [0; Self::PAYLOAD_LENGTH];
        payload_read[0] = 0xa1;
        payload_read[1] = 0x12;
        let handshake_response: [u8; Self::PAYLOAD_LENGTH] = [
            0xa1, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x02, 0x01, 0x00, 0x67, 0x33, 0x80, 0x19, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        // Requesting Handshake
        self.handle
            .write_control(
                SH::BMREQUESTTYPE,
                SH::BREQUEST,
                SH::WVALUE,
                SH::WINDEX,
                payload_handshake.as_mut_slice(),
                Duration::new(1000, 0),
            )
            .map_err(|e| format!("handshake failed: {e}"))?;

        // Reading Handshake
        self.handle
            .read_control(
                GH::BMREQUESTTYPE,
                GH::BREQUEST,
                GH::WVALUE,
                GH::WINDEX,
                payload_get.as_mut_slice(),
                Duration::new(1000, 0),
            )
            .map_err(|e| format!("reading handshake failed: {e}"))?;

        // Validating Handshake
        if !(payload_get == handshake_response) {
            return Err("handshake response did not match".into());
        }

        // Requesting Read Profile
        self.handle
            .write_control(
                SH::BMREQUESTTYPE,
                SH::BREQUEST,
                SH::WVALUE,
                SH::WINDEX,
                payload_read.as_mut_slice(),
                Duration::new(1000, 0),
            )
            .map_err(|e| format!("read request failed: {e}"))?;

        // reading profile to `profile`
        self.handle
            .read_control(
                GetHeader::BMREQUESTTYPE,
                GetHeader::BREQUEST,
                GetHeader::WVALUE,
                GetHeader::WINDEX,
                profile.profile_buf.as_mut_slice(),
                Duration::new(1000, 0),
            )
            .expect("failed reading profile");
        // returning profile
        Ok(profile)
    }

    fn write_profile(&self, profile: &mut Profile) {
        // setting set_report signature to buf
        profile.profile_buf[0] = 0xa0;
        profile.profile_buf[1] = 0x11;

        todo!();
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
