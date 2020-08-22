# <img alt="" width="48" align="top"  src="https://raw.githubusercontent.com/rust-skia/rust-skia/master/artwork/rust-skia-icon_512x512.png"/> Skia Bindings

This is a supporting package for [skia-safe](https://crates.io/crates/skia-safe), which provides safe Rust bindings to the [Skia Graphics Library](https://skia.org/).

## Organization

This package contains three components. 

- First, full configuration and build support for Skia in [`build.rs`](build.rs) and  [`build_support/`](build_support/).
- Additional C bindings to help out bindgen with stuff it has problems with or to work around linker errors. These are [`src/bindings.cpp`](src/bindings.cpp), and [`src/shaper.cpp`](src/shaper.cpp).
- And a number of functions that are used to download prebuilt binaries.

### Skia Build Support

Building Skia is quite exceptional, a number of prerequisites need to be available and configured properly for the target platform.

To configure and build Skia, [`build_support/skia.rs`](build_support/skia.rs) does all the hard work: it pulls `depot_tools/` and `skia/` from Google's repositories and a number of additional dependencies by executing `skia/tools/git-sync-deps` with Python. After that, it configures Skia with Google's [GN](https://gn.googlesource.com/gn/+/refs/heads/master/README.md) tool, and finally builds it by giving control to the `ninja` executable from the `depot_tools/` package.

### Binding Generation

`src/bindings.rs` and `src/shaper.cpp` contain the C++ code that Rust needs to interact with Skia's codebase. These files are processed by the [Rust's binding generator](<https://github.com/rust-lang/rust-bindgen>) that uses libclang for the layout computation _and also_ compiled by [clang](https://clang.llvm.org/).

If both steps went well, the resulting Rust binding code is written to `src/bindings.rs`, and the `skia-bindings` library is found in the output directory alongside where Skia was built previously.

### Debug Builds

By default, and for performance reasons, Skia is built in release mode even when cargo creates debug output. Skia debug builds can be enabled only by explicitly setting the environment variable `SKIA_DEBUG=1`.

### Prebuilt Binaries

Because building Skia _and_ creating the bindings is slow and depend on a number of components that lie outside the Rust ecosystem, we decided to experiment with prebuilt binaries.

Whenever a new version of `rust-skia` is built from the `release` branch on our CI server, the resulting Skia library, `skia-bindings` library, _and_ `bindings.rs` are uploaded to the releases tab of the [skia-binaries repository](<https://github.com/rust-skia/skia-binaries/releases>).

And whenever the build script detects that `skia-bindings` is built from inside a crate _and_ a prebuilt archive is available that matches the repository's hash, platform, and features, it downloads the package, unpacks it, and skips the full build step of Skia and the bindings.

## Offline Builds

A full build of Skia downloads several repositories from the Internet. For specific scenarios, like, for example, [a packaging script](https://github.com/rust-skia/rust-skia/issues/340), it can be useful to build Skia offline. To configure that, the build script supports the following environment variables:

| Variable                     | Description                                                  | Default                            |
| ---------------------------- | ------------------------------------------------------------ | ---------------------------------- |
| `SKIA_OFFLINE_SOURCE_DIR`    | An absolute path pointing to a complete Skia source tree including all dependencies. Setting this variable activates the offline build mode. |                                    |
| `SKIA_OFFLINE_NINJA_COMMAND` | The ninja command to run. This can be either a filename, or a absolute path to the ninja executable. | `ninja` or `ninja.exe`  on Windows |
| `SKIA_OFFLINE_GN_COMMAND`    | The `gn` command to run. Either an absolute path or a path relative to Skia's source directory. | `bin/gn`                           |

## Build Customization

Besides of the features `gl`, `vulkan`, `metal`, and `textlayout` that can be directly specified when the package is added as a cargo dependency, the Skia build can be customized further in `build.rs` by adjusting one of two structs that are defined in `build_support/skia.rs`:

### `BuildConfiguration`

This struct represents the top level build configuration for `skia-bindings` and contains a number of individual feature flags.

### `FinalBuildConfiguration`

The `FinalBuildConfiguration` is created from the `BuildConfiguration` and contains name value pairs used by GN to parameterize the Skia build and preprocessor defines used to create the `src/bindings.rs` file and the `skia-bindings` library.

