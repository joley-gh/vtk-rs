use std::ptr;

/// Opaque handle to vtkCellArray from VTK
#[repr(C)]
pub struct vtkCellArray {
    _private: [u8; 0],
}

#[link(name = "vtkrs", kind = "static")]
extern "C" {
    fn cell_array_new() -> *mut vtkCellArray;
    fn cell_array_delete(cells: *mut vtkCellArray);
    fn cell_array_insert_next_cell(cells: *mut vtkCellArray, npts: i64, pts: *const i64) -> i64;
    fn cell_array_get_number_of_cells(cells: *mut vtkCellArray) -> i64;
    fn cell_array_get_number_of_connectivity_ids(cells: *mut vtkCellArray) -> i64;
    fn cell_array_get_cell(
        cells: *mut vtkCellArray,
        loc: i64,
        npts: *mut i64,
        pts: *mut *mut i64
    ) -> bool;
    fn cell_array_reset(cells: *mut vtkCellArray);
    fn cell_array_initialize(cells: *mut vtkCellArray);
}

/// Safe wrapper for vtkCellArray
///
/// CellArray stores connectivity information for cells (lines, triangles, etc.).
/// For FEM beam structures, each cell represents a beam connecting two node points.
pub struct CellArray {
    ptr: *mut vtkCellArray,
}

impl CellArray {
    /// Create a new empty CellArray
    pub fn new() -> Self {
        let ptr = unsafe { cell_array_new() };
        assert!(!ptr.is_null(), "Failed to create vtkCellArray");
        Self { ptr }
    }

    /// Insert a cell defined by a list of point IDs
    ///
    /// For a beam element (line), pass 2 point IDs.
    /// Returns the cell ID of the inserted cell.
    ///
    /// # Example
    /// ```
    /// let mut cells = CellArray::new();
    /// // Create a beam from point 0 to point 1
    /// let beam_id = cells.insert_next_cell(&[0, 1]);
    /// ```
    pub fn insert_next_cell(&mut self, point_ids: &[i64]) -> i64 {
        unsafe { cell_array_insert_next_cell(self.ptr, point_ids.len() as i64, point_ids.as_ptr()) }
    }

    /// Get the number of cells in the array
    pub fn get_number_of_cells(&self) -> i64 {
        unsafe { cell_array_get_number_of_cells(self.ptr) }
    }

    /// Get the total number of connectivity IDs
    ///
    /// This includes all point IDs across all cells.
    /// For example, 10 beams (lines) would have 20 connectivity IDs.
    pub fn get_number_of_connectivity_ids(&self) -> i64 {
        unsafe { cell_array_get_number_of_connectivity_ids(self.ptr) }
    }

    /// Get the point IDs for a specific cell
    ///
    /// Returns None if the cell doesn't exist or has no points.
    pub fn get_cell(&self, cell_id: i64) -> Option<Vec<i64>> {
        let mut npts: i64 = 0;
        let mut pts: *mut i64 = ptr::null_mut();

        let success = unsafe { cell_array_get_cell(self.ptr, cell_id, &mut npts, &mut pts) };

        if success && npts > 0 && !pts.is_null() {
            // Copy the C++ allocated array into a Rust Vec
            let point_ids = unsafe {
                let slice = std::slice::from_raw_parts(pts, npts as usize);
                slice.to_vec()
            };

            // Free the C++ allocated array using std::alloc::dealloc
            // The C++ code uses `new`, so we need to match with appropriate deallocation
            unsafe {
                // Convert to Box and drop it to free memory
                let layout = std::alloc::Layout::array::<i64>(npts as usize).unwrap();
                std::alloc::dealloc(pts as *mut u8, layout);
            }

            Some(point_ids)
        } else {
            None
        }
    }

    /// Reset the array to empty state (reuse allocated memory)
    pub fn reset(&mut self) {
        unsafe { cell_array_reset(self.ptr) }
    }

    /// Initialize the array to empty state (free memory)
    pub fn initialize(&mut self) {
        unsafe { cell_array_initialize(self.ptr) }
    }

    /// Get the raw pointer (for internal use with other VTK functions)
    pub fn as_ptr(&self) -> *mut vtkCellArray {
        self.ptr
    }

    /// Create an iterator over all cells
    pub fn iter(&self) -> CellArrayIterator {
        CellArrayIterator {
            cell_array: self,
            current: 0,
            total: self.get_number_of_cells(),
        }
    }
}

impl Default for CellArray {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for CellArray {
    fn drop(&mut self) {
        unsafe {
            cell_array_delete(self.ptr);
        }
    }
}

unsafe impl Send for CellArray {}
unsafe impl Sync for CellArray {}

/// Iterator over cells in a CellArray
pub struct CellArrayIterator<'a> {
    cell_array: &'a CellArray,
    current: i64,
    total: i64,
}

impl<'a> Iterator for CellArrayIterator<'a> {
    type Item = (i64, Vec<i64>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.total {
            let cell_id = self.current;
            self.current += 1;

            if let Some(point_ids) = self.cell_array.get_cell(cell_id) {
                Some((cell_id, point_ids))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for CellArrayIterator<'a> {
    fn len(&self) -> usize {
        (self.total - self.current) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_array_creation() {
        let cells = CellArray::new();
        assert_eq!(cells.get_number_of_cells(), 0);
    }

    #[test]
    fn test_insert_beam() {
        let mut cells = CellArray::new();
        let beam_id = cells.insert_next_cell(&[0, 1]);
        assert_eq!(beam_id, 0);
        assert_eq!(cells.get_number_of_cells(), 1);
        assert_eq!(cells.get_number_of_connectivity_ids(), 2);
    }

    #[test]
    fn test_get_cell() {
        let mut cells = CellArray::new();
        cells.insert_next_cell(&[5, 10]);

        let point_ids = cells.get_cell(0).unwrap();
        assert_eq!(point_ids, vec![5, 10]);
    }

    #[test]
    fn test_iterator() {
        let mut cells = CellArray::new();
        cells.insert_next_cell(&[0, 1]);
        cells.insert_next_cell(&[1, 2]);
        cells.insert_next_cell(&[2, 3]);

        let collected: Vec<_> = cells.iter().collect();
        assert_eq!(collected.len(), 3);
        assert_eq!(collected[0], (0, vec![0, 1]));
        assert_eq!(collected[1], (1, vec![1, 2]));
        assert_eq!(collected[2], (2, vec![2, 3]));
    }
}
