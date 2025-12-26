use core::pin::Pin;

#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_area_picker.h");
        include!("vtk_renderer.h");

        type vtkAreaPicker;
        type vtkRenderer;
        type vtkProp3DCollection;

        fn vtk_area_picker_new() -> *mut vtkAreaPicker;
        fn vtk_area_picker_delete(picker: Pin<&mut vtkAreaPicker>);

        unsafe fn vtk_area_picker_area_pick(
            picker: Pin<&mut vtkAreaPicker>,
            x0: f64,
            y0: f64,
            x1: f64,
            y1: f64,
            renderer: *mut vtkRenderer
        ) -> i32;

        fn vtk_area_picker_get_prop3ds(picker: Pin<&mut vtkAreaPicker>) -> *mut vtkProp3DCollection;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkAreaPicker.html",
    @name AreaPicker, ffi::vtkAreaPicker,
    @new ffi::vtk_area_picker_new,
    @delete ffi::vtk_area_picker_delete
);

unsafe impl Send for AreaPicker {}
unsafe impl Sync for AreaPicker {}

impl AreaPicker {
    /// Perform area pick operation within a rectangular region.
    ///
    /// # Arguments
    /// * `x0, y0` - Bottom-left corner of selection rectangle (display coordinates)
    /// * `x1, y1` - Top-right corner of selection rectangle (display coordinates)
    /// * `renderer` - The renderer to pick from
    ///
    /// Returns true if any props were picked, false otherwise.
    pub fn area_pick(
        &mut self,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        renderer: &mut crate::Renderer
    ) -> bool {
        unsafe {
            let renderer_ptr = renderer.as_mut_ptr() as *mut ffi::vtkRenderer;
            let result = ffi::vtk_area_picker_area_pick(
                Pin::new_unchecked(&mut *self.as_mut_ptr()),
                x0,
                y0,
                x1,
                y1,
                renderer_ptr
            );
            result != 0
        }
    }

    /// Get the collection of Prop3D objects that were picked.
    /// Returns a raw pointer to vtkProp3DCollection - currently unimplemented in wrapper.
    ///
    /// TODO: Implement Prop3DCollection wrapper to safely iterate picked objects
    pub fn get_prop3ds(&mut self) -> *mut ffi::vtkProp3DCollection {
        unsafe { ffi::vtk_area_picker_get_prop3ds(Pin::new_unchecked(&mut *self.as_mut_ptr())) }
    }
}
