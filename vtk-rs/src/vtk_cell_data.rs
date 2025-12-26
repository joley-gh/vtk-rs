use std::ffi::{ CStr, CString };
use crate::{ DoubleArray, IntArray };

/// Opaque handle to vtkCellData from VTK
#[repr(C)]
pub struct vtkCellData {
    _private: [u8; 0],
}

#[link(name = "vtkrs", kind = "static")]
extern "C" {
    fn poly_data_get_cell_data(
        poly_data: *mut crate::vtk_poly_data::vtkPolyData
    ) -> *mut vtkCellData;
    fn cell_data_add_array(cell_data: *mut vtkCellData, array: *mut std::ffi::c_void);
    fn cell_data_remove_array(cell_data: *mut vtkCellData, name: *const i8);
    fn cell_data_get_array(cell_data: *mut vtkCellData, name: *const i8) -> *mut std::ffi::c_void;
    fn cell_data_get_number_of_arrays(cell_data: *mut vtkCellData) -> i64;
    fn cell_data_get_array_name(cell_data: *mut vtkCellData, index: i64) -> *const i8;
    fn cell_data_set_scalars(cell_data: *mut vtkCellData, array: *mut std::ffi::c_void);
    fn cell_data_get_scalars(cell_data: *mut vtkCellData) -> *mut std::ffi::c_void;
    fn cell_data_set_vectors(cell_data: *mut vtkCellData, array: *mut std::ffi::c_void);
    fn cell_data_get_vectors(cell_data: *mut vtkCellData) -> *mut std::ffi::c_void;
    fn cell_data_set_active_scalars(cell_data: *mut vtkCellData, name: *const i8);
    fn cell_data_set_active_vectors(cell_data: *mut vtkCellData, name: *const i8);
}

/// Safe wrapper for vtkCellData
///
/// CellData manages attributes (data arrays) associated with cells in PolyData.
/// For FEM: stores element properties like material IDs, cross-section types, stress results.
pub struct CellData {
    ptr: *mut vtkCellData,
    // CellData is owned by PolyData, so we don't delete it
    _owned: bool,
}

impl CellData {
    /// Create from raw pointer (internal use - CellData owned by PolyData)
    pub(crate) fn from_raw(ptr: *mut vtkCellData) -> Self {
        assert!(!ptr.is_null(), "CellData pointer is null");
        Self { ptr, _owned: false }
    }

    /// Add a data array to the cell data
    ///
    /// The array should have the same number of tuples as there are cells
    pub fn add_array(&mut self, array: &DoubleArray) {
        unsafe {
            cell_data_add_array(self.ptr, array.as_ptr() as *mut std::ffi::c_void);
        }
    }

    /// Add an integer array to the cell data
    pub fn add_int_array(&mut self, array: &IntArray) {
        unsafe {
            cell_data_add_array(self.ptr, array.as_ptr() as *mut std::ffi::c_void);
        }
    }

    /// Remove an array by name
    pub fn remove_array(&mut self, name: &str) {
        let c_name = CString::new(name).expect("CString conversion failed");
        unsafe {
            cell_data_remove_array(self.ptr, c_name.as_ptr());
        }
    }

    /// Get the number of arrays
    pub fn get_number_of_arrays(&self) -> i64 {
        unsafe { cell_data_get_number_of_arrays(self.ptr) }
    }

    /// Get array name by index
    pub fn get_array_name(&self, index: i64) -> Option<String> {
        unsafe {
            let name_ptr = cell_data_get_array_name(self.ptr, index);
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
            cell_data_set_active_scalars(self.ptr, c_name.as_ptr());
        }
    }

    /// Set scalars array directly
    pub fn set_scalars(&mut self, array: &DoubleArray) {
        unsafe {
            cell_data_set_scalars(self.ptr, array.as_ptr() as *mut std::ffi::c_void);
        }
    }

    /// Set the active vectors array
    ///
    /// Used for vector field visualization
    pub fn set_active_vectors(&mut self, name: &str) {
        let c_name = CString::new(name).expect("CString conversion failed");
        unsafe {
            cell_data_set_active_vectors(self.ptr, c_name.as_ptr());
        }
    }

    /// Set vectors array directly
    pub fn set_vectors(&mut self, array: &DoubleArray) {
        unsafe {
            cell_data_set_vectors(self.ptr, array.as_ptr() as *mut std::ffi::c_void);
        }
    }

    /// Get raw pointer (for internal use)
    pub fn as_ptr(&self) -> *mut vtkCellData {
        self.ptr
    }
}

// CellData is owned by PolyData, so no Drop implementation

unsafe impl Send for CellData {}
unsafe impl Sync for CellData {}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests require PolyData integration
}
