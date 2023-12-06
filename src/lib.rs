#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::prelude::{DateTime, Utc};
use std::os::raw::{c_char};

fn debug_append(prefix: &str, message: &str) {


    let st = std::time::SystemTime::now();
    let dt: DateTime<Utc> = st.clone().into();
    let now  = format!("{}", dt.format("%+"));
    println!("Hello, world! The time is {:?}", now);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("/tmp/debug.log")
        .expect("Unable to open file");

    if let Err(e) = writeln!(file, "{} [{}] {}", now, prefix, message) {
        eprintln!("Couldn't write to file: {}", e);
    }

}

fn main() {
    debug_append("main", "main fn ran");
}

#[no_mangle]
pub extern "C" fn run(_context: *const c_char) -> bool {

    debug_append("run", "Addin started 2");


    let my_string = "foo bar baz string string string";

    // Convert the string to a slice of bytes
    let bytes_slice = my_string.as_bytes();

    // assert!(bytes_slice.len() >= 24, "String is too short");

    // Create a reference to the first three u64 elements in the bytes slice
    let u64_slice = unsafe { &*(bytes_slice.as_ptr() as *const [u64; 3]) };


    unsafe {

        adsk_core_Application::log(u64_slice, adsk_core_LogLevels_InfoLogLevel, adsk_core_LogTypes_ConsoleLogType);
        debug_append("run", "Application should have logged");

    }


    true
}

#[no_mangle]
pub extern "C" fn stop(_context: *const c_char) -> bool {

    debug_append("stop", "Addin stopped");

    true
}
