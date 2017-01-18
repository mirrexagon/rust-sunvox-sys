//! Play a SunVox project.
//!


// --- External crates --- //
extern crate libc;
extern crate sunvox_sys;
// --- ==== --- //


// --- Use --- //
use std::env;
use std::ffi::CString;
use std::thread;
use std::time;

use sunvox_sys::*;
// --- ==== --- //


// --- Main --- //
fn main() {
    let file_name = env::args().nth(1).expect("Specify a .sunvox file to play.");
    let file_name = CString::new(file_name).unwrap();

    // ---

    let sv_ver = unsafe { sv_init(0 as *mut _, 48000, 2, 0) };
    if sv_ver < 0 {
        panic!("sv_init() error: {}", sv_ver);
    }

    let sv_ver_major = (sv_ver >> 16) & 255;
    let sv_ver_minor1 = (sv_ver >> 8) & 255;
    let sv_ver_minor2 = sv_ver & 255;

    println!("SunVox library version: {}.{}.{}",
             sv_ver_major,
             sv_ver_minor1,
             sv_ver_minor2);

    // ---

    unsafe {
        sv_open_slot(0);
    }

    println!("Loading SunVox file...");
    let file_name_ptr = file_name.into_raw();
    let ok = unsafe { sv_load(0, file_name_ptr) };
    let file_name = unsafe { CString::from_raw(file_name_ptr) };
    if ok != 0 {
        panic!("Could not load {:?}: {}", file_name, ok);
    }

    // ---

    unsafe {
        sv_play_from_beginning(0);
    }

    let loop_delay = time::Duration::new(1, 0);
    loop {
        println!("Current line: {}", unsafe { sv_get_current_line(0) });
        thread::sleep(loop_delay);
    }
}
// --- ==== --- //
