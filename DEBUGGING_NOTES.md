# VTK-RS Runtime Issues

## Current Status

The vtk-rs bindings compile successfully but crash at runtime with a segmentation fault (SIGSEGV) when attempting to create VTK objects.

## Symptoms

- ✅ Code compiles without errors
- ✅ VTK libraries are properly linked (verified with `otool -L`)
- ✅ VTK 9.5.1 is installed via Homebrew at `/opt/homebrew/opt/vtk/`
- ❌ Runtime crash when creating VTK objects (e.g., `SphereSource::new()`)
- ❌ Even unit tests in the library segfault

## Investigation Steps Taken

1. **Library Linking**: Verified that all VTK libraries are properly linked:
   ```bash
   otool -L target/debug/examples/sphere_source
   ```
   Shows correct linking to VTK 9.5.1 dylibs.

2. **Library Files**: Confirmed that library files exist and are accessible at:
   `/opt/homebrew/opt/vtk/lib/libvtkCommonCore-9.5.9.5.dylib`

3. **Test Execution**: Library tests also crash:
   ```bash
   cargo test -p vtk-rs --lib vtk_sphere_source::test::get_set_radius
   # Result: SIGSEGV
   ```

## Potential Causes

1. **VTK Initialization**: VTK may require explicit initialization before object creation
2. **Reference Counting**: The C++ bridge may have issues with VTK's reference counting system
3. **Memory Management**: The way `Pin<&mut T>` is used might not align with VTK's object model
4. **Version Mismatch**: The compiled bindings might expect different VTK behavior
5. **Runtime Dependencies**: Missing environment variables or configuration

## Code Analysis

### C++ Bridge (`vtk_sphere_source.cpp`)
```cpp
vtkSphereSource* vtk_sphere_source_new() {
    return vtkSphereSource::New();  // Creates object with refcount=1
}

void vtk_sphere_source_delete(vtkSphereSource& sphere_source) {
    sphere_source.Delete();  // Decrements refcount
}
```

### Rust Macro (`macros.rs`)
The `define_object!` macro creates a wrapper that:
- Calls the `new` function and wraps the raw pointer in `Pin<&'static mut T>`
- Calls `delete` in the `Drop` implementation

**Issue**: The lifetime `'static` on a mutable reference is semantically incorrect. The pointer is not actually static, and this might be causing undefined behavior.

## Suggested Fixes

### 1. Check VTK Initialization Requirements
Some VTK applications require initialization. Try adding:
```cpp
// In a C++ init function
vtkObject::GlobalWarningDisplayOff();
// Or check if VTK needs vtkOutputWindow configuration
```

### 2. Fix Lifetime Management
The current code uses:
```rust
let pinned = unsafe { core::pin::Pin::new_unchecked(&mut *($new_func)()) };
```

This creates a `Pin<&'static mut T>` from a temporary, which is unsound. Consider:
- Using `Box::new()` to get owned pointers
- Proper lifetime management instead of `'static`
- Using `NonNull<T>` and manual memory management

### 3. Reference Counting
VTK uses reference counting. Consider calling `Register()` after `New()`:
```cpp
vtkSphereSource* vtk_sphere_source_new() {
    auto* obj = vtkSphereSource::New();
    obj->Register(nullptr);  // Increment refcount
    return obj;
}
```

### 4. Test in Pure C++
Create a minimal C++ test to verify VTK works:
```cpp
#include <vtkSphereSource.h>
int main() {
    auto* sphere = vtkSphereSource::New();
    sphere->SetRadius(5.0);
    double r = sphere->GetRadius();
    sphere->Delete();
    return 0;
}
```

Compile and run:
```bash
c++ -std=c++17 test.cpp -I/opt/homebrew/opt/vtk/include/vtk-9.5 \
    -L/opt/homebrew/opt/vtk/lib -lvtkCommonCore-9.5 -lvtkFiltersSources-9.5 \
    -o test && ./test
```

## Workaround

The example has been modified to document the issue without crashing:
```bash
cargo run -p vtk-rs --example sphere_source
```

This will display documentation about the intended behavior without executing the crashing VTK code.

## Next Steps

1. Test VTK in pure C++ to isolate Rust binding issues
2. Review memory management in the `define_object!` macro
3. Check if VTK requires initialization calls
4. Consider using `cxx::UniquePtr` for automatic memory management
5. Review similar projects (e.g., vtk-rs forks, VTK Python bindings) for patterns
