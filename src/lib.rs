//! # sunvox-sys
//!
//! FFI bindings to the Sunvox library (http://warmplace.ru/soft/sunvox).
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


/// Single note off.
pub const NOTECMD_NOTE_OFF: c_int = 128;

/// Notes of all synths off.
pub const NOTECMD_ALL_NOTES_OFF: c_int = 129;

/// Stop and clean all synths.
pub const NOTECMD_CLEAN_SYNTHS: c_int = 130;

pub const NOTECMD_STOP: c_int = 131;
pub const NOTECMD_PLAY: c_int = 132;

// I can't find these in the official header file, but they're defined in
// https://github.com/metrasynth/sunvox-dll-python/blob/master/sunvox/types.py
/// Change the pitch of a currently playing note.
pub const NOTECMD_SET_PITCH: c_int = 133;

/// Apply effect in this note cell to the corresponding one in the previous track.
pub const NOTECMD_PREV_TRACK: c_int = 134;




/// A single note cell in a pattern.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct sunvox_note {
    /// The note column.
    ///
    ///
    /// - 0:  Nothing.
    /// - 1 to 127 inclusive: A normal note.
    /// - 128+: See the `NOTECMD` constants.
    pub note: c_uchar,

    /// The velocity column (note velocity).
    ///
    /// - 0: Empty (default).
    /// - 1 to 129 inclusive: The specified velocity + 1.
    pub vel: c_uchar,

    /// The module column (module to affect).
    ///
    /// - 0: Empty (none).
    /// - 1 to 255 inclusive: The specified module + 1.
    pub module: c_uchar,

    /// Padding.
    pub nothing: c_uchar,

    /// The value of the controller/effect column.
    ///
    /// Interpreted as a hexadecimal number, the first two digits are the
    /// controller of the selected module to affect, and the last two digits
    /// are the number of an effect. Set a pair of digits to zero to
    /// ignore that part.
    pub ctl: c_ushort,

    /// The value of the controller/effect parameter column.
    pub ctl_val: c_ushort,
}


pub const SV_INIT_FLAG_NO_DEBUG_OUTPUT: c_int = 1 << 0;

/// Interaction with sound card is on the user side.
pub const SV_INIT_FLAG_USER_AUDIO_CALLBACK: c_int = 1 << 1;
pub const SV_INIT_FLAG_AUDIO_INT16: c_int = 1 << 2;
pub const SV_INIT_FLAG_AUDIO_FLOAT32: c_int = 1 << 3;

/// Audio callback and song modification functions are in a single thread.
///
/// I believe by default they are run in separate threads.
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
    /// Get the next piece of SunVox audio.
    ///
    /// With `sv_audio_callback(`) you can ignore the built-in SunVox sound
    /// output mechanism and use some other sound system. Set the
    /// `SV_INIT_FLAG_USER_AUDIO_CALLBACK` flag when calling `sv_init()` if
    /// you want to use this function.
    ///
    /// # Parameters
    ///
    /// - buf: Destination buffer. If `SV_INIT_FLAG_AUDIO_INT16` was passed to
    /// `sv_init()`, this is a buffer of `i16`s. If `SV_INIT_FLAG_AUDIO_FLOAT32`
    /// was passed, this is a buffer of `f32`s. Stereo data will be interleaved
    /// in this buffer: LRLR... ; where the LR is one frame (Left+Right channels).
    /// - frames: Number of frames in destination buffer.
    /// - latency: Audio latency (in frames).
    /// - out_time: Output time (in ticks).
    ///
    /// The `out_time` parameter is elaborated on a little bit in this thread:
    /// http://www.warmplace.ru/forum/viewtopic.php?f=12&t=4152
    ///
    /// For normal use, pass the value of `sv_get_ticks()`, as detailed in that
    /// thread.
    pub fn sv_audio_callback(buf: *mut c_void,
                             frames: c_int,
                             latency: c_int,
                             out_time: c_uint)
                             -> c_int;


    /// Open a slot.
    ///
    /// A slot is an instance of the SunVox engine. Each slot can have a single
    /// project loaded at a time.
    pub fn sv_open_slot(slot: c_int) -> c_int;

    /// Close a slot. See `sv_open_slot()` for more details.
    pub fn sv_close_slot(slot: c_int) -> c_int;

    /// Lock a slot.
    ///
    /// There are a few functions that need to be called between a
    /// `sv_lock_slot()`/`sv_unlock_slot()` pair. These are marked with
    /// "USE LOCK/UNLOCK!".
    pub fn sv_lock_slot(slot: c_int) -> c_int;

    /// Unlock a slot. See `sv_lock_slot()` for more details.
    pub fn sv_unlock_slot(slot: c_int) -> c_int;


    /// Initialize the library.
    ///
    /// The `flags` parameter takes either zero (for default options), or a
    /// number of `SV_INIT_FLAG_xxx` constants ORed together.
    pub fn sv_init(dev: *const c_char, freq: c_int, channels: c_int, flags: c_uint) -> c_int;

    /// Deinitialize the library.
    pub fn sv_deinit() -> c_int;


    /// Get the internal sample type of the SunVox engine.
    ///
    /// Returns one of the `SV_STYPE_xxx` constants.
    ///
    /// Use it to get the scope buffer type from `get_module_scope()` function.
    pub fn sv_get_sample_type() -> c_int;


    /// Load a SunVox project file into the specified slot.
    pub fn sv_load(slot: c_int, name: *const c_char) -> c_int;

    /// Load a SunVox project from file data in memory.
    pub fn sv_load_from_memory(slot: c_int, data: *mut c_void, data_size: c_uint) -> c_int;


    /// Start playing the project from the current play cursor position.
    pub fn sv_play(slot: c_int) -> c_int;

    /// Start playing the project from the beginning.
    pub fn sv_play_from_beginning(slot: c_int) -> c_int;

    /// Stop playing the project. The play cursor stays where it is.
    pub fn sv_stop(slot: c_int) -> c_int;

    /// Enable or disables autostop.
    ///
    /// - 0: Disable autostop.
    /// - 1: Enable autostop.
    ///
    /// When disabled, the project plays in a loop.
    pub fn sv_set_autostop(slot: c_int, autostop: c_int) -> c_int;


    /// Whether the project is stopped (ie. not playing).
    ///
    /// Returns 0 if it is playing, 1 if it is stopped.
    pub fn sv_end_of_song(slot: c_int) -> c_int;


    /// Rewind the project to the beginning.
    pub fn sv_rewind(slot: c_int, line_num: c_int) -> c_int;


    /// Set the volume of the project.
    pub fn sv_volume(slot: c_int, vol: c_int) -> c_int;


    /// Cause an event to occur as though it had been played in a pattern.
    ///
    /// `track_num` is in the range 0 to 15 inclusive, and refers to the track
    /// number in a special hidden pattern.
    pub fn sv_send_event(slot: c_int,
                         track_num: c_int,
                         note: c_int,
                         vel: c_int,
                         module: c_int,
                         ctl: c_int,
                         ctl_val: c_int)
                         -> c_int;


    /// Get the line number of the play cursor.
    pub fn sv_get_current_line(slot: c_int) -> c_int;

    /// Get the line number of the play in fixed point format: 27.5
    ///
    /// TODO: Figure out exactly what this means.
    pub fn sv_get_current_line2(slot: c_int) -> c_int;


    /// The current signal level/amplitude for a given audio channel in the
    /// range 0 to 255 inclusive.
    pub fn sv_get_current_signal_level(slot: c_int, channel: c_int) -> c_int;


    /// Get the name of the currently loaded project.
    ///
    /// Returns zero if no project is loaded.
    pub fn sv_get_song_name(slot: c_int) -> *const c_char;


    /// Get the Beats Per Minute of the currently loaded project.
    ///
    /// Returns zero if no project is loaded.
    pub fn sv_get_song_bpm(slot: c_int) -> c_int;


    /// Get the Ticks Per Line of the currently loaded project.
    ///
    /// Returns zero if no project is loaded.
    pub fn sv_get_song_tpl(slot: c_int) -> c_int;


    /// Frame is one discrete of the sound. Sampling frequency 44100 Hz means,
    /// that you hear 44100 frames per second.
    pub fn sv_get_song_length_frames(slot: c_int) -> c_uint;
    pub fn sv_get_song_length_lines(slot: c_int) -> c_uint;


    /// Create a new module.
    ///
    /// USE LOCK/UNLOCK!
    pub fn sv_new_module(slot: c_int,
                         _type: *const c_char,
                         name: *const c_char,
                         x: c_int,
                         y: c_int,
                         z: c_int)
                         -> c_int;

    /// Remove the specified module. USE LOCK/UNLOCK!
    pub fn sv_remove_module(slot: c_int, mod_num: c_int) -> c_int;

    /// Connect the source to the destination. USE LOCK/UNLOCK!
    pub fn sv_connect_module(slot: c_int, source: c_int, destination: c_int) -> c_int;

    /// Disconnect the source from the destination. USE LOCK/UNLOCK!
    pub fn sv_disconnect_module(slot: c_int, source: c_int, destination: c_int) -> c_int;

    /// Load a module.
    ///
    /// Supported file formats: `sunsynth`, `xi`, `wav`, `aiff`
    pub fn sv_load_module(slot: c_int,
                          file_name: *const c_char,
                          x: c_int,
                          y: c_int,
                          z: c_int)
                          -> c_int;

    /// Load a sample to an existing Sampler.
    ///
    /// To replace the whole sampler, set `sample_slot` to -1.
    pub fn sv_sampler_load(slot: c_int,
                           sampler_module: c_int,
                           file_name: *const c_char,
                           sample_slot: c_int)
                           -> c_int;


    /// Get the number of modules in the currently loaded project?
    ///
    /// Does not seem to directly correspond to that.
    ///
    /// Returns zero if no project is loaded.
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

    /// Return value:  received number of samples (may be less or equal to `samples_to_read`).
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
    ///
    /// - `int pat_tracks = sv_get_pattern_tracks(slot, pat_num);`
    /// - `sunvox_note* data = sv_get_pattern_data(slot, pat_num);`
    /// - `sunvox_note* n = &data[ line_number * pat_tracks + track_number ];`
    /// - ... and then do someting with note n
    pub fn sv_get_pattern_data(slot: c_int, pat_num: c_int) -> *mut sunvox_note;


    /// USE LOCK/UNLOCK!
    pub fn sv_pattern_mute(slot: c_int, pat_num: c_int, mute: c_int) -> c_int;


    /// Get the current tick counter (from 0 to 0xFFFFFFFF).
    ///
    /// SunVox engine uses its own time space, measured in ticks.
    pub fn sv_get_ticks() -> c_uint;

    /// Get the number of SunVox ticks per second.
    pub fn sv_get_ticks_per_second() -> c_uint;
}
