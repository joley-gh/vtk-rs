#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_parametric_torus.h");
        include!("vtk_parametric_function.h");

        type vtkParametricTorus;
        type vtkParametricFunction;

        fn vtk_parametric_torus_new() -> *mut vtkParametricTorus;
        fn vtk_parametric_torus_delete(ptr: Pin<&mut vtkParametricTorus>);
        fn parametric_torus_set_ring_radius(torus: Pin<&mut vtkParametricTorus>, radius: f64);
        fn parametric_torus_get_ring_radius(torus: &vtkParametricTorus) -> f64;
        fn parametric_torus_set_cross_section_radius(
            torus: Pin<&mut vtkParametricTorus>,
            radius: f64
        );
        fn parametric_torus_get_cross_section_radius(torus: &vtkParametricTorus) -> f64;
        unsafe fn parametric_torus_as_parametric_function(
            torus: Pin<&mut vtkParametricTorus>
        ) -> *mut vtkParametricFunction;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkParametricTorus.html",
    @name ParametricTorus, ffi::vtkParametricTorus,
    @new ffi::vtk_parametric_torus_new,
    @delete ffi::vtk_parametric_torus_delete,
    @inherit vtkObjectBase
);

impl ParametricTorus {
    /// Set the radius from the center to the middle of the ring
    #[doc(alias = "SetRingRadius")]
    pub fn set_ring_radius(&mut self, radius: f64) {
        ffi::parametric_torus_set_ring_radius(self.ptr.as_mut(), radius)
    }

    /// Get the radius from the center to the middle of the ring
    #[doc(alias = "GetRingRadius")]
    pub fn get_ring_radius(&self) -> f64 {
        ffi::parametric_torus_get_ring_radius(&self.ptr.as_ref())
    }

    /// Set the radius of the cross section of the ring
    #[doc(alias = "SetCrossSectionRadius")]
    pub fn set_cross_section_radius(&mut self, radius: f64) {
        ffi::parametric_torus_set_cross_section_radius(self.ptr.as_mut(), radius)
    }

    /// Get the radius of the cross section of the ring
    #[doc(alias = "GetCrossSectionRadius")]
    pub fn get_cross_section_radius(&self) -> f64 {
        ffi::parametric_torus_get_cross_section_radius(&self.ptr.as_ref())
    }

    /// Get a pointer to the base vtkParametricFunction for use with ParametricFunctionSource
    pub fn as_parametric_function(&mut self) -> *mut std::ffi::c_void {
        unsafe {
            ffi::parametric_torus_as_parametric_function(self.ptr.as_mut()) as *mut std::ffi::c_void
        }
    }
}
