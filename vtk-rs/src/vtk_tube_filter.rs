/// VTK TubeFilter - Creates tubes around lines
///
/// This module wraps vtkTubeFilter to create 3D tubular representations of lines.
/// Perfect for visualizing beam elements in FEM structures.

use std::ffi::c_void;

// Direct extern "C" FFI bindings
extern "C" {
    fn tube_filter_new() -> *mut vtkTubeFilter;
    fn tube_filter_delete(filter: *mut vtkTubeFilter);
    fn tube_filter_set_input_connection(filter: *mut vtkTubeFilter, input: *mut c_void);
    fn tube_filter_get_output_port(filter: *mut vtkTubeFilter) -> *mut c_void;
    fn tube_filter_set_radius(filter: *mut vtkTubeFilter, radius: f64);
    fn tube_filter_get_radius(filter: *mut vtkTubeFilter) -> f64;
    fn tube_filter_set_number_of_sides(filter: *mut vtkTubeFilter, sides: i32);
    fn tube_filter_get_number_of_sides(filter: *mut vtkTubeFilter) -> i32;
    fn tube_filter_set_radius_factor(filter: *mut vtkTubeFilter, factor: f64);
    fn tube_filter_get_radius_factor(filter: *mut vtkTubeFilter) -> f64;
    fn tube_filter_set_capping(filter: *mut vtkTubeFilter, capping: i32);
    fn tube_filter_get_capping(filter: *mut vtkTubeFilter) -> i32;
}

#[repr(C)]
pub struct vtkTubeFilter {
    _private: [u8; 0],
}

/// TubeFilter creates 3D tubes around lines
///
/// # Example
/// ```no_run
/// use vtk_rs::*;
///
/// let mut line = LineSource::new();
/// line.set_point1(0.0, 0.0, 0.0);
/// line.set_point2(1.0, 0.0, 0.0);
///
/// let mut tube = TubeFilter::new();
/// tube.set_input_connection(LineSource::get_output_port(&mut line));
/// tube.set_radius(0.1);
/// tube.set_number_of_sides(16);
///
/// let mut mapper = PolyDataMapper::new();
/// mapper.set_input_connection(TubeFilter::get_output_port(&mut tube));
/// ```
pub struct TubeFilter {
    ptr: *mut vtkTubeFilter,
}

impl TubeFilter {
    /// Create a new TubeFilter
    pub fn new() -> Self {
        let ptr = unsafe { tube_filter_new() };
        assert!(!ptr.is_null(), "Failed to create vtkTubeFilter");
        Self { ptr }
    }

    /// Set the input connection from a line source or other algorithm
    pub fn set_input_connection(&mut self, output: impl Into<*mut c_void>) {
        unsafe {
            let ptr = output.into();
            tube_filter_set_input_connection(self.ptr, ptr);
        }
    }

    /// Get the output port for connecting to mappers
    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = tube_filter_get_output_port(self.ptr);
            crate::AlgorithmOutputPort::from_raw(ptr)
        }
    }

    /// Set the radius of the tubes
    ///
    /// Default is 0.5
    pub fn set_radius(&mut self, radius: f64) {
        unsafe {
            tube_filter_set_radius(self.ptr, radius);
        }
    }

    /// Get the current tube radius
    pub fn get_radius(&self) -> f64 {
        unsafe { tube_filter_get_radius(self.ptr) }
    }

    /// Set the number of sides for the tube cross-section
    ///
    /// More sides = smoother tubes, but higher polygon count.
    /// Default is 3. Typical values: 6-16 for beams, 20+ for smooth cylinders.
    pub fn set_number_of_sides(&mut self, sides: i32) {
        unsafe {
            tube_filter_set_number_of_sides(self.ptr, sides);
        }
    }

    /// Get the number of sides
    pub fn get_number_of_sides(&self) -> i32 {
        unsafe { tube_filter_get_number_of_sides(self.ptr) }
    }

    /// Set the radius factor for variable radius tubes
    ///
    /// This scales the radius based on data values
    pub fn set_radius_factor(&mut self, factor: f64) {
        unsafe {
            tube_filter_set_radius_factor(self.ptr, factor);
        }
    }

    /// Get the radius factor
    pub fn get_radius_factor(&self) -> f64 {
        unsafe { tube_filter_get_radius_factor(self.ptr) }
    }

    /// Enable/disable capping of tube ends
    ///
    /// When true, tubes are closed at the ends
    pub fn set_capping(&mut self, capping: bool) {
        unsafe {
            tube_filter_set_capping(self.ptr, if capping { 1 } else { 0 });
        }
    }

    /// Check if capping is enabled
    pub fn get_capping(&self) -> bool {
        unsafe { tube_filter_get_capping(self.ptr) != 0 }
    }

    /// Get the raw pointer (for internal use)
    pub fn as_ptr(&self) -> *mut vtkTubeFilter {
        self.ptr
    }
}

impl Drop for TubeFilter {
    fn drop(&mut self) {
        unsafe {
            tube_filter_delete(self.ptr);
        }
    }
}

impl Default for TubeFilter {
    fn default() -> Self {
        Self::new()
    }
}
