# rust-sunvox-sys 

[![crates.io](https://img.shields.io/crates/v/sunvox-sys.svg)](https://crates.io/crates/sunvox-sys)

FFI bindings to the SunVox library (http://warmplace.ru/soft/sunvox).


## Usage notes

Because I can't figure out how dynamically link `sunvox.so` with Rust at runtime (the way it is intended to be used in C), this crate expects to link to `libsunvox.so` in a library path. So, I installed `sunvox.so` on my system as `libsunvox.so` and patched its soname to be `libsunvox.so`.


## Attribution

- SunVox and the SunVox library are by Alexander Zolotov (http://warmplace.ru).
- The Metrasynth projects (https://github.com/metrasynth and https://metrasynth.readthedocs.io), specifically `sunvox-dll-python`, were very helpful to look at for reference.


## Crate License

This crate is released into the public domain via CC0. See `COPYING` for the license text.


## SunVox Library License

Note that the SunVox library itself has this license:

> You can use SunVox library freely, but the following text should be included in your products (e.g. in About window).
>
> SunVox modular synthesizer
> Copyright (c) 2008 - 2016, Alexander Zolotov <nightradio@gmail.com>, WarmPlace.ru
>
> Ogg Vorbis 'Tremor' integer playback codec
> Copyright (c) 2002, Xiph.org Foundation
