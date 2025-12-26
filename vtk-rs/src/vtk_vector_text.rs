use std::pin::Pin;

use crate::algorithm_output_port::AlgorithmOutputPort;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_vector_text.h");

        type vtkVectorText;

        fn vtk_vector_text_new() -> *mut vtkVectorText;
        fn vtk_vector_text_delete(ptr: Pin<&mut vtkVectorText>);

        fn vector_text_set_text(source: Pin<&mut vtkVectorText>, text: String);
        fn vector_text_get_text(source: Pin<&mut vtkVectorText>) -> String;

        fn vector_text_get_output_port(source: Pin<&mut vtkVectorText>) -> *mut vtkAlgorithmOutput;

        type vtkAlgorithmOutput;
    }
}

/// VectorText creates 3D polygonal text that can be used with Follower for billboard labels.
/// Unlike TextSource (which creates extruded text), VectorText creates stroke-based text
/// that's perfect for labels and annotations.
pub struct VectorText {
    ptr: *mut ffi::vtkVectorText,
}

impl VectorText {
    pub fn new() -> Self {
        let ptr = ffi::vtk_vector_text_new();
        Self { ptr }
    }

    fn as_mut(&mut self) -> Pin<&mut ffi::vtkVectorText> {
        unsafe { Pin::new_unchecked(&mut *self.ptr) }
    }

    pub fn set_text(&mut self, text: &str) {
        ffi::vector_text_set_text(self.as_mut(), text.to_string());
    }

    pub fn get_text(&mut self) -> String {
        ffi::vector_text_get_text(self.as_mut())
    }

    pub fn output_port(&mut self) -> AlgorithmOutputPort {
        let ptr = ffi::vector_text_get_output_port(self.as_mut());
        unsafe { crate::AlgorithmOutputPort::from_raw(ptr as *mut std::ffi::c_void) }
    }
}

impl Drop for VectorText {
    fn drop(&mut self) {
        ffi::vtk_vector_text_delete(self.as_mut());
    }
}
