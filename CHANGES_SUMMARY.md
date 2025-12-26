# Summary of Changes to Fix vtk-rs

This document summarizes the essential changes made to get the vtk-rs bindings working.

## Problem

The `sphere_source.rs` example was crashing with `SIGSEGV` (segmentation fault) when calling `vtkSphereSource::new()`. The crash was caused by:

1. **VTK module initialization not happening** - VTK's AutoInit system wasn't running
2. **CXX bridge version mismatch** - Headers were generated with cxx 1.0.190 but code used 1.0.192
3. **Missing compile definitions** - CMake wasn't defining the AutoInit macros

## Essential Changes

### 1. VTK Module Initialization (NEW FILE)

**File**: `vtk-rs/libvtkrs/src/vtk_init.cpp`

This new file explicitly calls VTK's AutoInit constructors. This is necessary because static library initializers don't run automatically.

```cpp
extern "C" void vtk_force_init() {
    vtkRenderingContextOpenGL2_AutoInit_Construct();
    vtkInteractionStyle_AutoInit_Construct();
    vtkRenderingFreeType_AutoInit_Construct();
    vtkRenderingOpenGL2_AutoInit_Construct();
    vtkRenderingGL2PSOpenGL2_AutoInit_Construct();
}
```

### 2. Rust Initialization Wrapper

**File**: `vtk-rs/src/lib.rs`

Added a thread-safe wrapper to call the C++ initialization:

```rust
extern "C" {
    fn vtk_force_init();
}

pub fn init_vtk() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        unsafe { vtk_force_init(); }
    });
}
```

### 3. Auto-Initialize in Object Creation

**File**: `vtk-rs/src/macros.rs`

Modified the `define_object!` macro to call initialization before creating objects:

```rust
pub fn new() -> Self {
    crate::init_vtk();  // Ensure VTK is initialized
    Self {
        ptr: unsafe { [<$snake_name _new>]() },
    }
}
```

### 4. CMake Build Configuration

**File**: `vtk-rs/libvtkrs/CMakeLists.txt`

Added:
- `vtk_init.cpp` to the sources list
- Compile definitions for VTK AutoInit macros

```cmake
set(SOURCES
  "src/vtk_init.cpp"  # NEW
  ...
)

target_compile_definitions(vtkrs PRIVATE
  vtkRenderingCore_AUTOINIT="2(vtkInteractionStyle,vtkRenderingOpenGL2)"
  vtkRenderingContext2D_AUTOINIT="1(vtkRenderingContextOpenGL2)"
  vtkRenderingVolume_AUTOINIT="1(vtkRenderingVolumeOpenGL2)"
  vtkRenderingOpenGL2_AUTOINIT="1(vtkRenderingGL2PSOpenGL2)"
)
```

### 5. Regenerated CXX Bridge Headers

**Files**: All `.rs.h` files in `vtk-rs/libvtkrs/include/`

Regenerated with cxxbridge 1.0.192 to fix symbol mismatch. The symbols changed from:
- Old: `cxxbridge1$190$...`
- New: `cxxbridge1$192$...`

### 6. Enhanced Build Script

**File**: `vtk-rs/build.rs`

Added validation and helpful error messages:
- Checks if VTK_DIR is valid
- Validates cxxbridge installation
- Provides verbose logging with `VERBOSE=1`

## Why These Changes Are Necessary

### vtk_init.cpp is Required

While `vtk_module_autoinit` in CMakeLists.txt generates the AutoInit code, static libraries don't execute their static initializers unless explicitly referenced. The `vtk_init.cpp` file provides that explicit reference.

### Header Regeneration is Critical

The cxx crate uses versioned symbols (e.g., `cxxbridge1$192$`). If headers are generated with a different version than what's in Cargo.lock, you get linker errors. Always regenerate headers after updating the cxx dependency.

### Thread-Safe Initialization

VTK's AutoInit constructors must be called exactly once before using any VTK objects. The `std::sync::Once` ensures thread-safety.

## Testing

```bash
# Build with verbose output
VERBOSE=1 cargo build

# Run the example
cargo run --example sphere_source
```

Expected output:
```
Creating VTK SphereSource example...
✓ Created SphereSource object
✓ Set center to [0, 0, 0] and radius to 5.0
...
✓ Example completed successfully!
```

## File Change Statistics

- **Modified**: 23 files (mostly regenerated headers)
- **New**: 4 files (vtk_init.cpp, sphere_source.rs example, documentation)
- **Critical**: 5 files for core functionality (vtk_init.cpp, lib.rs, macros.rs, CMakeLists.txt, build.rs)
