//! sunvox-sys
//!
//! FFI bindings to the Sunvox DLL (http://warmplace.ru/soft/sunvox).
//!
//! Most comments here are original comments from `sunvox.h`.
//! TODO: Go back and rewrite them?
//!


// --- Crate attributes --- //
#![allow(non_camel_case_types)]
// --- ==== --- //


// --- External crates --- //
extern crate libc;
// --- ==== --- //


// --- Use --- //
use libc::{c_void, c_int, c_uint, c_char, c_uchar, c_short, c_ushort};
// --- ==== --- //


pub const NOTECMD_NOTE_OFF: c_int = 128;
/// notes of all synths off
pub const NOTECMD_ALL_NOTES_OFF: c_int = 129;
/// stop and clean all synths
pub const NOTECMD_CLEAN_SYNTHS: c_int = 130;
pub const NOTECMD_STOP: c_int = 131;
pub const NOTECMD_PLAY: c_int = 132;

// I can't find these in the official header file, but they're defined in
// https://github.com/metrasynth/sunvox-dll-python/blob/master/sunvox/types.py
pub const NOTECMD_SET_PITCH: c_int = 133;
pub const NOTECMD_PREV_TRACK: c_int = 134;


// sv_send_event() parameters:
//   slot;
//   track_num: from 0 to 15;
//   note: 0 - nothing; 1..127 - note num; 128 - note off; 129, 130... - see NOTECMD_xxx defines;
//   vel: velocity 1..129; 0 - default;
//   module: 0 - nothing; 1..255 - module number;
//   ctl: CCXX. CC - number of controller. XX - std effect;
//   ctl_val: value of controller.


#[repr(C)]
#[derive(Clone, Debug)]
pub struct sunvox_note {
    /// 0 - nothing; 1..127 - note num; 128 - note off; 129, 130... - see NOTECMD_xxx defines
    note: c_uchar,

    /// Velocity 1..129; 0 - default
    vel: c_uchar,

    /// 0 - nothing; 1..255 - module number (real module number + 1)
    module: c_uchar,

    nothing: c_uchar,

    /// CCEE. CC - number of a controller (1..255). EE - std effect
    ctl: c_ushort,

    /// XXYY. Value of controller/effect
    ctl_val: c_ushort,
}


pub const SV_INIT_FLAG_NO_DEBUG_OUTPUT: c_int = 1 << 0;
/// Interaction with sound card is on the user side
pub const SV_INIT_FLAG_USER_AUDIO_CALLBACK: c_int = 1 << 1;
pub const SV_INIT_FLAG_AUDIO_INT16: c_int = 1 << 2;
pub const SV_INIT_FLAG_AUDIO_FLOAT32: c_int = 1 << 3;
/// Audio callback and song modification functions are in single thread
pub const SV_INIT_FLAG_ONE_THREAD: c_int = 1 << 4;


pub const SV_MODULE_FLAG_EXISTS: c_int = 1;
pub const SV_MODULE_FLAG_EFFECT: c_int = 2;
pub const SV_MODULE_INPUTS_OFF: c_int = 16;
pub const SV_MODULE_INPUTS_MASK: c_int = 255 << SV_MODULE_INPUTS_OFF;
pub const SV_MODULE_OUTPUTS_OFF: c_int = 16 + 8;
pub const SV_MODULE_OUTPUTS_MASK: c_int = 255 << SV_MODULE_OUTPUTS_OFF;


pub const SV_STYPE_INT16: c_int = 0;
pub const SV_STYPE_INT32: c_int = 0;
pub const SV_STYPE_FLOAT32: c_int = 0;
pub const SV_STYPE_FLOAT64: c_int = 0;


// USE LOCK/UNLOCK! - Use the functions with this label within the sv_lock_slot() / sv_unlock_slot() block only!

#[link(name = "sunvox")]
extern "C" {
    /// sv_audio_callback() - get the next piece of SunVox audio.
    ///
    /// With sv_audio_callback() you can ignore the built-in SunVox sound output mechanism and use some other sound system.
    /// Set SV_INIT_FLAG_USER_AUDIO_CALLBACK flag in sv_init() if you want to use sv_audio_callback() function.
    /// Parameters:
    /// buf - destination buffer of type signed short (if SV_INIT_FLAG_AUDIO_INT16 used in sv_init())
    ///       or float (if SV_INIT_FLAG_AUDIO_FLOAT32 used in sv_init());
    ///       stereo data will be interleaved in this buffer: LRLR... ; where the LR is the one frame (Left+Right channels);
    /// frames - number of frames in destination buffer;
    /// latency - audio latency (in frames);
    /// out_time - output time (in ticks).
    pub fn sv_audio_callback(buf: *mut c_void,
                             frames: c_int,
                             latency: c_int,
                             out_time: c_uint)
                             -> c_int;


    pub fn sv_open_slot(slot: c_int) -> c_int;
    pub fn sv_close_slot(slot: c_int) -> c_int;
    pub fn sv_lock_slot(slot: c_int) -> c_int;
    pub fn sv_unlock_slot(slot: c_int) -> c_int;


    pub fn sv_init(dev: *const c_char, freq: c_int, channels: c_int, flags: c_uint) -> c_int;
    pub fn sv_deinit() -> c_int;


    /// sv_get_sample_type() - get internal sample type of the SunVox engine. Return value: one of the SV_STYPE_xxx defines.
    ///
    /// Use it to get the scope buffer type from get_module_scope() function.
    pub fn sv_get_sample_type() -> c_int;


    pub fn sv_load(slot: c_int, name: *const c_char) -> c_int;
    pub fn sv_load_from_memory(slot: c_int, data: *mut c_void, data_size: c_uint) -> c_int;


    pub fn sv_play(slot: c_int) -> c_int;
    pub fn sv_play_from_beginning(slot: c_int) -> c_int;
    pub fn sv_stop(slot: c_int) -> c_int;

    /// autostop values: 0 - disable autostop; 1 - enable autostop.
    /// When disabled, song is playing infinitely in the loop.
    pub fn sv_set_autostop(slot: c_int, autostop: c_int) -> c_int;


    /// sv_end_of_song() return values: 0 - song is playing now; 1 - stopped.
    pub fn sv_end_of_song(slot: c_int) -> c_int;


    pub fn sv_rewind(slot: c_int, line_num: c_int) -> c_int;


    pub fn sv_volume(slot: c_int, vol: c_int) -> c_int;


    /// track_num - track number (0..15) within the special pattern
    /// ctl - 0xCCEE. CC - number of a controller (1..255). EE - std effect
    /// ctl_val - value of controller/effect
    pub fn sv_send_event(slot: c_int,
                         track_num: c_int,
                         note: c_int,
                         vel: c_int,
                         module: c_int,
                         ctl: c_int,
                         ctl_val: c_int)
                         -> c_int;


    /// Get current line number
    pub fn sv_get_current_line(slot: c_int) -> c_int;

    /// Get current line number in fixed point format: 27.5
    pub fn sv_get_current_line2(slot: c_int) -> c_int;


    /// From 0 to 255
    pub fn sv_get_current_signal_level(slot: c_int, channel: c_int) -> c_int;


    pub fn sv_get_song_name(slot: c_int) -> *const c_char;


    pub fn sv_get_song_bpm(slot: c_int) -> c_int;
    pub fn sv_get_song_tpl(slot: c_int) -> c_int;


    /// Frame is one discrete of the sound. Sampling frequency 44100 Hz means, that you hear 44100 frames per second.
    pub fn sv_get_song_length_frames(slot: c_int) -> c_uint;
    pub fn sv_get_song_length_lines(slot: c_int) -> c_uint;


    /// sv_new_module() - create a new module;
    ///
    /// USE LOCK/UNLOCK!
    pub fn sv_new_module(slot: c_int,
                         _type: *const c_char,
                         name: *const c_char,
                         x: c_int,
                         y: c_int,
                         z: c_int)
                         -> c_int;

    /// sv_remove_module() - remove selected module;
    ///
    /// USE LOCK/UNLOCK!
    pub fn sv_remove_module(slot: c_int, mod_num: c_int) -> c_int;

    /// sv_connect_module() - connect the source to the destination;
    ///
    /// USE LOCK/UNLOCK!
    pub fn sv_connect_module(slot: c_int, source: c_int, destination: c_int) -> c_int;

    /// sv_disconnect_module() - disconnect the source from the destination;
    ///
    /// USE LOCK/UNLOCK!
    pub fn sv_disconnect_module(slot: c_int, source: c_int, destination: c_int) -> c_int;

    /// sv_load_module() - load a module; supported file formats: sunsynth, xi, wav, aiff;
    pub fn sv_load_module(slot: c_int,
                          file_name: *const c_char,
                          x: c_int,
                          y: c_int,
                          z: c_int)
                          -> c_int;

    /// sv_sampler_load() - load a sample to already created Sampler; to replace the whole sampler - set sample_slot to -1;
    pub fn sv_sampler_load(slot: c_int,
                           sampler_module: c_int,
                           file_name: *const c_char,
                           sample_slot: c_int)
                           -> c_int;


    pub fn sv_get_number_of_modules(slot: c_int) -> c_int;


    pub fn sv_get_module_flags(slot: c_int, mod_num: c_int) -> c_uint;
    pub fn sv_get_module_inputs(slot: c_int, mod_num: c_int) -> *mut c_int;
    pub fn sv_get_module_outputs(slot: c_int, mod_num: c_int) -> *mut c_int;
    pub fn sv_get_module_name(slot: c_int, mod_num: c_int) -> *const c_char;
    pub fn sv_get_module_xy(slot: c_int, mod_num: c_int) -> c_uint;
    pub fn sv_get_module_color(slot: c_int, mod_num: c_int) -> c_int;
    pub fn sv_get_module_scope(slot: c_int,
                               mod_num: c_int,
                               channel: c_int,
                               buffer_offset: *mut c_int,
                               buffer_size: *mut c_int)
                               -> *mut c_void;

    /// sv_get_module_scope2() return value = received number of samples (may be less or equal to samples_to_read).
    pub fn sv_get_module_scope2(slot: c_int,
                                mod_num: c_int,
                                channel: c_int,
                                read_buf: *mut c_short,
                                samples_to_read: c_uint)
                                -> c_uint;

    pub fn sv_get_number_of_module_ctls(slot: c_int, mod_num: c_int) -> c_int;
    pub fn sv_get_module_ctl_name(slot: c_int, mod_num: c_int, ctl_num: c_int) -> *const c_char;
    pub fn sv_get_module_ctl_value(slot: c_int,
                                   mod_num: c_int,
                                   ctl_num: c_int,
                                   scaled: c_int)
                                   -> c_int;


    pub fn sv_get_number_of_patterns(slot: c_int) -> c_int;
    pub fn sv_get_pattern_x(slot: c_int, pat_num: c_int) -> c_int;
    pub fn sv_get_pattern_y(slot: c_int, pat_num: c_int) -> c_int;
    pub fn sv_get_pattern_tracks(slot: c_int, pat_num: c_int) -> c_int;
    pub fn sv_get_pattern_lines(slot: c_int, pat_num: c_int) -> c_int;


    /// How to use sv_get_pattern_data():
    ///   pat_tracks: c_int = sv_get_pattern_tracks(slot, pat_num);
    ///   sunvox_note* data = sv_get_pattern_data(slot, pat_num);
    ///   sunvox_note* n = &data[ line_number * pat_tracks + track_number ];
    ///   ... and then do someting with note n
    pub fn sv_get_pattern_data(slot: c_int, pat_num: c_int) -> *mut sunvox_note;


    /// USE LOCK/UNLOCK!
    pub fn sv_pattern_mute(slot: c_int, pat_num: c_int, mute: c_int) -> c_int;


    /// SunVox engine uses its own time space, measured in ticks.
    ///
    /// Use sv_get_ticks() to get current tick counter (from 0 to 0xFFFFFFFF).
    pub fn sv_get_ticks() -> c_uint;

    /// Use sv_get_ticks_per_second() to get the number of SunVox ticks per second.
    pub fn sv_get_ticks_per_second() -> c_uint;
}
