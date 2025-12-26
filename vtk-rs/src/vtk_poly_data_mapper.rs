#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_poly_data_mapper.h");
        include!("vtk_algorithm_output.h");
        include!("vtk_mapper.h");

        type vtkPolyDataMapper;
        type vtkAlgorithmOutput;
        type vtkMapper;

        fn poly_data_mapper_new() -> *mut vtkPolyDataMapper;
        fn poly_data_mapper_delete(pdm: Pin<&mut vtkPolyDataMapper>);
        unsafe fn poly_data_mapper_set_input_connection(
            mapper: Pin<&mut vtkPolyDataMapper>,
            output: *mut vtkAlgorithmOutput
        );
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkPolyDataMapper.html",
    @name PolyDataMapper, ffi::vtkPolyDataMapper,
    @new ffi::poly_data_mapper_new,
    // @clone ffi::poly_data_mapper_clone,
    @delete ffi::poly_data_mapper_delete,
    @inherit vtkPolyDataMapper
);

impl PolyDataMapper {
    /// Sets the input connection from any VTK algorithm output port.
    ///
    /// # Example
    /// ```no_run
    /// # use vtk_rs as vtk;
    /// let mut sphere = vtk::SphereSource::new();
    /// let mut mapper = vtk::PolyDataMapper::new();
    ///
    /// // AlgorithmOutputPort automatically converts with .into()
    /// mapper.set_input_connection(sphere.get_output_port());
    /// ```
    pub fn set_input_connection(&mut self, output: impl Into<*mut std::ffi::c_void>) {
        unsafe {
            let ptr = output.into();
            let algo_output = ptr as *mut ffi::vtkAlgorithmOutput;
            ffi::poly_data_mapper_set_input_connection(self.ptr.as_mut(), algo_output);
        }
    }

    /// Get raw pointer for FFI (internal use)
    pub(crate) fn as_raw_ptr(&mut self) -> *mut ffi::vtkMapper {
        self.as_mut_ptr() as *mut ffi::vtkMapper
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkPolyDataMapper`](https://vtk.org/doc/nightly/html/classvtkPolyDataMapper.html)
#[allow(non_camel_case_types)]
pub trait vtkPolyDataMapper: private::Sealed {}
