## VTK-RS Wrapper Development Roadmap

### Priority 1: Core Geometric Primitives
Essential VTK source objects for building 3D visualizations.

- [ ] **vtkConeSource** - Cone primitive
  - set_radius, set_height, set_resolution, set_direction, set_center
  - Most common for arrows/direction indicators
  
- [ ] **vtkCylinderSource** - Cylinder primitive
  - set_radius, set_height, set_resolution, set_center
  - Useful for pipes, columns, axes
  
- [ ] **vtkCubeSource** - Cube/Box primitive
  - set_x_length, set_y_length, set_z_length, set_center, set_bounds
  - Basic building block for many applications
  
- [ ] **vtkPlaneSource** - Plane primitive
  - set_origin, set_point1, set_point2, set_resolution, set_center, set_normal
  - Fundamental for cutting planes, ground planes, reference surfaces

- [ ] **vtkDiskSource** - Disk/Circle primitive
  - set_inner_radius, set_outer_radius, set_radial_resolution, set_circumferential_resolution
  - Useful for circular surfaces, end caps

- [ ] **vtkArrowSource** - Arrow primitive
  - set_tip_length, set_tip_radius, set_tip_resolution, set_shaft_radius, set_shaft_resolution
  - Direction indicators, vector fields

- [ ] **vtkRegularPolygonSource** - Regular polygon primitive
  - set_number_of_sides, set_radius, set_center, set_normal
  - Hexagons, pentagons, etc.

### Priority 2: Interaction Widgets & Selection
Advanced user interaction capabilities.

- [ ] **vtkRubberBandPicker** - Rubber band selection
  - Interactive area selection with visual feedback
  - Essential for multi-object selection interfaces

- [ ] **vtkAreaPicker** - Area/frustum picking
  - Pick all objects within a 2D rectangular region
  - Works with rubber band interactions

- [ ] **vtkPointPicker** - Point picking
  - Pick individual points from point clouds
  - Complements existing CellPicker and PropPicker

- [ ] **vtkWorldPointPicker** - World coordinate picking
  - Convert screen coordinates to 3D world coordinates
  - Essential for accurate 3D placement

### Priority 3: Advanced Sources & Utilities

- [ ] **vtkParametricFunctionSource** - Parametric surfaces
  - Torus, Mobius strip, Klein bottle, etc.
  - Advanced mathematical surfaces

- [ ] **vtkSuperquadricSource** - Superquadrics
  - Rounded boxes, pillows, etc.
  - Smooth transitions between shapes

- [ ] **vtkTextSource** - 3D text
  - Extruded text geometry
  - Labels and annotations

### Priority 4: Actors & Annotations

- [ ] **vtkTextActor** - 2D text overlay
  - Screen-space text rendering
  - HUD elements, labels

- [ ] **vtkScalarBarActor** - Color bar legends
  - Visualize color mappings
  - Essential for scientific visualization

- [ ] **vtkLegendBoxActor** - Legend boxes
  - Multi-item legends
  - Scene annotations

### Priority 5: Data Structures & Filters

- [ ] **vtkImageData** - Structured grid data
  - Volumetric data, images
  - Medical imaging, CFD

- [ ] **vtkUnstructuredGrid** - Unstructured mesh data
  - FEM meshes, arbitrary topology
  - Finite element analysis

- [ ] **vtkContourFilter** - Isosurface extraction
  - Extract surfaces at constant values
  - Volumetric visualization

- [ ] **vtkClipPolyData** - Clipping filter
  - Cut meshes with planes
  - Cross-sections

### Excluded (Application-Specific)
These are application-specific features, not core VTK wrapper functionality.
Move to separate application project when needed.

- ~~FEM-specific node deletion/modification~~
- ~~FEM beam management~~
- ~~FEM material properties~~
- ~~FEM analysis integration~~
- ~~FEM boundary conditions~~

## Link Automatically with Cmake

1. Build (and link) `libvtkrs` with cmake
2. Build (but do not link) `vtkrs` with rustc
3. Link `vtkrs` library with `libvtkrs` and all Automatically obtained VTK libraries with Cmake

This should probably be done by creating a rustc-wrapper.
See https://doc.rust-lang.org/cargo/reference/config.html

```toml
# .cargo/config.toml
[build]
rustc = "rustc"
# This here should be crucial:
rustc-wrapper = "rustc_emit_and_cmake_linking"
```

We probably will have to write a wrapper which needs to be installed beforehand.
It is unclear if this is possible this way.

```toml
# Cargo.toml
[build-dependencies]
rustc_cmake_wrapper = { path="rustc_cmake_wrapper" }
```

Get inspiration by https://github.com/mozilla/sccache/.
