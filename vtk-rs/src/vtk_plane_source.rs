#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_plane_source.h");
        include!("vtk_algorithm_output.h");

        type vtkPlaneSource;
        type vtkAlgorithmOutput;

        fn vtk_plane_source_new() -> *mut vtkPlaneSource;
        fn vtk_plane_source_delete(plane: Pin<&mut vtkPlaneSource>);

        fn vtk_plane_source_set_origin(plane: Pin<&mut vtkPlaneSource>, x: f64, y: f64, z: f64);
        fn vtk_plane_source_get_origin(
            plane: &vtkPlaneSource,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );

        fn vtk_plane_source_set_point1(plane: Pin<&mut vtkPlaneSource>, x: f64, y: f64, z: f64);
        fn vtk_plane_source_get_point1(
            plane: &vtkPlaneSource,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );

        fn vtk_plane_source_set_point2(plane: Pin<&mut vtkPlaneSource>, x: f64, y: f64, z: f64);
        fn vtk_plane_source_get_point2(
            plane: &vtkPlaneSource,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );

        fn vtk_plane_source_set_x_resolution(plane: Pin<&mut vtkPlaneSource>, r: i32);
        fn vtk_plane_source_get_x_resolution(plane: &vtkPlaneSource) -> i32;

        fn vtk_plane_source_set_y_resolution(plane: Pin<&mut vtkPlaneSource>, r: i32);
        fn vtk_plane_source_get_y_resolution(plane: &vtkPlaneSource) -> i32;

        fn vtk_plane_source_set_center(plane: Pin<&mut vtkPlaneSource>, x: f64, y: f64, z: f64);
        fn vtk_plane_source_set_normal(plane: Pin<&mut vtkPlaneSource>, x: f64, y: f64, z: f64);
        fn vtk_plane_source_push(plane: Pin<&mut vtkPlaneSource>, distance: f64);

        unsafe fn vtk_plane_source_get_output_port(
            plane: Pin<&mut vtkPlaneSource>
        ) -> *mut vtkAlgorithmOutput;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkPlaneSource.html",
    @name PlaneSource, ffi::vtkPlaneSource,
    @new ffi::vtk_plane_source_new,
    @delete ffi::vtk_plane_source_delete
);

unsafe impl Send for PlaneSource {}
unsafe impl Sync for PlaneSource {}

impl PlaneSource {
    pub fn set_origin(&mut self, x: f64, y: f64, z: f64) {
        ffi::vtk_plane_source_set_origin(self.ptr.as_mut(), x, y, z);
    }

    pub fn get_origin(&self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::vtk_plane_source_get_origin(&self.ptr.as_ref(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    pub fn set_point1(&mut self, x: f64, y: f64, z: f64) {
        ffi::vtk_plane_source_set_point1(self.ptr.as_mut(), x, y, z);
    }

    pub fn get_point1(&self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::vtk_plane_source_get_point1(&self.ptr.as_ref(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    pub fn set_point2(&mut self, x: f64, y: f64, z: f64) {
        ffi::vtk_plane_source_set_point2(self.ptr.as_mut(), x, y, z);
    }

    pub fn get_point2(&self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::vtk_plane_source_get_point2(&self.ptr.as_ref(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    pub fn set_x_resolution(&mut self, r: i32) {
        ffi::vtk_plane_source_set_x_resolution(self.ptr.as_mut(), r);
    }

    pub fn get_x_resolution(&self) -> i32 {
        ffi::vtk_plane_source_get_x_resolution(&self.ptr.as_ref())
    }

    pub fn set_y_resolution(&mut self, r: i32) {
        ffi::vtk_plane_source_set_y_resolution(self.ptr.as_mut(), r);
    }

    pub fn get_y_resolution(&self) -> i32 {
        ffi::vtk_plane_source_get_y_resolution(&self.ptr.as_ref())
    }

    pub fn set_center(&mut self, x: f64, y: f64, z: f64) {
        ffi::vtk_plane_source_set_center(self.ptr.as_mut(), x, y, z);
    }

    pub fn set_normal(&mut self, x: f64, y: f64, z: f64) {
        ffi::vtk_plane_source_set_normal(self.ptr.as_mut(), x, y, z);
    }

    pub fn push(&mut self, distance: f64) {
        ffi::vtk_plane_source_push(self.ptr.as_mut(), distance);
    }

    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = ffi::vtk_plane_source_get_output_port(self.ptr.as_mut());
            crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void)
        }
    }
}
