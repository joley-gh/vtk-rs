use std::ffi::{ CStr, CString };

/// Opaque handle to vtkDoubleArray from VTK
#[repr(C)]
pub struct vtkDoubleArray {
    _private: [u8; 0],
}

#[link(name = "vtkrs", kind = "static")]
extern "C" {
    fn double_array_new() -> *mut vtkDoubleArray;
    fn double_array_delete(array: *mut vtkDoubleArray);
    fn double_array_set_number_of_components(array: *mut vtkDoubleArray, num_components: i64);
    fn double_array_get_number_of_components(array: *mut vtkDoubleArray) -> i64;
    fn double_array_set_number_of_tuples(array: *mut vtkDoubleArray, num_tuples: i64);
    fn double_array_get_number_of_tuples(array: *mut vtkDoubleArray) -> i64;
    fn double_array_get_number_of_values(array: *mut vtkDoubleArray) -> i64;
    fn double_array_set_name(array: *mut vtkDoubleArray, name: *const i8);
    fn double_array_get_name(array: *mut vtkDoubleArray) -> *const i8;
    fn double_array_insert_next_value(array: *mut vtkDoubleArray, value: f64) -> i64;
    fn double_array_insert_next_tuple1(array: *mut vtkDoubleArray, value: f64) -> i64;
    fn double_array_insert_next_tuple2(array: *mut vtkDoubleArray, v0: f64, v1: f64) -> i64;
    fn double_array_insert_next_tuple3(
        array: *mut vtkDoubleArray,
        v0: f64,
        v1: f64,
        v2: f64
    ) -> i64;
    fn double_array_set_value(array: *mut vtkDoubleArray, id: i64, value: f64);
    fn double_array_set_tuple1(array: *mut vtkDoubleArray, id: i64, value: f64);
    fn double_array_set_tuple2(array: *mut vtkDoubleArray, id: i64, v0: f64, v1: f64);
    fn double_array_set_tuple3(array: *mut vtkDoubleArray, id: i64, v0: f64, v1: f64, v2: f64);
    fn double_array_get_value(array: *mut vtkDoubleArray, id: i64) -> f64;
    fn double_array_get_tuple(array: *mut vtkDoubleArray, id: i64, tuple: *mut f64);
    fn double_array_initialize(array: *mut vtkDoubleArray);
    fn double_array_squeeze(array: *mut vtkDoubleArray);
}

/// Safe wrapper for vtkDoubleArray
///
/// DoubleArray stores floating-point data for FEM analysis results:
/// - Scalar data: displacement magnitude, stress, temperature (1 component)
/// - Vector data: displacement vector, force vector (3 components)
/// - Custom data: material properties, element results (N components)
pub struct DoubleArray {
    ptr: *mut vtkDoubleArray,
}

impl DoubleArray {
    /// Create a new empty DoubleArray
    pub fn new() -> Self {
        let ptr = unsafe { double_array_new() };
        assert!(!ptr.is_null(), "Failed to create vtkDoubleArray");
        Self { ptr }
    }

    /// Set the number of components per tuple
    ///
    /// - 1 component: Scalars (e.g., displacement magnitude, stress)
    /// - 3 components: Vectors (e.g., displacement vector [dx, dy, dz])
    /// - N components: Custom multi-component data
    pub fn set_number_of_components(&mut self, num_components: i64) {
        unsafe {
            double_array_set_number_of_components(self.ptr, num_components);
        }
    }

    /// Get the number of components per tuple
    pub fn get_number_of_components(&self) -> i64 {
        unsafe { double_array_get_number_of_components(self.ptr) }
    }

    /// Set the number of tuples (pre-allocate memory)
    pub fn set_number_of_tuples(&mut self, num_tuples: i64) {
        unsafe {
            double_array_set_number_of_tuples(self.ptr, num_tuples);
        }
    }

    /// Get the number of tuples
    pub fn get_number_of_tuples(&self) -> i64 {
        unsafe { double_array_get_number_of_tuples(self.ptr) }
    }

    /// Get the total number of values (tuples × components)
    pub fn get_number_of_values(&self) -> i64 {
        unsafe { double_array_get_number_of_values(self.ptr) }
    }

    /// Set the array name (used for identifying data in VTK pipeline)
    pub fn set_name(&mut self, name: &str) {
        let c_name = CString::new(name).expect("CString conversion failed");
        unsafe {
            double_array_set_name(self.ptr, c_name.as_ptr());
        }
    }

    /// Get the array name
    pub fn get_name(&self) -> Option<String> {
        unsafe {
            let name_ptr = double_array_get_name(self.ptr);
            if name_ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(name_ptr).to_string_lossy().into_owned())
            }
        }
    }

    /// Insert a single value (for 1-component arrays)
    pub fn insert_next_value(&mut self, value: f64) -> i64 {
        unsafe { double_array_insert_next_value(self.ptr, value) }
    }

    /// Insert a 1-component tuple
    pub fn insert_next_tuple1(&mut self, value: f64) -> i64 {
        unsafe { double_array_insert_next_tuple1(self.ptr, value) }
    }

    /// Insert a 2-component tuple
    pub fn insert_next_tuple2(&mut self, v0: f64, v1: f64) -> i64 {
        unsafe { double_array_insert_next_tuple2(self.ptr, v0, v1) }
    }

    /// Insert a 3-component tuple (e.g., displacement vector)
    pub fn insert_next_tuple3(&mut self, v0: f64, v1: f64, v2: f64) -> i64 {
        unsafe { double_array_insert_next_tuple3(self.ptr, v0, v1, v2) }
    }

    /// Set a single value at specific index
    pub fn set_value(&mut self, id: i64, value: f64) {
        unsafe {
            double_array_set_value(self.ptr, id, value);
        }
    }

    /// Set a 1-component tuple at specific index
    pub fn set_tuple1(&mut self, id: i64, value: f64) {
        unsafe {
            double_array_set_tuple1(self.ptr, id, value);
        }
    }

    /// Set a 2-component tuple at specific index
    pub fn set_tuple2(&mut self, id: i64, v0: f64, v1: f64) {
        unsafe {
            double_array_set_tuple2(self.ptr, id, v0, v1);
        }
    }

    /// Set a 3-component tuple at specific index
    pub fn set_tuple3(&mut self, id: i64, v0: f64, v1: f64, v2: f64) {
        unsafe {
            double_array_set_tuple3(self.ptr, id, v0, v1, v2);
        }
    }

    /// Get a single value
    pub fn get_value(&self, id: i64) -> f64 {
        unsafe { double_array_get_value(self.ptr, id) }
    }

    /// Get a tuple as a vector
    ///
    /// The vector length will match the number of components
    pub fn get_tuple(&self, id: i64) -> Vec<f64> {
        let num_components = self.get_number_of_components() as usize;
        let mut tuple = vec![0.0; num_components];
        unsafe {
            double_array_get_tuple(self.ptr, id, tuple.as_mut_ptr());
        }
        tuple
    }

    /// Clear all data
    pub fn initialize(&mut self) {
        unsafe {
            double_array_initialize(self.ptr);
        }
    }

    /// Reclaim unused memory
    pub fn squeeze(&mut self) {
        unsafe {
            double_array_squeeze(self.ptr);
        }
    }

    /// Get the raw pointer (for internal use with other VTK functions)
    pub fn as_ptr(&self) -> *mut vtkDoubleArray {
        self.ptr
    }

    /// Create a scalar array for displacement magnitudes
    pub fn new_scalar(name: &str) -> Self {
        let mut array = Self::new();
        array.set_number_of_components(1);
        array.set_name(name);
        array
    }

    /// Create a vector array for displacement vectors
    pub fn new_vector(name: &str) -> Self {
        let mut array = Self::new();
        array.set_number_of_components(3);
        array.set_name(name);
        array
    }
}

impl Default for DoubleArray {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DoubleArray {
    fn drop(&mut self) {
        unsafe {
            double_array_delete(self.ptr);
        }
    }
}

unsafe impl Send for DoubleArray {}
unsafe impl Sync for DoubleArray {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_array_scalar() {
        let mut array = DoubleArray::new_scalar("Displacement");
        assert_eq!(array.get_number_of_components(), 1);

        array.insert_next_value(1.5);
        array.insert_next_value(2.7);

        assert_eq!(array.get_number_of_tuples(), 2);
        assert_eq!(array.get_value(0), 1.5);
        assert_eq!(array.get_value(1), 2.7);
    }

    #[test]
    fn test_double_array_vector() {
        let mut array = DoubleArray::new_vector("DisplacementVector");
        assert_eq!(array.get_number_of_components(), 3);

        array.insert_next_tuple3(1.0, 2.0, 3.0);
        array.insert_next_tuple3(4.0, 5.0, 6.0);

        assert_eq!(array.get_number_of_tuples(), 2);

        let tuple0 = array.get_tuple(0);
        assert_eq!(tuple0, vec![1.0, 2.0, 3.0]);

        let tuple1 = array.get_tuple(1);
        assert_eq!(tuple1, vec![4.0, 5.0, 6.0]);
    }
}
