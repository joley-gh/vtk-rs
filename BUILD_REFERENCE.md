# vtk-rs Build System Reference

## Quick Commands

```bash
# Standard build
cargo build

# Verbose build (shows VTK detection)
VERBOSE=1 cargo build

# Run example
cargo run --example sphere_source

# Clean and rebuild
cargo clean -p vtk-rs && cargo build
```

## Environment Variables

| Variable | Purpose | Required | Example |
|----------|---------|----------|---------|
| `VTK_DIR` | Path to VTK libraries | No* | `/opt/homebrew/Cellar/vtk/9.5.2_2/lib` |
| `VTK_VERSION` | Library version suffix | No* | `-9.5.1` |
| `VERBOSE` | Enable build logging | No | `1` or `true` |

*Auto-detected if not set. The build system searches:
- `/usr/local/lib/`
- `/opt/homebrew/lib/`
- `/opt/homebrew/Cellar/vtk/`

## Troubleshooting

### Symbol Mismatch Error

**Error**: `undefined symbol: cxxbridge1$192$...` or similar

**Cause**: CXX bridge headers don't match the `cxx` crate version

**Solution**:
```bash
cd vtk-rs
cargo install cxxbridge-cmd  # Installs matching version
bash generate_headers.sh
```

### VTK Not Found

**Error**: `Could not find suitable installation directory`

**Solution**: Set `VTK_DIR` explicitly:
```bash
export VTK_DIR=/path/to/vtk/lib
cargo build
```

### SIGSEGV on Object Creation

**Error**: Crash when creating VTK objects

**Cause**: VTK modules not initialized (should be fixed now)

**Verify**: Check that `vtk_init.cpp` is being compiled:
```bash
VERBOSE=1 cargo build 2>&1 | grep vtk_init
```

### CMake Build Failure

**Error**: CMake can't find VTK

**Check**:
1. VTK is installed: `ls /opt/homebrew/Cellar/vtk/`
2. CMake can find it: `cmake --find-package -DNAME=VTK -DCOMPILER_ID=GNU -DLANGUAGE=C -DMODE=EXIST`

## Build System Architecture

```
build.rs
  ├── Validates environment (VTK_DIR, cxxbridge)
  ├── Calls CMake to build libvtkrs.a
  │   └── libvtkrs/CMakeLists.txt
  │       ├── Finds VTK installation
  │       ├── Compiles C++ sources (including vtk_init.cpp)
  │       └── Creates static library
  └── Links VTK modules using vtk-rs-link
```

## Key Files

| File | Purpose |
|------|---------|
| `vtk-rs/build.rs` | Build script with validation |
| `vtk-rs/libvtkrs/src/vtk_init.cpp` | VTK module initialization |
| `vtk-rs/libvtkrs/CMakeLists.txt` | C++ bridge build config |
| `vtk-rs/src/lib.rs` | Rust initialization wrapper |
| `vtk-rs/src/macros.rs` | Auto-init in object creation |
| `vtk-rs/libvtkrs/include/*.rs.h` | CXX bridge headers |

## Development Workflow

### Adding a New VTK Class

1. Create Rust module: `vtk-rs/src/vtk_new_class.rs`
2. Create C++ bridge: `vtk-rs/libvtkrs/src/vtk_new_class.cpp`
3. Add to CMakeLists.txt sources
4. Regenerate headers: `bash generate_headers.sh`
5. Build: `cargo build`

### Updating CXX Version

1. Update `cxx` in `Cargo.toml`
2. Run `cargo update`
3. Install matching cxxbridge: `cargo install cxxbridge-cmd`
4. Regenerate headers: `cd vtk-rs && bash generate_headers.sh`
5. Rebuild: `cargo build`

## Performance

- **Cold build**: ~30-40 seconds (includes CMake + Rust compilation)
- **Incremental**: ~2-5 seconds (Rust only)
- **Clean rebuild of vtk-rs**: ~20 seconds

## Continuous Integration Tips

```yaml
# .github/workflows/ci.yml example
- name: Install VTK
  run: brew install vtk

- name: Build
  env:
    VTK_DIR: /opt/homebrew/Cellar/vtk/9.5.2_2/lib
    VTK_VERSION: -9.5.1
  run: cargo build --verbose
```
