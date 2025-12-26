# Getting Started with vtk-rs

This guide explains how to get vtk-rs working on your system after cloning the repository.

## Prerequisites

- Rust toolchain (stable)
- VTK 9.5 installed (via Homebrew on macOS: `brew install vtk`)
- CMake
- C++ compiler

## Quick Start

```bash
# Build the project (VTK will be auto-detected)
cargo build

# Run an example
cargo run --example sphere_source
```

The build system will automatically find VTK in common installation locations.

## Configuration (Optional)

### Environment Variables

While not required, you can set these for explicit control:

```bash
export VTK_DIR=/opt/homebrew/Cellar/vtk/9.5.2_2/lib  # Path to VTK libraries
export VTK_VERSION=-9.5.1                             # Version suffix for libraries
```

**Finding your VTK installation:**
```bash
ls /opt/homebrew/Cellar/vtk/
```

### Verbose Build

To see what the build system is doing:

```bash
VERBOSE=1 cargo build
```

This shows:
- VTK detection and path resolution
- cxxbridge version checking
- Module linking information

## Advanced: Regenerating CXX Bridge Headers

⚠️ **Only needed if you change the `cxx` crate version or modify the bridge definitions.**

The C++ bridge headers (`.rs.h` files) must match the version of the `cxx` crate in `Cargo.lock`. If you get symbol mismatch errors after updating dependencies:

```bash
# Install cxxbridge-cmd matching your cxx version
cargo install cxxbridge-cmd  # Will match the version in Cargo.lock

# Navigate to vtk-rs subdirectory
cd vtk-rs

# Regenerate headers
bash generate_headers.sh
```

### 4. Run Examples

```bash
cargo run -p vtk-rs --example sphere_source
```

## Key Changes Made to Fix Runtime Issues

### Problem 1: VTK Module Initialization

**Issue**: VTK objects (like `vtkSphereSource`) were crashing with `SIGSEGV` at address `0x0` when calling `::New()`.

**Root Cause**: VTK's object factories were not initialized. VTK uses a factory pattern that requires module initialization before creating objects.

**Solution**: Created `vtk-rs/libvtkrs/src/vtk_init.cpp` that explicitly calls the AutoInit constructors:

```cpp
extern "C" void vtk_force_init() {
    vtkRenderingContextOpenGL2_AutoInit_Construct();
    vtkInteractionStyle_AutoInit_Construct();
    vtkRenderingFreeType_AutoInit_Construct();
    vtkRenderingOpenGL2_AutoInit_Construct();
    vtkRenderingGL2PSOpenGL2_AutoInit_Construct();
}
```

This function is called from Rust (in `lib.rs`) before any VTK objects are created:

```rust
pub fn init_vtk() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        unsafe { vtk_force_init(); }
    });
}
```

The `define_object!` macro was updated to call `crate::init_vtk()` in every `new()` method.

### Problem 2: CXX Bridge Version Mismatch

**Issue**: The binary was looking for symbols like `_cxxbridge1$192$vtk_sphere_source_new` but the static library had `_cxxbridge1$190$vtk_sphere_source_new`.

**Root Cause**: The checked-in C++ bridge headers (`.rs.h` files) were generated with an older version of cxx (1.0.190), but the project was using cxx 1.0.192.

**Solution**: Regenerated all bridge headers using `generate_headers.sh` with the correct cxx version. The version number in the symbol name must match between:
- The cxx crate version in `Cargo.lock`
- The cxxbridge-cmd tool version used to generate headers
- The generated `.rs.h` files in `vtk-rs/libvtkrs/include/`

### Problem 3: VTK AutoInit Compile Definitions

**Issue**: VTK module initialization requires specific compile-time definitions.

**Solution**: Added compile definitions to `CMakeLists.txt`:

```cmake
target_compile_definitions(vtkrs PRIVATE
    "vtkRenderingContext2D_AUTOINIT=1(vtkRenderingContextOpenGL2)"
    "vtkRenderingCore_AUTOINIT=3(vtkInteractionStyle,vtkRenderingFreeType,vtkRenderingOpenGL2)"
    "vtkRenderingOpenGL2_AUTOINIT=1(vtkRenderingGL2PSOpenGL2)"
)
```

## Troubleshooting

### Segmentation Fault on Object Creation

If you get a segfault when creating VTK objects:
1. Ensure VTK environment variables are set correctly
2. Regenerate CXX bridge headers to match your cxx version
3. Do a clean rebuild: `cargo clean && cargo build -p vtk-rs`

### Symbol Not Found / Undefined Symbol

This indicates a CXX version mismatch:
1. Check your cxx version in `Cargo.lock`
2. Reinstall cxxbridge-cmd with matching version
3. Run `bash generate_headers.sh` in the `vtk-rs` directory
4. Rebuild completely

### CMake Can't Find VTK

Ensure VTK is installed and the `VTK_DIR` environment variable points to the lib directory containing VTK CMake files.

## Testing

Run the test suite:
```bash
export VTK_DIR=/opt/homebrew/Cellar/vtk/9.5.2_2/lib
export VTK_VERSION=-9.5.1
cargo test -p vtk-rs
```

## Notes

- The environment variables must be set for every build session
- If you update the cxx crate version, you must regenerate headers
- VTK initialization happens automatically on first object creation
- The initialization is thread-safe (uses `std::sync::Once`)

## Additional Resources

- [VTK Documentation](https://vtk.org/doc/nightly/html/)
- [CXX Documentation](https://cxx.rs/)
- Project README: [README.md](README.md)
