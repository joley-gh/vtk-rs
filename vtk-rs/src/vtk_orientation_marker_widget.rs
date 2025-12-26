#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_orientation_marker_widget.h");

        type vtkOrientationMarkerWidget;
        type vtkProp;
        type vtkRenderWindowInteractor;

        fn orientation_marker_widget_new() -> *mut vtkOrientationMarkerWidget;
        fn orientation_marker_widget_delete(widget: Pin<&mut vtkOrientationMarkerWidget>);
        unsafe fn orientation_marker_widget_set_orientation_marker(
            widget: Pin<&mut vtkOrientationMarkerWidget>,
            marker: *mut vtkProp
        );
        unsafe fn orientation_marker_widget_set_interactor(
            widget: Pin<&mut vtkOrientationMarkerWidget>,
            interactor: *mut vtkRenderWindowInteractor
        );
        fn orientation_marker_widget_set_viewport(
            widget: Pin<&mut vtkOrientationMarkerWidget>,
            min_x: f64,
            min_y: f64,
            max_x: f64,
            max_y: f64
        );
        fn orientation_marker_widget_set_enabled(
            widget: Pin<&mut vtkOrientationMarkerWidget>,
            enabled: i32
        );
        fn orientation_marker_widget_interactive_off(widget: Pin<&mut vtkOrientationMarkerWidget>);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkOrientationMarkerWidget.html",
    @name OrientationMarkerWidget, ffi::vtkOrientationMarkerWidget,
    @new ffi::orientation_marker_widget_new,
    @delete ffi::orientation_marker_widget_delete
);

impl OrientationMarkerWidget {
    /// Sets the orientation marker (usually an AxesActor).
    pub fn set_orientation_marker(&mut self, marker: &mut crate::AxesActor) {
        unsafe {
            let marker_ptr = marker.as_mut_ptr() as *mut ffi::vtkProp;
            ffi::orientation_marker_widget_set_orientation_marker(self.ptr.as_mut(), marker_ptr);
        }
    }

    /// Sets the interactor for the widget.
    pub fn set_interactor(&mut self, interactor: &mut crate::RenderWindowInteractor) {
        unsafe {
            let interactor_ptr = interactor.as_mut_ptr() as *mut ffi::vtkRenderWindowInteractor;
            ffi::orientation_marker_widget_set_interactor(self.ptr.as_mut(), interactor_ptr);
        }
    }

    /// Sets the viewport for the widget (normalized coordinates 0.0 to 1.0).
    /// Defaults to lower-left corner: (0.0, 0.0, 0.2, 0.2)
    pub fn set_viewport(&mut self, min_x: f64, min_y: f64, max_x: f64, max_y: f64) {
        ffi::orientation_marker_widget_set_viewport(self.ptr.as_mut(), min_x, min_y, max_x, max_y);
    }

    /// Enables or disables the widget.
    pub fn set_enabled(&mut self, enabled: bool) {
        ffi::orientation_marker_widget_set_enabled(self.ptr.as_mut(), if enabled { 1 } else { 0 });
    }

    /// Disables interactive mode (widget won't respond to mouse events).
    pub fn interactive_off(&mut self) {
        ffi::orientation_marker_widget_interactive_off(self.ptr.as_mut());
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkOrientationMarkerWidget`](https://vtk.org/doc/nightly/html/classvtkOrientationMarkerWidget.html)
///
/// A widget that displays an orientation marker (like axes) in a corner of the render window.
#[allow(non_camel_case_types)]
pub trait vtkOrientationMarkerWidget: private::Sealed {}
