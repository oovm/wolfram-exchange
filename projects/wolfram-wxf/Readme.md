Wolfram Exchange
================

`wex` is a tool that can convert various formats such as `yaml`, `toml` to wolfram format, **without the need of the wolfram runtime**.

## Install

- rust user?

```sh
cargo install wxf-converter
```

- want to try rust?

https://www.rust-lang.org/tools/install

- otherwise

Download the precompiled executable in [Github Release](https://github.com/GalAster/wolfram-exchange-cli/releases)

Rename to `wex` and put under `$Path`

## Usage

### Basic

```sh
wex in.yml      # check file but no output
wex in.yml -t   # output only text
wex in.yml -c   # output only binary
wex in.yml -ct  # output both text and binary
```

### Advance

use `wex -h` to get help.

```yaml
Wolfram Format Converter

USAGE:
    wex [FLAGS] [OPTIONS] <INPUT>

FLAGS:
    -b, --binary      Generates wxf file
    -c, --compress    Generates mx file
    -t, --text        Generates m file
    -h, --help        Prints help information
    -V, --version     Prints version information

OPTIONS:
    -f, --format <Format>    Sets the input file format

ARGS:
    <INPUT>    Sets the input file to use
```

## Contribution

If you have any questions or suggestions, post them on:

https://github.com/GalAster/wolfram-exchange/issues