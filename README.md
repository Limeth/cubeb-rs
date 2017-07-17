# Discontinued

This project has been discontinued in favour of [the official Mozilla wrapper](https://github.com/djg/cubeb-rs).

# cubeb-rs

Rust abstraction for the cross-platform audio library Cubeb 

## Developer notes

### Generating the `src/ffi.rs`

In order to update the `src/ffi.rs`, you need the `bindgen` utility installed.
Next, you should be able to  run the following command from the project root
directory:

`bindgen -o src/ffi.rs cubeb/include/cubeb/cubeb.h --distrust-clang-mangling -- -Icubeb/build/exports`
