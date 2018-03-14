# C API for the isosurface crate

The API is documented in the header file `include/isosurface.h`.

## Example

An example is located in `examples`.

Make sure that [Rust and Cargo are
installed](https://www.rust-lang.org/en-US/install.html), and you can
build it and run it:

```bash
$ git clone https://github.com/rekka/isosurface-rs.git
$ cd isosurface-rs/isosurface-capi/examples
$ ./compile
$ ./simple
```

## Acknowledgments

I used the great
[regex/regex-capi](https://github.com/rust-lang/regex/tree/master/regex-capi)
crate as a template.
