use std::ffi::{CStr, CString};

/// Opaque handle to vtkIntArray from VTK
#[repr(C)]
pub struct vtkIntArray {
    _private: [u8; 0],
}

#[link(name = "vtkrs", kind = "static")]
extern "C" {
    fn int_array_new() -> *mut vtkIntArray;
    fn int_array_delete(array: *mut vtkIntArray);
    fn int_array_set_number_of_components(array: *mut vtkIntArray, num_components: i64);
    fn int_array_get_number_of_components(array: *mut vtkIntArray) -> i64;
    fn int_array_set_number_of_tuples(array: *mut vtkIntArray, num_tuples: i64);
    fn int_array_get_number_of_tuples(array: *mut vtkIntArray) -> i64;
    fn int_array_get_number_of_values(array: *mut vtkIntArray) -> i64;
    fn int_array_set_name(array: *mut vtkIntArray, name: *const i8);
    fn int_array_get_name(array: *mut vtkIntArray) -> *const i8;
    fn int_array_insert_next_value(array: *mut vtkIntArray, value: i32) -> i64;
    fn int_array_insert_next_tuple1(array: *mut vtkIntArray, value: i32) -> i64;
    fn int_array_set_value(array: *mut vtkIntArray, id: i64, value: i32);
    fn int_array_set_tuple1(array: *mut vtkIntArray, id: i64, value: i32);
    fn int_array_get_value(array: *mut vtkIntArray, id: i64) -> i32;
    fn int_array_get_tuple(array: *mut vtkIntArray, id: i64, tuple: *mut i32);
    fn int_array_initialize(array: *mut vtkIntArray);
    fn int_array_squeeze(array: *mut vtkIntArray);
}

/// Safe wrapper for vtkIntArray
///
/// IntArray stores integer data for FEM:
/// - Element IDs
/// - Material type IDs
/// - Boundary condition flags
/// - Node/element groups
pub struct IntArray {
    ptr: *mut vtkIntArray,
}

impl IntArray {
    /// Create a new empty IntArray
    pub fn new() -> Self {
        let ptr = unsafe { int_array_new() };
        assert!(!ptr.is_null(), "Failed to create vtkIntArray");
        Self { ptr }
    }

    /// Set the number of components per tuple
    pub fn set_number_of_components(&mut self, num_components: i64) {
        unsafe {
            int_array_set_number_of_components(self.ptr, num_components);
        }
    }

    /// Get the number of components per tuple
    pub fn get_number_of_components(&self) -> i64 {
        unsafe { int_array_get_number_of_components(self.ptr) }
    }

    /// Set the number of tuples (pre-allocate memory)
    pub fn set_number_of_tuples(&mut self, num_tuples: i64) {
        unsafe {
            int_array_set_number_of_tuples(self.ptr, num_tuples);
        }
    }

    /// Get the number of tuples
    pub fn get_number_of_tuples(&self) -> i64 {
        unsafe { int_array_get_number_of_tuples(self.ptr) }
    }

    /// Get the total number of values (tuples × components)
    pub fn get_number_of_values(&self) -> i64 {
        unsafe { int_array_get_number_of_values(self.ptr) }
    }

    /// Set the array name (used for identifying data in VTK pipeline)
    pub fn set_name(&mut self, name: &str) {
        let c_name = CString::new(name).expect("CString conversion failed");
        unsafe {
            int_array_set_name(self.ptr, c_name.as_ptr());
        }
    }

    /// Get the array name
    pub fn get_name(&self) -> Option<String> {
        unsafe {
            let name_ptr = int_array_get_name(self.ptr);
            if name_ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(name_ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Insert a single value (for 1-component arrays)
    pub fn insert_next_value(&mut self, value: i32) -> i64 {
        unsafe { int_array_insert_next_value(self.ptr, value) }
    }

    /// Insert a 1-component tuple
    pub fn insert_next_tuple1(&mut self, value: i32) -> i64 {
        unsafe { int_array_insert_next_tuple1(self.ptr, value) }
    }

    /// Set a single value at specific index
    pub fn set_value(&mut self, id: i64, value: i32) {
        unsafe {
            int_array_set_value(self.ptr, id, value);
        }
    }

    /// Set a 1-component tuple at specific index
    pub fn set_tuple1(&mut self, id: i64, value: i32) {
        unsafe {
            int_array_set_tuple1(self.ptr, id, value);
        }
    }

    /// Get a single value
    pub fn get_value(&self, id: i64) -> i32 {
        unsafe { int_array_get_value(self.ptr, id) }
    }

    /// Get a tuple as a vector
    pub fn get_tuple(&self, id: i64) -> Vec<i32> {
        let num_components = self.get_number_of_components() as usize;
        let mut tuple = vec![0; num_components];
        unsafe {
            int_array_get_tuple(self.ptr, id, tuple.as_mut_ptr());
        }
        tuple
    }

    /// Clear all data
    pub fn initialize(&mut self) {
        unsafe {
            int_array_initialize(self.ptr);
        }
    }

    /// Reclaim unused memory
    pub fn squeeze(&mut self) {
        unsafe {
            int_array_squeeze(self.ptr);
        }
    }

    /// Get the raw pointer (for internal use with other VTK functions)
    pub fn as_ptr(&self) -> *mut vtkIntArray {
        self.ptr
    }

    /// Create an array for element/material IDs
    pub fn new_id_array(name: &str) -> Self {
        let mut array = Self::new();
        array.set_number_of_components(1);
        array.set_name(name);
        array
    }
}

impl Default for IntArray {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for IntArray {
    fn drop(&mut self) {
        unsafe {
            int_array_delete(self.ptr);
        }
    }
}

unsafe impl Send for IntArray {}
unsafe impl Sync for IntArray {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_array() {
        let mut array = IntArray::new_id_array("MaterialID");
        assert_eq!(array.get_number_of_components(), 1);
        
        array.insert_next_value(1);
        array.insert_next_value(2);
        array.insert_next_value(1);
        
        assert_eq!(array.get_number_of_tuples(), 3);
        assert_eq!(array.get_value(0), 1);
        assert_eq!(array.get_value(1), 2);
        assert_eq!(array.get_value(2), 1);
    }

    #[test]
    fn test_int_array_modify() {
        let mut array = IntArray::new();
        array.set_number_of_components(1);
        array.set_number_of_tuples(3);
        
        array.set_value(0, 10);
        array.set_value(1, 20);
        array.set_value(2, 30);
        
        assert_eq!(array.get_value(0), 10);
        assert_eq!(array.get_value(1), 20);
        assert_eq!(array.get_value(2), 30);
    }
}
