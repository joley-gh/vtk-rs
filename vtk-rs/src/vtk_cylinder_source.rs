#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_cylinder_source.h");
        include!("vtk_algorithm_output.h");

        type vtkCylinderSource;
        type vtkAlgorithmOutput;

        fn vtk_cylinder_source_new() -> *mut vtkCylinderSource;
        fn vtk_cylinder_source_delete(cylinder: Pin<&mut vtkCylinderSource>);

        fn vtk_cylinder_source_set_radius(cylinder: Pin<&mut vtkCylinderSource>, radius: f64);
        fn vtk_cylinder_source_get_radius(cylinder: &vtkCylinderSource) -> f64;

        fn vtk_cylinder_source_set_height(cylinder: Pin<&mut vtkCylinderSource>, height: f64);
        fn vtk_cylinder_source_get_height(cylinder: &vtkCylinderSource) -> f64;

        fn vtk_cylinder_source_set_resolution(
            cylinder: Pin<&mut vtkCylinderSource>,
            resolution: i32
        );
        fn vtk_cylinder_source_get_resolution(cylinder: &vtkCylinderSource) -> i32;

        fn vtk_cylinder_source_set_center(
            cylinder: Pin<&mut vtkCylinderSource>,
            x: f64,
            y: f64,
            z: f64
        );
        fn vtk_cylinder_source_get_center(
            cylinder: &vtkCylinderSource,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );

        fn vtk_cylinder_source_set_capping(cylinder: Pin<&mut vtkCylinderSource>, cap: bool);
        fn vtk_cylinder_source_get_capping(cylinder: &vtkCylinderSource) -> bool;

        unsafe fn vtk_cylinder_source_get_output_port(
            cylinder: Pin<&mut vtkCylinderSource>
        ) -> *mut vtkAlgorithmOutput;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkCylinderSource.html",
    @name CylinderSource, ffi::vtkCylinderSource,
    @new ffi::vtk_cylinder_source_new,
    @delete ffi::vtk_cylinder_source_delete
);

unsafe impl Send for CylinderSource {}
unsafe impl Sync for CylinderSource {}

impl CylinderSource {
    /// Set the radius of the cylinder.
    pub fn set_radius(&mut self, radius: f64) {
        ffi::vtk_cylinder_source_set_radius(self.ptr.as_mut(), radius);
    }

    /// Get the radius of the cylinder.
    pub fn get_radius(&self) -> f64 {
        ffi::vtk_cylinder_source_get_radius(&self.ptr.as_ref())
    }

    /// Set the height of the cylinder (along its axis).
    pub fn set_height(&mut self, height: f64) {
        ffi::vtk_cylinder_source_set_height(self.ptr.as_mut(), height);
    }

    /// Get the height of the cylinder.
    pub fn get_height(&self) -> f64 {
        ffi::vtk_cylinder_source_get_height(&self.ptr.as_ref())
    }

    /// Set the number of facets used to represent the cylinder.
    pub fn set_resolution(&mut self, resolution: i32) {
        ffi::vtk_cylinder_source_set_resolution(self.ptr.as_mut(), resolution);
    }

    /// Get the resolution (number of facets).
    pub fn get_resolution(&self) -> i32 {
        ffi::vtk_cylinder_source_get_resolution(&self.ptr.as_ref())
    }

    /// Set the center position of the cylinder.
    pub fn set_center(&mut self, x: f64, y: f64, z: f64) {
        ffi::vtk_cylinder_source_set_center(self.ptr.as_mut(), x, y, z);
    }

    /// Get the center position of the cylinder.
    pub fn get_center(&self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::vtk_cylinder_source_get_center(&self.ptr.as_ref(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    /// Enable or disable end capping. When enabled, the ends are closed with polygons.
    pub fn set_capping(&mut self, cap: bool) {
        ffi::vtk_cylinder_source_set_capping(self.ptr.as_mut(), cap);
    }

    /// Check if end capping is enabled.
    pub fn get_capping(&self) -> bool {
        ffi::vtk_cylinder_source_get_capping(&self.ptr.as_ref())
    }

    /// Get the output port for connecting to a mapper or filter.
    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = ffi::vtk_cylinder_source_get_output_port(self.ptr.as_mut());
            crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void)
        }
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkCylinderSource`](https://vtk.org/doc/nightly/html/classvtkCylinderSource.html)
#[allow(non_camel_case_types)]
pub trait vtkCylinderSource: private::Sealed {}
