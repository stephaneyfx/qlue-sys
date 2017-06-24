[![MIT license](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

# Purpose

Native bindings to QML for Rust using the qlue library.

# Build dependencies

- CMake
- C++14 compiler
- Qt 5.9 or newer

# Build

The `Qt5_DIR` environment variable may be set to help cmake find the Qt5 cmake
config script.

```sh
git clone https://github.com/stephaneyfx/qlue-sys.git
cd qlue-sys
cargo build
```

# Platform support

All, to the extent of the availability of the dependencies.
