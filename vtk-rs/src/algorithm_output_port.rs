/// A wrapper for VTK algorithm output ports that enables ergonomic type conversion.
///
/// This type wraps a raw pointer to `vtkAlgorithmOutput` and implements `Into<*mut c_void>`
/// to allow seamless passing to methods like `PolyDataMapper::set_input_connection()`.
///
/// # Example
/// ```no_run
/// # use vtk_rs as vtk;
/// let mut sphere_source = vtk::SphereSource::new();
/// let mut mapper = vtk::PolyDataMapper::new();
///
/// // No explicit cast needed - Into trait is called automatically
/// mapper.set_input_connection(sphere_source.get_output_port());
/// ```
pub struct AlgorithmOutputPort {
    ptr: *mut std::ffi::c_void,
}

impl AlgorithmOutputPort {
    /// Creates a new AlgorithmOutputPort from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be a valid `vtkAlgorithmOutput*` pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut std::ffi::c_void) -> Self {
        Self { ptr }
    }
}

impl From<AlgorithmOutputPort> for *mut std::ffi::c_void {
    #[inline]
    fn from(port: AlgorithmOutputPort) -> Self {
        port.ptr
    }
}

// Also implement Into for reference to avoid moving
impl From<&AlgorithmOutputPort> for *mut std::ffi::c_void {
    #[inline]
    fn from(port: &AlgorithmOutputPort) -> Self {
        port.ptr
    }
}
