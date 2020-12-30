use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-lib=SimConnect");
    println!(r#"cargo:rustc-link-search=C:\MSFS SDK\SimConnect SDK\lib"#);

    // setup the bindings generator and tell it for what we want to generate the bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args(&["-x", "c++"])
        .whitelist_function("SimConnect_Open")
        .whitelist_function("SimConnect_Close")
        .whitelist_function("SimConnect_MapClientEventToSimEvent")
        .whitelist_function("SimConnect_AddClientEventToNotificationGroup")
        .whitelist_function("SimConnect_SetNotificationGroupPriority")
        .whitelist_function("SimConnect_GetNextDispatch")
        .whitelist_type("SIMCONNECT_RECV_EXCEPTION")
        .whitelist_type("SIMCONNECT_RECV_EVENT")
        .whitelist_type("SIMCONNECT_RECV_ID")
        .whitelist_var("SIMCONNECT_GROUP_PRIORITY_STANDARD")
        .generate()
        .expect("Unable to generate bindings");

    // tell cargo where we want to store the newly generated bindings
    bindings
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
