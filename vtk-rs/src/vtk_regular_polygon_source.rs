#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_regular_polygon_source.h");
        include!("vtk_algorithm_output.h");

        type vtkRegularPolygonSource;
        type vtkAlgorithmOutput;

        fn vtk_regular_polygon_source_new() -> *mut vtkRegularPolygonSource;
        fn vtk_regular_polygon_source_delete(polygon: Pin<&mut vtkRegularPolygonSource>);
        fn vtk_regular_polygon_source_set_number_of_sides(
            polygon: Pin<&mut vtkRegularPolygonSource>,
            sides: i32
        );
        fn vtk_regular_polygon_source_get_number_of_sides(polygon: &vtkRegularPolygonSource) -> i32;
        fn vtk_regular_polygon_source_set_center(
            polygon: Pin<&mut vtkRegularPolygonSource>,
            x: f64,
            y: f64,
            z: f64
        );
        fn vtk_regular_polygon_source_get_center(
            polygon: &vtkRegularPolygonSource,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );
        fn vtk_regular_polygon_source_set_normal(
            polygon: Pin<&mut vtkRegularPolygonSource>,
            x: f64,
            y: f64,
            z: f64
        );
        fn vtk_regular_polygon_source_get_normal(
            polygon: &vtkRegularPolygonSource,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );
        fn vtk_regular_polygon_source_set_radius(
            polygon: Pin<&mut vtkRegularPolygonSource>,
            radius: f64
        );
        fn vtk_regular_polygon_source_get_radius(polygon: &vtkRegularPolygonSource) -> f64;
        fn vtk_regular_polygon_source_set_generate_polygon(
            polygon: Pin<&mut vtkRegularPolygonSource>,
            generate: bool
        );
        fn vtk_regular_polygon_source_get_generate_polygon(
            polygon: &vtkRegularPolygonSource
        ) -> bool;
        fn vtk_regular_polygon_source_generate_polygon_on(
            polygon: Pin<&mut vtkRegularPolygonSource>
        );
        fn vtk_regular_polygon_source_generate_polygon_off(
            polygon: Pin<&mut vtkRegularPolygonSource>
        );
        fn vtk_regular_polygon_source_set_generate_polyline(
            polygon: Pin<&mut vtkRegularPolygonSource>,
            generate: bool
        );
        fn vtk_regular_polygon_source_get_generate_polyline(
            polygon: &vtkRegularPolygonSource
        ) -> bool;
        fn vtk_regular_polygon_source_generate_polyline_on(
            polygon: Pin<&mut vtkRegularPolygonSource>
        );
        fn vtk_regular_polygon_source_generate_polyline_off(
            polygon: Pin<&mut vtkRegularPolygonSource>
        );
        fn vtk_regular_polygon_source_set_output_points_precision(
            polygon: Pin<&mut vtkRegularPolygonSource>,
            precision: i32
        );
        fn vtk_regular_polygon_source_get_output_points_precision(
            polygon: &vtkRegularPolygonSource
        ) -> i32;

        unsafe fn vtk_regular_polygon_source_get_output_port(
            polygon: Pin<&mut vtkRegularPolygonSource>
        ) -> *mut vtkAlgorithmOutput;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkRegularPolygonSource.html",
    @name RegularPolygonSource, ffi::vtkRegularPolygonSource,
    @new ffi::vtk_regular_polygon_source_new,
    @delete ffi::vtk_regular_polygon_source_delete
);

unsafe impl Send for RegularPolygonSource {}
unsafe impl Sync for RegularPolygonSource {}
impl RegularPolygonSource {
    /// Set the number of sides for the regular polygon.
    pub fn set_number_of_sides(&mut self, sides: i32) {
        ffi::vtk_regular_polygon_source_set_number_of_sides(self.ptr.as_mut(), sides);
    }

    /// Get the number of sides for the regular polygon.
    pub fn get_number_of_sides(&self) -> i32 {
        ffi::vtk_regular_polygon_source_get_number_of_sides(&self.ptr.as_ref())
    }
    /// Set the center of the regular polygon.
    pub fn set_center(&mut self, x: f64, y: f64, z: f64) {
        ffi::vtk_regular_polygon_source_set_center(self.ptr.as_mut(), x, y, z);
    }
    /// Get the center of the regular polygon.
    /// Returns (x, y, z).
    pub fn get_center(&self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::vtk_regular_polygon_source_get_center(&self.ptr.as_ref(), &mut x, &mut y, &mut z);
        (x, y, z)
    }
    /// Set the normal vector of the regular polygon.
    pub fn set_normal(&mut self, x: f64, y: f64, z: f64) {
        ffi::vtk_regular_polygon_source_set_normal(self.ptr.as_mut(), x, y, z);
    }
    /// Get the normal vector of the regular polygon.
    /// Returns (x, y, z).
    pub fn get_normal(&self) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi::vtk_regular_polygon_source_get_normal(&self.ptr.as_ref(), &mut x, &mut y, &mut z);
        (x, y, z)
    }
    /// Set the radius of the regular polygon.
    pub fn set_radius(&mut self, radius: f64) {
        ffi::vtk_regular_polygon_source_set_radius(self.ptr.as_mut(), radius);
    }
    /// Get the radius of the regular polygon.
    pub fn get_radius(&self) -> f64 {
        ffi::vtk_regular_polygon_source_get_radius(&self.ptr.as_ref())
    }
    /// Set whether to generate the polygon (filled).
    pub fn set_generate_polygon(&mut self, generate: bool) {
        ffi::vtk_regular_polygon_source_set_generate_polygon(self.ptr.as_mut(), generate);
    }
    /// Get whether to generate the polygon (filled).
    pub fn get_generate_polygon(&self) -> bool {
        ffi::vtk_regular_polygon_source_get_generate_polygon(&self.ptr.as_ref())
    }
    /// Enable generating the polygon (filled).
    pub fn generate_polygon_on(&mut self) {
        ffi::vtk_regular_polygon_source_generate_polygon_on(self.ptr.as_mut());
    }
    /// Disable generating the polygon (filled).
    pub fn generate_polygon_off(&mut self) {
        ffi::vtk_regular_polygon_source_generate_polygon_off(self.ptr.as_mut());
    }
    /// Set whether to generate the polyline (outline).
    pub fn set_generate_polyline(&mut self, generate: bool) {
        ffi::vtk_regular_polygon_source_set_generate_polyline(self.ptr.as_mut(), generate);
    }
    /// Get whether to generate the polyline (outline).
    pub fn get_generate_polyline(&self) -> bool {
        ffi::vtk_regular_polygon_source_get_generate_polyline(&self.ptr.as_ref())
    }
    /// Enable generating the polyline (outline).
    pub fn generate_polyline_on(&mut self) {
        ffi::vtk_regular_polygon_source_generate_polyline_on(self.ptr.as_mut());
    }
    /// Disable generating the polyline (outline).
    pub fn generate_polyline_off(&mut self) {
        ffi::vtk_regular_polygon_source_generate_polyline_off(self.ptr.as_mut());
    }
    /// Set the output points precision.
    pub fn set_output_points_precision(&mut self, precision: i32) {
        ffi::vtk_regular_polygon_source_set_output_points_precision(self.ptr.as_mut(), precision);
    }
    /// Get the output points precision.
    pub fn get_output_points_precision(&self) -> i32 {
        ffi::vtk_regular_polygon_source_get_output_points_precision(&self.ptr.as_ref())
    }

    /// Get the output port for this source.
    pub fn get_output_port(&mut self) -> crate::AlgorithmOutputPort {
        unsafe {
            let ptr = ffi::vtk_regular_polygon_source_get_output_port(self.ptr.as_mut());
            crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void)
        }
    }
}
