# vtk-rs

Rust bindings for the [Visualization Toolkit (VTK)](https://vtk.org/).

## ⚠️ Fork Notice
This is a fork of [jonaspleyer/vtk-rs](https://github.com/jonaspleyer/vtk-rs) specifically optimized for **macOS ARM (Apple Silicon)**.

### Platform-Specific Implementation
- **Target Platform**: macOS with ARM architecture (M1/M2/M3 chips)
- **Modified Components**:
  - `vtk-rs/generate_headers.sh`: Updated `sed` commands for macOS compatibility
  - `vtk-rs/build.rs`: Added custom build configuration for ARM architecture
- **VTK Version**: Tested with VTK 9.3+ via Homebrew

### Key Changes from Upstream
1. macOS ARM-specific sed command syntax in header generation
2. Custom build.rs for proper linking on Apple Silicon
3. Comprehensive geometric primitives implementation (Priority 1 complete)

## Scope
The goal of this project is to provide safe and thin bindings.
This means we are planning to support as much of the original functionality as possible, provided
that their use is reasonable from a rusty point of view.
This crate does in particular not aim at formulating higher-level interfaces such as
[pyvista](https://docs.pyvista.org/) although such functionality could be added in the future within
the scope of additional crates.

## ❗ Note ❗
This crate will be reworked using
[vtkWrap](https://docs.vtk.org/en/latest/advanced/WrappingTools.html) in order to automate much of
the process of generating the bindings.
In its current state, the crate will probably remain unusable for now.

## Testing

| | stable | Env Flags + Command |
|---|---|---|
| `macos-latest` | [![stable-macos-latest](https://img.shields.io/github/actions/workflow/status/joley-gh/vtk-rs/test_stable_macos-latest.yml?style=flat-square&label=CI)](https://github.com/joley-gh/vtk-rs/actions/workflows/test_stable_macos-latest.yml) |`cargo test` |

## Dependencies

This package relies on a system install of `vtk`.
We currently only test versions `>=9.1`.

### macOS ARM (Apple Silicon)
For this fork, VTK must be installed via Homebrew:
```bash
brew install vtk
```

The build system is specifically configured for ARM architecture and uses modified `sed` commands compatible with macOS.

### Other Platforms (Original Implementation)
In some scenarios, it might be necessary to install additional dependencies.
Otherwise, compilation of the `cmake` part might fail with spurious linker errors.

| Distro | Packages |
| --- | --- |
| Archlinux | `pacman -S clang cmake vtk openmpi fast_float nlohmann-json gl2ps utf8cpp` |
| Ubuntu 22 & 24 | `apt install libvtk9.1 libvtk9-dev` |
| Macos 13 & 14 | `brew install vtk` |

**Note**: For macOS ARM users of this fork, ensure you're using the Homebrew installation as the build system expects ARM-specific paths and configurations.

## Building
`vtk-rs` will try to determine the path for `vtk` automatically.

### macOS ARM Build Process
This fork includes a custom `build.rs` that handles ARM-specific configuration:
1. Run `./vtk-rs/generate_headers.sh` to generate CXX bridge headers (uses macOS-compatible sed)
2. Build with `cargo build` - the custom build.rs handles VTK library detection

### Build Environment Flags
It is possible to control the compilation process via environment flags.
| Flag | Effect |
| --- | --- |
| `VTK_DIR` | `cargo:rustc-link-search=$VTK_DIR` |
| `VTK_VERSION` | Add suffix to vtk libraries (i.e. `libvtkCommonCore-9.4`). |

## Internals
This crate builds on [`cmake`](https://docs.rs/cmake/latest/cmake/) and [`cxx`](https://cxx.rs/)
in order to generate the necessary code and bindings.
The bindings for `vtk` modules are written manually in `C++`.
From there, we generate appropriate bindings with `cxx::bridge` using the CLI tool
[`cxxbridge`](https://crates.io/crates/cxxbridge-cmd).
However, we do not use `cxx` to compile the code but rather let `cmake` handle this task.
To implement the desired class methods, we use Rust
[macros](https://doc.rust-lang.org/reference/macros-by-example.html).

## Roadmap
1. [x] Stabilize Build system
2. [x] Automate system library detection and generate linker flags
3. [ ] Gradually implement functionality for examples. Start with 3D geometry.
- *SphereSource*
  https://examples.vtk.org/site/Cxx/GeometricObjects/SphereSource/
- *CylinderExample*
  https://examples.vtk.org/site/Cxx/GeometricObjects/CylinderExample/
