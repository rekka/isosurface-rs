# C API for the isosurface crate

The API is documented in the header file `include/isosurface.h`.

## Example

An example is located in `examples`.

Make sure that [Rust and Cargo are
installed](https://www.rust-lang.org/en-US/install.html), and you can
build it and run it as:

```bash
$ git clone https://github.com/rekka/isosurface-rs.git
$ cd isosurface-rs/isosurface-capi/examples
$ ./compile
$ LD_LIBRARY_PATH=../../target/release ./simple
```

This uses `isosurface` as a shared (dynamic) library, located at
`../../target/release/libisosurface.{so,dylib}`.

For an example how to use it as a static library, see
[examples/compile](https://github.com/rekka/isosurface-rs/blob/master/isosurface-capi/examples/compile).

## Acknowledgments

I used the great
[regex/regex-capi](https://github.com/rust-lang/regex/tree/master/regex-capi)
crate as a template.

