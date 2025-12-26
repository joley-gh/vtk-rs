use std::ffi::{ CStr, CString };
use crate::{ DoubleArray, IntArray };

/// Opaque handle to vtkPointData from VTK
#[repr(C)]
pub struct vtkPointData {
    _private: [u8; 0],
}

#[link(name = "vtkrs", kind = "static")]
extern "C" {
    fn poly_data_get_point_data(
        poly_data: *mut crate::vtk_poly_data::vtkPolyData
    ) -> *mut vtkPointData;
    fn point_data_add_array(point_data: *mut vtkPointData, array: *mut std::ffi::c_void);
    fn point_data_remove_array(point_data: *mut vtkPointData, name: *const i8);
    fn point_data_get_array(
        point_data: *mut vtkPointData,
        name: *const i8
    ) -> *mut std::ffi::c_void;
    fn point_data_get_number_of_arrays(point_data: *mut vtkPointData) -> i64;
    fn point_data_get_array_name(point_data: *mut vtkPointData, index: i64) -> *const i8;
    fn point_data_set_scalars(point_data: *mut vtkPointData, array: *mut std::ffi::c_void);
    fn point_data_get_scalars(point_data: *mut vtkPointData) -> *mut std::ffi::c_void;
    fn point_data_set_vectors(point_data: *mut vtkPointData, array: *mut std::ffi::c_void);
    fn point_data_get_vectors(point_data: *mut vtkPointData) -> *mut std::ffi::c_void;
    fn point_data_set_active_scalars(point_data: *mut vtkPointData, name: *const i8);
    fn point_data_set_active_vectors(point_data: *mut vtkPointData, name: *const i8);
}

/// Safe wrapper for vtkPointData
///
/// PointData manages attributes (data arrays) associated with points in PolyData.
/// For FEM: stores node properties like displacements, boundary conditions, temperatures.
pub struct PointData {
    ptr: *mut vtkPointData,
    // PointData is owned by PolyData, so we don't delete it
    _owned: bool,
}

impl PointData {
    /// Create from raw pointer (internal use - PointData owned by PolyData)
    pub(crate) fn from_raw(ptr: *mut vtkPointData) -> Self {
        assert!(!ptr.is_null(), "PointData pointer is null");
        Self { ptr, _owned: false }
    }

    /// Add a data array to the point data
    ///
    /// The array should have the same number of tuples as there are points
    pub fn add_array(&mut self, array: &DoubleArray) {
        unsafe {
            point_data_add_array(self.ptr, array.as_ptr() as *mut std::ffi::c_void);
        }
    }

    /// Add an integer array to the point data
    pub fn add_int_array(&mut self, array: &IntArray) {
        unsafe {
            point_data_add_array(self.ptr, array.as_ptr() as *mut std::ffi::c_void);
        }
    }

    /// Remove an array by name
    pub fn remove_array(&mut self, name: &str) {
        let c_name = CString::new(name).expect("CString conversion failed");
        unsafe {
            point_data_remove_array(self.ptr, c_name.as_ptr());
        }
    }

    /// Get the number of arrays
    pub fn get_number_of_arrays(&self) -> i64 {
        unsafe { point_data_get_number_of_arrays(self.ptr) }
    }

    /// Get array name by index
    pub fn get_array_name(&self, index: i64) -> Option<String> {
        unsafe {
            let name_ptr = point_data_get_array_name(self.ptr, index);
            if name_ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(name_ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Set the active scalars array
    ///
    /// This determines which array is used for color mapping in visualization
    pub fn set_active_scalars(&mut self, name: &str) {
        let c_name = CString::new(name).expect("CString conversion failed");
        unsafe {
            point_data_set_active_scalars(self.ptr, c_name.as_ptr());
        }
    }

    /// Set scalars array directly
    pub fn set_scalars(&mut self, array: &DoubleArray) {
        unsafe {
            point_data_set_scalars(self.ptr, array.as_ptr() as *mut std::ffi::c_void);
        }
    }

    /// Set the active vectors array
    ///
    /// Used for vector field visualization (arrows, glyphs)
    pub fn set_active_vectors(&mut self, name: &str) {
        let c_name = CString::new(name).expect("CString conversion failed");
        unsafe {
            point_data_set_active_vectors(self.ptr, c_name.as_ptr());
        }
    }

    /// Set vectors array directly
    pub fn set_vectors(&mut self, array: &DoubleArray) {
        unsafe {
            point_data_set_vectors(self.ptr, array.as_ptr() as *mut std::ffi::c_void);
        }
    }

    /// Get raw pointer (for internal use)
    pub fn as_ptr(&self) -> *mut vtkPointData {
        self.ptr
    }
}

// PointData is owned by PolyData, so no Drop implementation

unsafe impl Send for PointData {}
unsafe impl Sync for PointData {}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests require PolyData integration
}
