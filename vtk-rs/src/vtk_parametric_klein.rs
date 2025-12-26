#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_parametric_klein.h");
        include!("vtk_parametric_function.h");

        type vtkParametricKlein;
        type vtkParametricFunction;

        fn vtk_parametric_klein_new() -> *mut vtkParametricKlein;
        fn vtk_parametric_klein_delete(ptr: Pin<&mut vtkParametricKlein>);
        unsafe fn parametric_klein_as_parametric_function(
            klein: Pin<&mut vtkParametricKlein>
        ) -> *mut vtkParametricFunction;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkParametricKlein.html",
    @name ParametricKlein, ffi::vtkParametricKlein,
    @new ffi::vtk_parametric_klein_new,
    @delete ffi::vtk_parametric_klein_delete,
    @inherit vtkObjectBase
);

impl ParametricKlein {
    /// Get a pointer to the base vtkParametricFunction for use with ParametricFunctionSource
    pub fn as_parametric_function(&mut self) -> *mut std::ffi::c_void {
        unsafe {
            ffi::parametric_klein_as_parametric_function(self.ptr.as_mut()) as *mut std::ffi::c_void
        }
    }
}
