use crate::bindings;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::ffi::c_void;

#[repr(u32)]
#[derive(Copy, Clone, TryFromPrimitive)]
pub enum Events {
    Brakes,
}

impl Events {
    fn as_c_str(self) -> *const u8 {
        match self {
            Events::Brakes => "brakes\0".as_ptr(),
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Groups {
    Group0,
}

pub enum Notification {
    Connected,
    Disconnected,
    Brakes,
}

pub struct SimConnect {
    handle: std::ptr::NonNull<c_void>,
}

impl SimConnect {
    pub fn new() -> Result<Self, i32> {
        let mut handle = std::ptr::null_mut();
        let name = b"flyg-msfs-client\0" as *const u8 as *const i8;

        //
        let open_result = unsafe {
            bindings::SimConnect_Open(
                &mut handle,
                name,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                0,
            )
        };
        if 0x0 != open_result {
            return Err(open_result);
        }
        Ok(Self {
            handle: std::ptr::NonNull::new(handle).expect("Foo"),
        })
    }

    pub fn register_event(&self, event: Events) -> Result<(), i32> {
        let map_client_event_to_sim_event_result = unsafe {
            bindings::SimConnect_MapClientEventToSimEvent(
                self.handle.as_ptr(),
                event as u32,
                event.as_c_str() as *const i8,
            )
        };
        if 0x0 != map_client_event_to_sim_event_result {
            return Err(map_client_event_to_sim_event_result);
        }

        let group = Groups::Group0;
        let add_client_event_to_notification_group_result = unsafe {
            bindings::SimConnect_AddClientEventToNotificationGroup(
                self.handle.as_ptr(),
                group as u32,
                event as u32,
                0,
            )
        };
        if 0x0 != add_client_event_to_notification_group_result {
            return Err(add_client_event_to_notification_group_result);
        }

        let set_notification_group_priority_result = unsafe {
            bindings::SimConnect_SetNotificationGroupPriority(
                self.handle.as_ptr(),
                group as u32,
                bindings::SIMCONNECT_GROUP_PRIORITY_STANDARD,
            )
        };
        if 0x0 != set_notification_group_priority_result {
            return Err(set_notification_group_priority_result);
        }

        Ok(())
    }

    pub fn get_next_notification(&self) -> Option<Notification> {
        use log::{error, trace};

        //
        let mut data: *mut bindings::SIMCONNECT_RECV = std::ptr::null_mut();
        let mut cb_data: bindings::DWORD = 0;

        // query the simulator for the next event we want to process
        let get_next_dispatch_result = unsafe {
            bindings::SimConnect_GetNextDispatch(self.handle.as_ptr(), &mut data, &mut cb_data)
        };

        // check if we succeeded (S_OK [0x0]). If not, handle the error appropriately
        if 0x0 != get_next_dispatch_result {
            // even if the Prepared3D documentation says that the call should succeed if no message is available
            // and the event should then be SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NULL, it seems that the MSFS2020
            // does not support this behaviour. Right now (30-12-2020) the simulator returns S_FAIL (0x80004005)
            // if no message is currently available. We should remove this fix as soon as the simulator implements
            // a correct behaviour
            if get_next_dispatch_result as u32 == 0x80004005 {
                return None;
            }

            // for all other error codes, log the error and return None
            error!(
                "The call to SimConnect_GetNextDispatch failed. The result was 0x{:x}",
                get_next_dispatch_result
            );
            return None;
        }

        // since we got an event from the simulator, we now have to convert it to an event we can
        // process using our API
        match unsafe { (*data).dwID as i32 } {
            // this event should be returned if the simulator did not have any events which it could
            // provide to us. This can happen on a regular basis and can be safely ignored
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NULL => {
                trace!("No SimConnect notifications available to return");
                None
            }

            // simple event that the connection to the simulator succeeded
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_OPEN => Some(Notification::Connected),

            // simple event that the connection to the simulator was closed
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_QUIT => {
                Some(Notification::Disconnected)
            }

            // we got an event we subscribed to, this is the base event which handles all event-
            // subscriptions. Now we have to check for which event it was emitted and convert it
            // the the proper data structure we can return
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT => {
                let event = unsafe { *(data as *const bindings::SIMCONNECT_RECV_EVENT) };
                match Events::try_from(event.uEventID) {
                    // the brakes were pressed / released (TODO: differentiate between the brake states)
                    Ok(Events::Brakes) => Some(Notification::Brakes),

                    // this should not really happen at all and if it happens it indicates an issue
                    // with our wrapper code
                    _ => {
                        error!("");
                        None
                    }
                }
            }

            // we got an unknown event type. This is not really an issue because we simply do not
            // need the information. While we are in development mode, we should log it anyway to see
            // if we miss something important
            id => {
                trace!("Got an unknown simulator event which is currently not handled. Its identifier was 0x{:x}", id);
                None
            }
        }
    }
}

impl Drop for SimConnect {
    fn drop(&mut self) {
        let _ = unsafe { bindings::SimConnect_Close(self.handle.as_ptr()) };
    }
}
