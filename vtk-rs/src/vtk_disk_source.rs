#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_disk_source.h");
        include!("vtk_algorithm_output.h");

        type vtkDiskSource;
        type vtkAlgorithmOutput;

        fn vtk_disk_source_new() -> *mut vtkDiskSource;
        fn vtk_disk_source_delete(disk: Pin<&mut vtkDiskSource>);

        fn vtk_disk_source_set_inner_radius(disk: Pin<&mut vtkDiskSource>, radius: f64);
        fn vtk_disk_source_get_inner_radius(disk: &vtkDiskSource) -> f64;

        fn vtk_disk_source_set_outer_radius(disk: Pin<&mut vtkDiskSource>, radius: f64);
        fn vtk_disk_source_get_outer_radius(disk: &vtkDiskSource) -> f64;

        fn vtk_disk_source_set_radial_resolution(disk: Pin<&mut vtkDiskSource>, r: i32);
        fn vtk_disk_source_get_radial_resolution(disk: &vtkDiskSource) -> i32;

        fn vtk_disk_source_set_circumferential_resolution(disk: Pin<&mut vtkDiskSource>, r: i32);
        fn vtk_disk_source_get_circumferential_resolution(disk: &vtkDiskSource) -> i32;

        fn vtk_disk_source_set_center(disk: Pin<&mut vtkDiskSource>, x: f64, y: f64, z: f64);
        fn vtk_disk_source_get_center(disk: &vtkDiskSource, x: &mut f64, y: &mut f64, z: &mut f64);

        fn vtk_disk_source_set_normal(disk: Pin<&mut vtkDiskSource>, x: f64, y: f64, z: f64);
        fn vtk_disk_source_get_normal(disk: &vtkDiskSource, x: &mut f64, y: &mut f64, z: &mut f64);

        unsafe fn vtk_disk_source_get_output_port(
            disk: Pin<&mut vtkDiskSource>
        ) -> *mut vtkAlgorithmOutput;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkDiskSource.html",
    @name DiskSource, ffi::vtkDiskSource,
    @new ffi::vtk_disk_source_new,
    @delete ffi::vtk_disk_source_delete
);

unsafe impl Send for DiskSource {}
unsafe impl Sync for DiskSource {}

impl DiskSource {
    pub fn set_inner_radius(&mut self, radius: f64) {
        ffi::vtk_disk_source_set_inner_radius(self.ptr.as_mut(), radius);
    }

    pub fn get_inner_radius(&self) -> f64 {
        ffi::vtk_disk_source_get_inner_radius(&self.ptr.as_ref())
    }

    pub fn set_outer_radius(&mut self, radius: f64) {
        ffi::vtk_disk_source_set_outer_radius(self.ptr.as_mut(), radius);
    }

    pub fn get_outer_radius(&self) -> f64 {
        ffi::vtk_disk_source_get_outer_radius(&self.ptr.as_ref())
    }

    pub fn set_radial_resolution(&mut self, r: i32) {
        ffi::vtk_disk_source_set_radial_resolution(self.ptr.as_mut(), r);
    }

    pub fn get_radial_resolution(&self) -> i32 {
        ffi::vtk_disk_source_get_radial_resolution(&self.ptr.as_ref())
    }

    pub fn set_circumferential_resolution(&mut self, r: i32) {
        ffi::vtk_disk_source_set_circumferential_resolution(self.ptr.as_mut(), r);
    }

    pub fn get_circumferential_resolution(&self) -> i32 {
        ffi::vtk_disk_source_get_circumferential_resolution(&self.ptr.as_ref())
    }

    pub fn set_center(&mut self, x: f64, y: f64, z: f64) {
        ffi::vtk_disk_source_set_center(self.ptr.as_mut(), x, y, z);
    }

    pub fn get_center(&self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::vtk_disk_source_get_center(&self.ptr.as_ref(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    pub fn set_normal(&mut self, x: f64, y: f64, z: f64) {
        ffi::vtk_disk_source_set_normal(self.ptr.as_mut(), x, y, z);
    }

    pub fn get_normal(&self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::vtk_disk_source_get_normal(&self.ptr.as_ref(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = ffi::vtk_disk_source_get_output_port(self.ptr.as_mut());
            crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void)
        }
    }
}
