#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_arrow_source.h");
        include!("vtk_algorithm_output.h");

        type vtkArrowSource;
        type vtkAlgorithmOutput;

        fn vtk_arrow_source_new() -> *mut vtkArrowSource;
        fn vtk_arrow_source_delete(arrow: Pin<&mut vtkArrowSource>);

        fn vtk_arrow_source_set_arrow_origin_to_default(arrow: Pin<&mut vtkArrowSource>);
        fn vtk_arrow_source_set_arrow_origin_to_center(arrow: Pin<&mut vtkArrowSource>);
        fn vtk_arrow_source_get_arrow_origin_as_string(arrow: &vtkArrowSource) -> String;

        fn vtk_arrow_source_set_tip_length(arrow: Pin<&mut vtkArrowSource>, length: f64);
        fn vtk_arrow_source_get_tip_length(arrow: &vtkArrowSource) -> f64;

        fn vtk_arrow_source_set_tip_radius(arrow: Pin<&mut vtkArrowSource>, radius: f64);
        fn vtk_arrow_source_get_tip_radius(arrow: &vtkArrowSource) -> f64;

        fn vtk_arrow_source_set_tip_resolution(arrow: Pin<&mut vtkArrowSource>, res: i32);
        fn vtk_arrow_source_get_tip_resolution(arrow: &vtkArrowSource) -> i32;

        fn vtk_arrow_source_set_shaft_radius(arrow: Pin<&mut vtkArrowSource>, radius: f64);
        fn vtk_arrow_source_get_shaft_radius(arrow: &vtkArrowSource) -> f64;

        fn vtk_arrow_source_set_shaft_resolution(arrow: Pin<&mut vtkArrowSource>, res: i32);
        fn vtk_arrow_source_get_shaft_resolution(arrow: &vtkArrowSource) -> i32;

        fn vtk_arrow_source_set_invert(arrow: Pin<&mut vtkArrowSource>, invert: bool);
        fn vtk_arrow_source_invert_on(arrow: Pin<&mut vtkArrowSource>);
        fn vtk_arrow_source_invert_off(arrow: Pin<&mut vtkArrowSource>);
        fn vtk_arrow_source_get_invert(arrow: &vtkArrowSource) -> bool;

        unsafe fn vtk_arrow_source_get_output_port(
            arrow: Pin<&mut vtkArrowSource>
        ) -> *mut vtkAlgorithmOutput;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkArrowSource.html",
    @name ArrowSource, ffi::vtkArrowSource,
    @new ffi::vtk_arrow_source_new,
    @delete ffi::vtk_arrow_source_delete
);

unsafe impl Send for ArrowSource {}
unsafe impl Sync for ArrowSource {}

impl ArrowSource {
    pub fn set_arrow_origin_to_default(&mut self) {
        ffi::vtk_arrow_source_set_arrow_origin_to_default(self.ptr.as_mut());
    }

    pub fn set_arrow_origin_to_center(&mut self) {
        ffi::vtk_arrow_source_set_arrow_origin_to_center(self.ptr.as_mut());
    }

    pub fn get_arrow_origin_as_string(&self) -> String {
        ffi::vtk_arrow_source_get_arrow_origin_as_string(&self.ptr.as_ref())
    }

    pub fn set_tip_length(&mut self, length: f64) {
        ffi::vtk_arrow_source_set_tip_length(self.ptr.as_mut(), length);
    }

    pub fn get_tip_length(&self) -> f64 {
        ffi::vtk_arrow_source_get_tip_length(&self.ptr.as_ref())
    }

    pub fn set_tip_radius(&mut self, radius: f64) {
        ffi::vtk_arrow_source_set_tip_radius(self.ptr.as_mut(), radius);
    }

    pub fn get_tip_radius(&self) -> f64 {
        ffi::vtk_arrow_source_get_tip_radius(&self.ptr.as_ref())
    }
    pub fn set_tip_resolution(&mut self, res: i32) {
        ffi::vtk_arrow_source_set_tip_resolution(self.ptr.as_mut(), res);
    }
    pub fn get_tip_resolution(&self) -> i32 {
        ffi::vtk_arrow_source_get_tip_resolution(&self.ptr.as_ref())
    }
    pub fn set_shaft_radius(&mut self, radius: f64) {
        ffi::vtk_arrow_source_set_shaft_radius(self.ptr.as_mut(), radius);
    }
    pub fn get_shaft_radius(&self) -> f64 {
        ffi::vtk_arrow_source_get_shaft_radius(&self.ptr.as_ref())
    }
    pub fn set_shaft_resolution(&mut self, res: i32) {
        ffi::vtk_arrow_source_set_shaft_resolution(self.ptr.as_mut(), res);
    }
    pub fn get_shaft_resolution(&self) -> i32 {
        ffi::vtk_arrow_source_get_shaft_resolution(&self.ptr.as_ref())
    }
    pub fn set_invert(&mut self, invert: bool) {
        ffi::vtk_arrow_source_set_invert(self.ptr.as_mut(), invert);
    }
    pub fn invert_on(&mut self) {
        ffi::vtk_arrow_source_invert_on(self.ptr.as_mut());
    }
    pub fn invert_off(&mut self) {
        ffi::vtk_arrow_source_invert_off(self.ptr.as_mut());
    }
    pub fn get_invert(&self) -> bool {
        ffi::vtk_arrow_source_get_invert(&self.ptr.as_ref())
    }

    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = ffi::vtk_arrow_source_get_output_port(self.ptr.as_mut());
            crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void)
        }
    }
}
