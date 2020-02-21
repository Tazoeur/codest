# Codest

Small project to quickly see if a string is encoded in a known format.

## Installation & use

- clone project
- `cargo build --release`
- bring `target/release/codest` to your path
- `codest -h` for help

```text
codest 1.0
Taz <g0latour@gmail.com>
Easily test a string with many decoding tools

USAGE:
    codest [FLAGS] <INPUT>

FLAGS:
    -h, --help       Prints help information
    -s, --silent     Prevent any log message to be output
    -l, --list       Display a list of the supported decoder
    -V, --version    Prints version information
    -v               Sets the level of verbosity (Error[default] > Warning > Info > Debug > Trace)

ARGS:
    <INPUT>    The string you want to test
```
