#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_cone_source.h");
        include!("vtk_algorithm_output.h");

        type vtkConeSource;
        type vtkAlgorithmOutput;

        fn vtk_cone_source_new() -> *mut vtkConeSource;
        fn vtk_cone_source_delete(cone: Pin<&mut vtkConeSource>);

        fn vtk_cone_source_set_radius(cone: Pin<&mut vtkConeSource>, radius: f64);
        fn vtk_cone_source_get_radius(cone: &vtkConeSource) -> f64;

        fn vtk_cone_source_set_height(cone: Pin<&mut vtkConeSource>, height: f64);
        fn vtk_cone_source_get_height(cone: &vtkConeSource) -> f64;

        fn vtk_cone_source_set_resolution(cone: Pin<&mut vtkConeSource>, resolution: i32);
        fn vtk_cone_source_get_resolution(cone: &vtkConeSource) -> i32;

        fn vtk_cone_source_set_direction(cone: Pin<&mut vtkConeSource>, x: f64, y: f64, z: f64);
        fn vtk_cone_source_get_direction(
            cone: &vtkConeSource,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );

        fn vtk_cone_source_set_center(cone: Pin<&mut vtkConeSource>, x: f64, y: f64, z: f64);
        fn vtk_cone_source_get_center(cone: &vtkConeSource, x: &mut f64, y: &mut f64, z: &mut f64);

        fn vtk_cone_source_set_capping(cone: Pin<&mut vtkConeSource>, cap: bool);
        fn vtk_cone_source_get_capping(cone: &vtkConeSource) -> bool;

        fn vtk_cone_source_set_angle(cone: Pin<&mut vtkConeSource>, angle: f64);
        fn vtk_cone_source_get_angle(cone: &vtkConeSource) -> f64;

        unsafe fn vtk_cone_source_get_output_port(
            cone: Pin<&mut vtkConeSource>
        ) -> *mut vtkAlgorithmOutput;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkConeSource.html",
    @name ConeSource, ffi::vtkConeSource,
    @new ffi::vtk_cone_source_new,
    @delete ffi::vtk_cone_source_delete
);

unsafe impl Send for ConeSource {}
unsafe impl Sync for ConeSource {}

impl ConeSource {
    /// Set the base radius of the cone.
    pub fn set_radius(&mut self, radius: f64) {
        ffi::vtk_cone_source_set_radius(self.ptr.as_mut(), radius);
    }

    /// Get the base radius of the cone.
    pub fn get_radius(&self) -> f64 {
        ffi::vtk_cone_source_get_radius(&self.ptr.as_ref())
    }

    /// Set the height of the cone (along its axis).
    pub fn set_height(&mut self, height: f64) {
        ffi::vtk_cone_source_set_height(self.ptr.as_mut(), height);
    }

    /// Get the height of the cone.
    pub fn get_height(&self) -> f64 {
        ffi::vtk_cone_source_get_height(&self.ptr.as_ref())
    }

    /// Set the number of facets used to represent the cone.
    pub fn set_resolution(&mut self, resolution: i32) {
        ffi::vtk_cone_source_set_resolution(self.ptr.as_mut(), resolution);
    }

    /// Get the resolution (number of facets).
    pub fn get_resolution(&self) -> i32 {
        ffi::vtk_cone_source_get_resolution(&self.ptr.as_ref())
    }

    /// Set the orientation vector of the cone (from base to apex).
    pub fn set_direction(&mut self, x: f64, y: f64, z: f64) {
        ffi::vtk_cone_source_set_direction(self.ptr.as_mut(), x, y, z);
    }

    /// Get the direction vector of the cone.
    pub fn get_direction(&self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::vtk_cone_source_get_direction(&self.ptr.as_ref(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    /// Set the center position of the cone.
    pub fn set_center(&mut self, x: f64, y: f64, z: f64) {
        ffi::vtk_cone_source_set_center(self.ptr.as_mut(), x, y, z);
    }

    /// Get the center position of the cone.
    pub fn get_center(&self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::vtk_cone_source_get_center(&self.ptr.as_ref(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    /// Enable or disable base capping. When enabled, the base is closed with a polygon.
    pub fn set_capping(&mut self, cap: bool) {
        ffi::vtk_cone_source_set_capping(self.ptr.as_mut(), cap);
    }

    /// Check if base capping is enabled.
    pub fn get_capping(&self) -> bool {
        ffi::vtk_cone_source_get_capping(&self.ptr.as_ref())
    }

    /// Set the cone angle in degrees.
    pub fn set_angle(&mut self, angle: f64) {
        ffi::vtk_cone_source_set_angle(self.ptr.as_mut(), angle);
    }

    /// Get the cone angle in degrees.
    pub fn get_angle(&self) -> f64 {
        ffi::vtk_cone_source_get_angle(&self.ptr.as_ref())
    }

    /// Get the output port for connecting to a mapper or filter.
    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = ffi::vtk_cone_source_get_output_port(self.ptr.as_mut());
            crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void)
        }
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkConeSource`](https://vtk.org/doc/nightly/html/classvtkConeSource.html)
#[allow(non_camel_case_types)]
pub trait vtkConeSource: private::Sealed {}
