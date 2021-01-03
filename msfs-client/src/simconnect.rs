use crate::bindings;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::ffi::{c_void, CString};
use std::time::Duration;

macro_rules! as_c_string {
    ($target:expr) => {
        std::ffi::CString::new($target).unwrap().as_ptr();
    };
}

#[repr(u32)]
#[derive(Copy, Clone, TryFromPrimitive)]
pub enum Event {
    Brakes,
    UserTextDisplay,
}

impl Event {
    fn as_c_str(self) -> *const u8 {
        match self {
            Event::Brakes => "BRAKES\0".as_ptr(),
            Event::UserTextDisplay => std::ptr::null(),
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Group {
    Group0,
}

pub enum Notification {
    Connected,
    Disconnected,
    Brakes,
}

struct PositionInformation {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}

pub struct SimConnect {
    handle: std::ptr::NonNull<c_void>,
}

impl SimConnect {
    pub fn new() -> Result<Self, String> {
        let mut handle = std::ptr::null_mut();

        // try to connect to the local SimConnect server (e.g. the simulator)
        let open_result = unsafe {
            bindings::SimConnect_Open(
                &mut handle,
                as_c_string!("flyg-msfs-client"),
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                0,
            )
        };

        // check if the call to SimConnect_Open was successful or not and return a proper error message,
        // if the call failed
        if 0x0 != open_result {
            return Err(format!(
                "The call to SimConnect_Open failed with a return code of 0x{:x}",
                open_result
            ));
        }

        // try to create a NonNull instance from the returned connection handle
        let simconnect_handle = match std::ptr::NonNull::new(handle) {
            Some(real_handle) => real_handle,
            None => {
                return Err("The returned SimConnect handle was NULL which is not okay according to the API specifications".to_string());
            }
        };

        // return the new SimConnect instance
        Ok(SimConnect {
            handle: simconnect_handle,
        })
    }

    pub fn request_position_updates(&self) -> Result<(), i32> {
        use log::error;

        let data_definition_id = 100;
        let request_id = 200;

        let mut add_to_data_definition_result = unsafe {
            bindings::SimConnect_AddToDataDefinition(
                self.handle.as_ptr(),
                data_definition_id,
                as_c_string!("PLANE LATITUDE"),
                as_c_string!("Degrees"),
                bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
                0.0,
                bindings::SIMCONNECT_UNUSED,
            )
        };

        if 0x0 != add_to_data_definition_result {
            error!("FAIL!");
            return Err(add_to_data_definition_result);
        }

        add_to_data_definition_result = unsafe {
            bindings::SimConnect_AddToDataDefinition(
                self.handle.as_ptr(),
                data_definition_id,
                as_c_string!("PLANE LONGITUDE"),
                as_c_string!("Degrees"),
                bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
                0.0,
                bindings::SIMCONNECT_UNUSED,
            )
        };

        if 0x0 != add_to_data_definition_result {
            error!("FAIL!");
            return Err(add_to_data_definition_result);
        }

        add_to_data_definition_result = unsafe {
            bindings::SimConnect_AddToDataDefinition(
                self.handle.as_ptr(),
                data_definition_id,
                as_c_string!("PLANE ALTITUDE"),
                as_c_string!("Feet"),
                bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
                0.0,
                bindings::SIMCONNECT_UNUSED,
            )
        };

        if 0x0 != add_to_data_definition_result {
            error!("FAIL!");
            return Err(add_to_data_definition_result);
        }

        let request_data_on_sim_object_result = unsafe {
            bindings::SimConnect_RequestDataOnSimObject(
                self.handle.as_ptr(),
                request_id,
                data_definition_id,
                bindings::SIMCONNECT_OBJECT_ID_USER,
                bindings::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_VISUAL_FRAME,
                bindings::SIMCONNECT_DATA_REQUEST_FLAG_CHANGED,
                0,
                0,
                0,
            )
        };

        if 0x0 != request_data_on_sim_object_result {
            error!("FAIL!");
            return Err(request_data_on_sim_object_result);
        }

        Ok(())
    }

    pub fn register_event(&self, event: Event) -> Result<(), i32> {
        //
        let map_client_event_to_sim_event_result = unsafe {
            bindings::SimConnect_MapClientEventToSimEvent(
                self.handle.as_ptr(),
                event as u32,
                event.as_c_str() as *const i8,
            )
        };

        //
        if 0x0 != map_client_event_to_sim_event_result {
            return Err(map_client_event_to_sim_event_result);
        }

        //
        let group = Group::Group0;
        let add_client_event_to_notification_group_result = unsafe {
            bindings::SimConnect_AddClientEventToNotificationGroup(
                self.handle.as_ptr(),
                group as u32,
                event as u32,
                0,
            )
        };

        //
        if 0x0 != add_client_event_to_notification_group_result {
            return Err(add_client_event_to_notification_group_result);
        }

        //
        let set_notification_group_priority_result = unsafe {
            bindings::SimConnect_SetNotificationGroupPriority(
                self.handle.as_ptr(),
                group as u32,
                bindings::SIMCONNECT_GROUP_PRIORITY_STANDARD,
            )
        };

        //
        if 0x0 != set_notification_group_priority_result {
            return Err(set_notification_group_priority_result);
        }

        //
        Ok(())
    }

    pub fn display_message_to_user(&self, message: String, duration: Duration) {
        use log::error;

        // get a C-style representation of the string we want to display to the user
        let text_to_display = match CString::new(message.clone()) {
            Ok(message_as_cstring) => message_as_cstring,
            Err(_) => {
                error!("Could not convert message ({}) to a CString", message);
                return;
            }
        };

        // send the text to the SimConnect interface to be displayed to the user
        let sim_connect_text_result = unsafe {
            bindings::SimConnect_Text(
                self.handle.as_ptr(),
                bindings::SIMCONNECT_TEXT_TYPE_SIMCONNECT_TEXT_TYPE_PRINT_WHITE,
                duration.as_secs() as f32,
                Event::UserTextDisplay as u32,
                (message.len() + 1) as u32,
                text_to_display.as_ptr() as *mut std::ffi::c_void,
            )
        };

        // there is nothing we can do if the call failed, but at least log an error so we can fix the
        // issue in a future version
        if 0x0 != sim_connect_text_result {
            error!(
                "Failed to call SimConnect_Text. The result was 0x{:x}",
                sim_connect_text_result
            );
        }
    }

    pub fn get_next_notification(&self) -> Option<Notification> {
        use log::{error, trace};
        use std::mem::transmute_copy;

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
                match Event::try_from(event.uEventID) {
                    // the brakes were pressed / released (TODO: differentiate between the brake states)
                    Ok(Event::Brakes) => Some(Notification::Brakes),

                    // this event is emitted if a request from the user to display text in the simulator
                    // was executed, nothing we have to do here so we can just ignore it and return a
                    // None
                    Ok(Event::UserTextDisplay) => None,

                    // this should not really happen at all and if it happens it indicates an issue
                    // with our wrapper code
                    _ => {
                        error!("Could not process a received simulator event. This can could be due to an unhandled event or an issue with the simulator connection");
                        None
                    }
                }
            }

            // we got an update to a simulator object we requested for. This could be caused by a periodic
            // interval or even an exceeded epsilon on a datum (based on the way the update was requested).
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA => {
                let object_data: &bindings::SIMCONNECT_RECV_SIMOBJECT_DATA = unsafe {
                    transmute_copy(&(data as *const bindings::SIMCONNECT_RECV_SIMOBJECT_DATA))
                };
                match object_data.dwRequestID {
                    200 => {
                        unsafe { assert_eq!(object_data.dwDefineID, 100) };
                        let position_data: &PositionInformation =
                            unsafe { transmute_copy(&&object_data.dwData) };
                        trace!(
                            "Lat: {}, Long: {}, Alt: {}",
                            position_data.latitude,
                            position_data.longitude,
                            position_data.altitude
                        );
                        None
                    }

                    _ => {
                        unsafe {
                            trace!("Got SimObject for the request with the ID 0x{:x}, which was not handled", object_data.dwRequestID)
                        };
                        None
                    }
                }
            }

            // there was an exception in one of the last requests. All required information to fix this
            // issue are inside of the provided structure
            bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EXCEPTION => {
                let exception_data =
                    unsafe { *(data as *const bindings::SIMCONNECT_RECV_EXCEPTION) };
                unsafe {
                    error!("Exception (0x{:x}) during the execution of the command with the send message ID 0x{:x} (offset 0x{:x}, data 0x{:x})", exception_data.dwException, exception_data.dwSendID, exception_data.dwIndex, cb_data);
                }
                None
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
