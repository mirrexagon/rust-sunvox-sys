//! Play multiple SunVox projects AT THE SAME TIME.
//!
//! This is a terrible idea.


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
    let mut file_names: Vec<_> = {
        let mut args_iter = env::args();
        args_iter.next().unwrap(); // Skip executable name.
        args_iter.map(|arg| CString::new(arg).unwrap()).collect()
    };

    let n_files = file_names.len();

    if n_files == 0 {
        panic!("No files specified.");
    }

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

    for (i, file_name) in file_names.drain(..).enumerate() {
        unsafe {
            if sv_open_slot(i as i32) < 0 {
                panic!("Could not open slot {}", i);
            }
        }

        println!("Loading {:?} into slot {}...", &file_name, i);
        let file_name_ptr = file_name.into_raw();
        let ok = unsafe { sv_load(i as i32, file_name_ptr) };
        let file_name = unsafe { CString::from_raw(file_name_ptr) };
        if ok != 0 {
            panic!("Could not load {:?}: {}", file_name, ok);
        }
    }

    // ---

    for i in 0..n_files {
        unsafe {
            sv_play_from_beginning(i as i32);
        }
    }

    let loop_delay = time::Duration::new(1, 0);
    loop {
        println!("Current line: {}", unsafe { sv_get_current_line(0) });
        thread::sleep(loop_delay);
    }
}
// --- ==== --- //
