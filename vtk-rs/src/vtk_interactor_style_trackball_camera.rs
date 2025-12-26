#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_interactor_style_trackball_camera.h");

        type vtkInteractorStyleTrackballCamera;

        fn interactor_style_trackball_camera_new() -> *mut vtkInteractorStyleTrackballCamera;
        fn interactor_style_trackball_camera_delete(
            style: Pin<&mut vtkInteractorStyleTrackballCamera>
        );
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkInteractorStyleTrackballCamera.html",
    @name InteractorStyleTrackballCamera, ffi::vtkInteractorStyleTrackballCamera,
    @new ffi::interactor_style_trackball_camera_new,
    @delete ffi::interactor_style_trackball_camera_delete
);

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkInteractorStyleTrackballCamera`](https://vtk.org/doc/nightly/html/classvtkInteractorStyleTrackballCamera.html)
///
/// Interactive manipulation of the camera with trackball-style controls:
/// - Left mouse button: Rotate the camera around the scene
/// - Right mouse button: Zoom in/out
/// - Middle mouse button: Pan the camera
/// - Scroll wheel: Zoom in/out
#[allow(non_camel_case_types)]
pub trait vtkInteractorStyleTrackballCamera: private::Sealed {}
