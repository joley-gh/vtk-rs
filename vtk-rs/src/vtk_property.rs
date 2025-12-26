#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_property.h");

        type vtkProperty;

        fn property_new() -> *mut vtkProperty;
        fn property_delete(property: Pin<&mut vtkProperty>);

        // Color and opacity
        fn property_set_color(property: Pin<&mut vtkProperty>, r: f64, g: f64, b: f64);
        fn property_get_color(
            property: Pin<&mut vtkProperty>,
            r: &mut f64,
            g: &mut f64,
            b: &mut f64
        );
        fn property_set_opacity(property: Pin<&mut vtkProperty>, opacity: f64);
        fn property_get_opacity(property: Pin<&mut vtkProperty>) -> f64;

        // Line and point size
        fn property_set_line_width(property: Pin<&mut vtkProperty>, width: f64);
        fn property_get_line_width(property: Pin<&mut vtkProperty>) -> f64;
        fn property_set_point_size(property: Pin<&mut vtkProperty>, size: f64);
        fn property_get_point_size(property: Pin<&mut vtkProperty>) -> f64;

        // Representation type
        fn property_set_representation(property: Pin<&mut vtkProperty>, representation: i32);
        fn property_get_representation(property: Pin<&mut vtkProperty>) -> i32;

        // Edge visibility
        fn property_set_edge_visibility(property: Pin<&mut vtkProperty>, visible: bool);
        fn property_get_edge_visibility(property: Pin<&mut vtkProperty>) -> bool;
        fn property_set_edge_color(property: Pin<&mut vtkProperty>, r: f64, g: f64, b: f64);
        fn property_get_edge_color(
            property: Pin<&mut vtkProperty>,
            r: &mut f64,
            g: &mut f64,
            b: &mut f64
        );

        // Interpolation type
        fn property_set_interpolation(property: Pin<&mut vtkProperty>, interpolation: i32);
        fn property_get_interpolation(property: Pin<&mut vtkProperty>) -> i32;

        // Lighting properties
        fn property_set_ambient(property: Pin<&mut vtkProperty>, ambient: f64);
        fn property_get_ambient(property: Pin<&mut vtkProperty>) -> f64;
        fn property_set_diffuse(property: Pin<&mut vtkProperty>, diffuse: f64);
        fn property_get_diffuse(property: Pin<&mut vtkProperty>) -> f64;
        fn property_set_specular(property: Pin<&mut vtkProperty>, specular: f64);
        fn property_get_specular(property: Pin<&mut vtkProperty>) -> f64;
        fn property_set_specular_power(property: Pin<&mut vtkProperty>, power: f64);
        fn property_get_specular_power(property: Pin<&mut vtkProperty>) -> f64;
    }
}

/// Representation types for vtkProperty
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepresentationType {
    Points = 0,
    Wireframe = 1,
    Surface = 2,
}

/// Interpolation types for vtkProperty
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpolationType {
    Flat = 0,
    Gouraud = 1,
    Phong = 2,
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkProperty.html",
    @name Property, ffi::vtkProperty,
    @new ffi::property_new,
    @delete ffi::property_delete
);

impl Property {
    /// Set the color of the object. Values should be in the range [0.0, 1.0].
    pub fn set_color(&mut self, r: f64, g: f64, b: f64) {
        ffi::property_set_color(self.ptr.as_mut(), r, g, b);
    }

    /// Get the color of the object.
    pub fn get_color(&mut self) -> (f64, f64, f64) {
        let (mut r, mut g, mut b) = (0.0, 0.0, 0.0);
        ffi::property_get_color(self.ptr.as_mut(), &mut r, &mut g, &mut b);
        (r, g, b)
    }

    /// Set the opacity of the object. Value should be in the range [0.0, 1.0].
    pub fn set_opacity(&mut self, opacity: f64) {
        ffi::property_set_opacity(self.ptr.as_mut(), opacity);
    }

    /// Get the opacity of the object.
    pub fn get_opacity(&mut self) -> f64 {
        ffi::property_get_opacity(self.ptr.as_mut())
    }

    /// Set the line width for wireframe rendering.
    pub fn set_line_width(&mut self, width: f64) {
        ffi::property_set_line_width(self.ptr.as_mut(), width);
    }

    /// Get the line width for wireframe rendering.
    pub fn get_line_width(&mut self) -> f64 {
        ffi::property_get_line_width(self.ptr.as_mut())
    }

    /// Set the point size for point rendering.
    pub fn set_point_size(&mut self, size: f64) {
        ffi::property_set_point_size(self.ptr.as_mut(), size);
    }

    /// Get the point size for point rendering.
    pub fn get_point_size(&mut self) -> f64 {
        ffi::property_get_point_size(self.ptr.as_mut())
    }

    /// Set the representation type (points, wireframe, or surface).
    pub fn set_representation(&mut self, representation: RepresentationType) {
        ffi::property_set_representation(self.ptr.as_mut(), representation as i32);
    }

    /// Get the representation type.
    pub fn get_representation(&mut self) -> RepresentationType {
        match ffi::property_get_representation(self.ptr.as_mut()) {
            0 => RepresentationType::Points,
            1 => RepresentationType::Wireframe,
            2 => RepresentationType::Surface,
            _ => RepresentationType::Surface, // Default to surface
        }
    }

    /// Set whether edges are visible (useful for surface representation).
    pub fn set_edge_visibility(&mut self, visible: bool) {
        ffi::property_set_edge_visibility(self.ptr.as_mut(), visible);
    }

    /// Get whether edges are visible.
    pub fn get_edge_visibility(&mut self) -> bool {
        ffi::property_get_edge_visibility(self.ptr.as_mut())
    }

    /// Set the color of edges when edge visibility is enabled.
    pub fn set_edge_color(&mut self, r: f64, g: f64, b: f64) {
        ffi::property_set_edge_color(self.ptr.as_mut(), r, g, b);
    }

    /// Get the color of edges.
    pub fn get_edge_color(&mut self) -> (f64, f64, f64) {
        let (mut r, mut g, mut b) = (0.0, 0.0, 0.0);
        ffi::property_get_edge_color(self.ptr.as_mut(), &mut r, &mut g, &mut b);
        (r, g, b)
    }

    /// Set the interpolation type (flat, Gouraud, or Phong shading).
    pub fn set_interpolation(&mut self, interpolation: InterpolationType) {
        ffi::property_set_interpolation(self.ptr.as_mut(), interpolation as i32);
    }

    /// Get the interpolation type.
    pub fn get_interpolation(&mut self) -> InterpolationType {
        match ffi::property_get_interpolation(self.ptr.as_mut()) {
            0 => InterpolationType::Flat,
            1 => InterpolationType::Gouraud,
            2 => InterpolationType::Phong,
            _ => InterpolationType::Gouraud, // Default to Gouraud
        }
    }

    /// Set the ambient lighting coefficient. Value in range [0.0, 1.0].
    pub fn set_ambient(&mut self, ambient: f64) {
        ffi::property_set_ambient(self.ptr.as_mut(), ambient);
    }

    /// Get the ambient lighting coefficient.
    pub fn get_ambient(&mut self) -> f64 {
        ffi::property_get_ambient(self.ptr.as_mut())
    }

    /// Set the diffuse lighting coefficient. Value in range [0.0, 1.0].
    pub fn set_diffuse(&mut self, diffuse: f64) {
        ffi::property_set_diffuse(self.ptr.as_mut(), diffuse);
    }

    /// Get the diffuse lighting coefficient.
    pub fn get_diffuse(&mut self) -> f64 {
        ffi::property_get_diffuse(self.ptr.as_mut())
    }

    /// Set the specular lighting coefficient. Value in range [0.0, 1.0].
    pub fn set_specular(&mut self, specular: f64) {
        ffi::property_set_specular(self.ptr.as_mut(), specular);
    }

    /// Get the specular lighting coefficient.
    pub fn get_specular(&mut self) -> f64 {
        ffi::property_get_specular(self.ptr.as_mut())
    }

    /// Set the specular power (shininess). Typical values are 1-128.
    pub fn set_specular_power(&mut self, power: f64) {
        ffi::property_set_specular_power(self.ptr.as_mut(), power);
    }

    /// Get the specular power.
    pub fn get_specular_power(&mut self) -> f64 {
        ffi::property_get_specular_power(self.ptr.as_mut())
    }
}
