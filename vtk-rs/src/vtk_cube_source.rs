#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_cube_source.h");
        include!("vtk_algorithm_output.h");

        type vtkCubeSource;
        type vtkAlgorithmOutput;

        fn vtk_cube_source_new() -> *mut vtkCubeSource;
        fn vtk_cube_source_delete(cube: Pin<&mut vtkCubeSource>);

        fn vtk_cube_source_set_x_length(cube: Pin<&mut vtkCubeSource>, length: f64);
        fn vtk_cube_source_get_x_length(cube: &vtkCubeSource) -> f64;

        fn vtk_cube_source_set_y_length(cube: Pin<&mut vtkCubeSource>, length: f64);
        fn vtk_cube_source_get_y_length(cube: &vtkCubeSource) -> f64;

        fn vtk_cube_source_set_z_length(cube: Pin<&mut vtkCubeSource>, length: f64);
        fn vtk_cube_source_get_z_length(cube: &vtkCubeSource) -> f64;

        fn vtk_cube_source_set_center(cube: Pin<&mut vtkCubeSource>, x: f64, y: f64, z: f64);
        fn vtk_cube_source_get_center(
            cube: &vtkCubeSource,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );

        fn vtk_cube_source_set_bounds(
            cube: Pin<&mut vtkCubeSource>,
            xmin: f64,
            xmax: f64,
            ymin: f64,
            ymax: f64,
            zmin: f64,
            zmax: f64
        );
        fn vtk_cube_source_get_bounds(
            cube: &vtkCubeSource,
            xmin: &mut f64,
            xmax: &mut f64,
            ymin: &mut f64,
            ymax: &mut f64,
            zmin: &mut f64,
            zmax: &mut f64
        );

        unsafe fn vtk_cube_source_get_output_port(
            cube: Pin<&mut vtkCubeSource>
        ) -> *mut vtkAlgorithmOutput;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkCubeSource.html",
    @name CubeSource, ffi::vtkCubeSource,
    @new ffi::vtk_cube_source_new,
    @delete ffi::vtk_cube_source_delete
);

unsafe impl Send for CubeSource {}
unsafe impl Sync for CubeSource {}

impl CubeSource {
    /// Set the length of the cube in the x-direction.
    pub fn set_x_length(&mut self, length: f64) {
        ffi::vtk_cube_source_set_x_length(self.ptr.as_mut(), length);
    }

    /// Get the length of the cube in the x-direction.
    pub fn get_x_length(&self) -> f64 {
        ffi::vtk_cube_source_get_x_length(&self.ptr.as_ref())
    }

    /// Set the length of the cube in the y-direction.
    pub fn set_y_length(&mut self, length: f64) {
        ffi::vtk_cube_source_set_y_length(self.ptr.as_mut(), length);
    }

    /// Get the length of the cube in the y-direction.
    pub fn get_y_length(&self) -> f64 {
        ffi::vtk_cube_source_get_y_length(&self.ptr.as_ref())
    }

    /// Set the length of the cube in the z-direction.
    pub fn set_z_length(&mut self, length: f64) {
        ffi::vtk_cube_source_set_z_length(self.ptr.as_mut(), length);
    }

    /// Get the length of the cube in the z-direction.
    pub fn get_z_length(&self) -> f64 {
        ffi::vtk_cube_source_get_z_length(&self.ptr.as_ref())
    }

    /// Set the center of the cube.
    pub fn set_center(&mut self, x: f64, y: f64, z: f64) {
        ffi::vtk_cube_source_set_center(self.ptr.as_mut(), x, y, z);
    }

    /// Get the center of the cube.
    pub fn get_center(&self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::vtk_cube_source_get_center(&self.ptr.as_ref(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    /// Convenience method to set all three side lengths at once.
    pub fn set_bounds(&mut self, xmin: f64, xmax: f64, ymin: f64, ymax: f64, zmin: f64, zmax: f64) {
        ffi::vtk_cube_source_set_bounds(self.ptr.as_mut(), xmin, xmax, ymin, ymax, zmin, zmax);
    }

    /// Get the bounding box of the cube.
    pub fn get_bounds(&self) -> (f64, f64, f64, f64, f64, f64) {
        let mut xmin = 0.0;
        let mut xmax = 0.0;
        let mut ymin = 0.0;
        let mut ymax = 0.0;
        let mut zmin = 0.0;
        let mut zmax = 0.0;
        ffi::vtk_cube_source_get_bounds(
            &self.ptr.as_ref(),
            &mut xmin,
            &mut xmax,
            &mut ymin,
            &mut ymax,
            &mut zmin,
            &mut zmax,
        );
        (xmin, xmax, ymin, ymax, zmin, zmax)
    }

    /// Get the output port for connecting to a mapper or filter.
    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = ffi::vtk_cube_source_get_output_port(self.ptr.as_mut());
            crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void)
        }
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkCubeSource`](https://vtk.org/doc/nightly/html/classvtkCubeSource.html)
#[allow(non_camel_case_types)]
pub trait vtkCubeSource: private::Sealed {}
