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
        .whitelist_function("SimConnect_GetLastSentPacketID")
        .whitelist_function("SimConnect_GetNextDispatch")
        .whitelist_function("SimConnect_Text")
        .whitelist_function("SimConnect_AddToDataDefinition")
        .whitelist_function("SimConnect_RequestDataOnSimObject")
        .whitelist_type("SIMCONNECT_EXCEPTION")
        .whitelist_type("SIMCONNECT_RECV_EXCEPTION")
        .whitelist_type("SIMCONNECT_RECV_EVENT")
        .whitelist_type("SIMCONNECT_RECV_SIMOBJECT_DATA")
        .whitelist_type("SIMCONNECT_RECV_ID")
        .whitelist_var("SIMCONNECT_UNUSED")
        .whitelist_var("SIMCONNECT_OBJECT_ID_USER")
        .whitelist_var("SIMCONNECT_DATA_REQUEST_FLAG_CHANGED")
        .generate()
        .expect("Unable to generate bindings");

    // tell cargo where we want to store the newly generated bindings
    bindings
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
