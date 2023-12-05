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

    debug_append("run", "Addin started");

    true
}

#[no_mangle]
pub extern "C" fn stop(_context: *const c_char) -> bool {

    debug_append("stop", "Addin stopped");

    true
}
