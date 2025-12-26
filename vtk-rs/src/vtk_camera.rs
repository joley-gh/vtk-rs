#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_camera.h");

        type vtkCamera;

        fn camera_new() -> *mut vtkCamera;
        fn camera_delete(camera: Pin<&mut vtkCamera>);

        // Position and orientation
        fn camera_set_position(camera: Pin<&mut vtkCamera>, x: f64, y: f64, z: f64);
        fn camera_get_position(camera: Pin<&mut vtkCamera>, x: &mut f64, y: &mut f64, z: &mut f64);
        fn camera_set_focal_point(camera: Pin<&mut vtkCamera>, x: f64, y: f64, z: f64);
        fn camera_get_focal_point(
            camera: Pin<&mut vtkCamera>,
            x: &mut f64,
            y: &mut f64,
            z: &mut f64
        );
        fn camera_set_view_up(camera: Pin<&mut vtkCamera>, x: f64, y: f64, z: f64);
        fn camera_get_view_up(camera: Pin<&mut vtkCamera>, x: &mut f64, y: &mut f64, z: &mut f64);

        // Camera movements
        fn camera_azimuth(camera: Pin<&mut vtkCamera>, angle: f64);
        fn camera_elevation(camera: Pin<&mut vtkCamera>, angle: f64);
        fn camera_roll(camera: Pin<&mut vtkCamera>, angle: f64);
        fn camera_zoom(camera: Pin<&mut vtkCamera>, factor: f64);
        fn camera_dolly(camera: Pin<&mut vtkCamera>, factor: f64);

        // Clipping planes
        fn camera_set_clipping_range(camera: Pin<&mut vtkCamera>, near: f64, far: f64);
        fn camera_get_clipping_range(camera: Pin<&mut vtkCamera>, near: &mut f64, far: &mut f64);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkCamera.html",
    @name Camera, ffi::vtkCamera,
    @new ffi::camera_new,
    @delete ffi::camera_delete
);

impl Camera {
    /// Set the position of the camera in world coordinates.
    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        ffi::camera_set_position(self.ptr.as_mut(), x, y, z);
    }

    /// Get the position of the camera in world coordinates.
    pub fn get_position(&mut self) -> (f64, f64, f64) {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        ffi::camera_get_position(self.ptr.as_mut(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    /// Set the focal point of the camera in world coordinates.
    /// This is the point the camera is looking at.
    pub fn set_focal_point(&mut self, x: f64, y: f64, z: f64) {
        ffi::camera_set_focal_point(self.ptr.as_mut(), x, y, z);
    }

    /// Get the focal point of the camera in world coordinates.
    pub fn get_focal_point(&mut self) -> (f64, f64, f64) {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        ffi::camera_get_focal_point(self.ptr.as_mut(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    /// Set the view up direction for the camera.
    /// This defines which direction is "up" for the camera.
    pub fn set_view_up(&mut self, x: f64, y: f64, z: f64) {
        ffi::camera_set_view_up(self.ptr.as_mut(), x, y, z);
    }

    /// Get the view up direction for the camera.
    pub fn get_view_up(&mut self) -> (f64, f64, f64) {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        ffi::camera_get_view_up(self.ptr.as_mut(), &mut x, &mut y, &mut z);
        (x, y, z)
    }

    /// Rotate the camera about the view up vector centered at the focal point.
    /// Angle is in degrees.
    pub fn azimuth(&mut self, angle: f64) {
        ffi::camera_azimuth(self.ptr.as_mut(), angle);
    }

    /// Rotate the camera about the cross product of the view plane normal
    /// and the view up vector, centered at the focal point.
    /// Angle is in degrees.
    pub fn elevation(&mut self, angle: f64) {
        ffi::camera_elevation(self.ptr.as_mut(), angle);
    }

    /// Rotate the camera about the direction of projection.
    /// Angle is in degrees.
    pub fn roll(&mut self, angle: f64) {
        ffi::camera_roll(self.ptr.as_mut(), angle);
    }

    /// Change the camera's viewing angle (field of view).
    /// A factor > 1.0 zooms in, < 1.0 zooms out.
    pub fn zoom(&mut self, factor: f64) {
        ffi::camera_zoom(self.ptr.as_mut(), factor);
    }

    /// Move the camera toward (factor > 1.0) or away from (factor < 1.0) the focal point.
    pub fn dolly(&mut self, factor: f64) {
        ffi::camera_dolly(self.ptr.as_mut(), factor);
    }

    /// Set the near and far clipping planes.
    pub fn set_clipping_range(&mut self, near: f64, far: f64) {
        ffi::camera_set_clipping_range(self.ptr.as_mut(), near, far);
    }

    /// Get the near and far clipping planes.
    pub fn get_clipping_range(&mut self) -> (f64, f64) {
        let (mut near, mut far) = (0.0, 0.0);
        ffi::camera_get_clipping_range(self.ptr.as_mut(), &mut near, &mut far);
        (near, far)
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkCamera`](https://vtk.org/doc/nightly/html/classvtkCamera.html)
///
/// A virtual camera for 3D rendering.
#[allow(non_camel_case_types)]
pub trait vtkCamera: private::Sealed {}
