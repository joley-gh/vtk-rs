// Direct extern "C" bindings (no cxx bridge)
#[repr(C)]
pub struct vtkLineSource {
    _private: [u8; 0],
}

// Opaque type for algorithm output (used internally)
#[repr(C)]
struct VtkAlgorithmOutputOpaque {
    _private: [u8; 0],
}

// Ensure the vtkrs static library is linked
#[link(name = "vtkrs", kind = "static")]
extern "C" {
    fn line_source_new() -> *mut vtkLineSource;
    fn line_source_delete(line_source: *mut vtkLineSource);
    fn line_source_set_point1(line_source: *mut vtkLineSource, x: f64, y: f64, z: f64);
    fn line_source_get_point1(
        line_source: *mut vtkLineSource,
        x: *mut f64,
        y: *mut f64,
        z: *mut f64
    );
    fn line_source_set_point2(line_source: *mut vtkLineSource, x: f64, y: f64, z: f64);
    fn line_source_get_point2(
        line_source: *mut vtkLineSource,
        x: *mut f64,
        y: *mut f64,
        z: *mut f64
    );
    fn line_source_set_resolution(line_source: *mut vtkLineSource, resolution: i32);
    fn line_source_get_resolution(line_source: *mut vtkLineSource) -> i32;
    fn line_source_get_output_port(
        line_source: *mut vtkLineSource
    ) -> *mut VtkAlgorithmOutputOpaque;
}

/// Safe wrapper for vtkLineSource
pub struct LineSource {
    ptr: *mut vtkLineSource,
}

impl LineSource {
    pub fn new() -> Self {
        crate::init_vtk();
        let ptr = unsafe { line_source_new() };
        if ptr.is_null() {
            panic!("Failed to create LineSource");
        }
        Self { ptr }
    }

    pub fn as_mut_ptr(&mut self) -> *mut vtkLineSource {
        self.ptr
    }
    /// Set the first point of the line.
    pub fn set_point1(&mut self, x: f64, y: f64, z: f64) {
        unsafe {
            line_source_set_point1(self.ptr, x, y, z);
        }
    }

    /// Get the first point of the line.
    pub fn get_point1(&mut self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        unsafe {
            line_source_get_point1(self.ptr, &mut x, &mut y, &mut z);
        }
        (x, y, z)
    }

    /// Set the second point of the line.
    pub fn set_point2(&mut self, x: f64, y: f64, z: f64) {
        unsafe {
            line_source_set_point2(self.ptr, x, y, z);
        }
    }

    /// Get the second point of the line.
    pub fn get_point2(&mut self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        unsafe {
            line_source_get_point2(self.ptr, &mut x, &mut y, &mut z);
        }
        (x, y, z)
    }

    /// Set the number of segments in the line.
    /// A resolution of 1 creates a simple line between the two points.
    /// Higher resolutions create additional intermediate points.
    pub fn set_resolution(&mut self, resolution: i32) {
        unsafe {
            line_source_set_resolution(self.ptr, resolution);
        }
    }

    /// Get the number of segments in the line.
    pub fn get_resolution(&mut self) -> i32 {
        unsafe { line_source_get_resolution(self.ptr) }
    }

    /// Get the output port for connecting to mappers.
    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = line_source_get_output_port(self.ptr);
            crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void)
        }
    }

    /// Calculate the distance between point1 and point2.
    pub fn get_length(&mut self) -> f64 {
        let (x1, y1, z1) = self.get_point1();
        let (x2, y2, z2) = self.get_point2();
        let dx = x2 - x1;
        let dy = y2 - y1;
        let dz = z2 - z1;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl Drop for LineSource {
    fn drop(&mut self) {
        unsafe {
            line_source_delete(self.ptr);
        }
    }
}
