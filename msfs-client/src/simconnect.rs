use crate::bindings;
use std::ffi::c_void;

pub struct SimConnect {
    handle: std::ptr::NonNull<c_void>,
}

impl SimConnect {
    pub fn new() -> Result<Self, u32> {
        let mut handle = std::ptr::null_mut();
        let name = b"flyg-msfs-client\0" as *const u8 as *const i8;
        let hresult = unsafe {
            bindings::SimConnect_Open(
                &mut handle,
                name,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                0,
            )
        };
        if 0 != hresult {
            return Err(hresult as u32);
        }
        return Ok(Self {
            handle: std::ptr::NonNull::new(handle).expect("Foo"),
        });
    }
}

impl Drop for SimConnect {
    fn drop(&mut self) {
        let _ = unsafe { bindings::SimConnect_Close(self.handle.as_ptr()) };
    }
}
