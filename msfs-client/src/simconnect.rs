use crate::bindings;
use num_enum::TryFromPrimitive;
use std::ffi::{c_void, CString};

macro_rules! handleSimConnectAPIError {
    ($hr:expr) => {
        let hr = $hr;
        if hr != 0 {
            return Err(hr as u32);
        }
    };
}

#[repr(u32)]
#[derive(Copy, Clone, TryFromPrimitive)]
pub enum Events {
    Brakes,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Groups {
    Group0,
}

pub struct SimConnect {
    pub handle: std::ptr::NonNull<c_void>,
}

impl SimConnect {
    pub fn new() -> Result<Self, u32> {
        let mut handle = std::ptr::null_mut();
        let name = b"flyg-msfs-client\0" as *const u8 as *const i8;
        handleSimConnectAPIError!(unsafe {
            bindings::SimConnect_Open(
                &mut handle,
                name,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                0,
            )
        });
        return Ok(Self {
            handle: std::ptr::NonNull::new(handle).expect("Foo"),
        });
    }

    pub fn associate_breaks(&self) -> Result<(), u32> {
        let event = Events::Brakes;
        let name = CString::new("brakes").unwrap();
        handleSimConnectAPIError!(unsafe {
            bindings::SimConnect_MapClientEventToSimEvent(
                self.handle.as_ptr(),
                event as u32,
                name.as_ptr(),
            )
        });
        let group = Groups::Group0;
        handleSimConnectAPIError!(unsafe {
            bindings::SimConnect_AddClientEventToNotificationGroup(
                self.handle.as_ptr(),
                group as u32,
                event as u32,
                0,
            )
        });
        handleSimConnectAPIError!(unsafe {
            bindings::SimConnect_SetNotificationGroupPriority(self.handle.as_ptr(), group as u32, 1)
        });
        Ok(())
    }
}

impl Drop for SimConnect {
    fn drop(&mut self) {
        let _ = unsafe { bindings::SimConnect_Close(self.handle.as_ptr()) };
    }
}
