// Direct extern "C" bindings (no cxx bridge)
#[repr(C)]
pub struct vtkPoints {
    _private: [u8; 0],
}

// Ensure the vtkrs static library is linked
#[link(name = "vtkrs", kind = "static")]
extern "C" {
    fn points_new() -> *mut vtkPoints;
    fn points_delete(points: *mut vtkPoints);
    fn points_insert_next_point(points: *mut vtkPoints, x: f64, y: f64, z: f64) -> i64;
    fn points_set_point(points: *mut vtkPoints, id: i64, x: f64, y: f64, z: f64);
    fn points_get_point(points: *mut vtkPoints, id: i64, x: *mut f64, y: *mut f64, z: *mut f64);
    fn points_get_number_of_points(points: *mut vtkPoints) -> i64;
    fn points_reset(points: *mut vtkPoints);
    fn points_set_number_of_points(points: *mut vtkPoints, number: i64);
    fn points_resize(points: *mut vtkPoints, number: i64);
}

/// Safe wrapper for vtkPoints - manages a collection of 3D points
pub struct Points {
    ptr: *mut vtkPoints,
}

impl Points {
    /// Create a new empty Points collection
    pub fn new() -> Self {
        crate::init_vtk();
        let ptr = unsafe { points_new() };
        if ptr.is_null() {
            panic!("Failed to create Points");
        }
        Self { ptr }
    }

    /// Get raw pointer (for internal use)
    pub fn as_mut_ptr(&mut self) -> *mut vtkPoints {
        self.ptr
    }

    /// Get raw const pointer (for internal use)
    pub fn as_ptr(&self) -> *mut vtkPoints {
        self.ptr
    }

    /// Insert a new point and return its ID
    ///
    /// # Arguments
    /// * `x`, `y`, `z` - Coordinates of the point
    ///
    /// # Returns
    /// The ID of the inserted point (starting from 0)
    pub fn insert_next_point(&mut self, x: f64, y: f64, z: f64) -> i64 {
        unsafe { points_insert_next_point(self.ptr, x, y, z) }
    }

    /// Set the coordinates of a point at a specific ID
    ///
    /// # Arguments
    /// * `id` - The ID of the point to modify
    /// * `x`, `y`, `z` - New coordinates
    pub fn set_point(&mut self, id: i64, x: f64, y: f64, z: f64) {
        unsafe {
            points_set_point(self.ptr, id, x, y, z);
        }
    }

    /// Get the coordinates of a point by ID
    ///
    /// # Arguments
    /// * `id` - The ID of the point
    ///
    /// # Returns
    /// Tuple of (x, y, z) coordinates
    pub fn get_point(&self, id: i64) -> (f64, f64, f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        unsafe {
            points_get_point(self.ptr as *mut _, id, &mut x, &mut y, &mut z);
        }
        (x, y, z)
    }

    /// Get the number of points in the collection
    pub fn get_number_of_points(&self) -> i64 {
        unsafe { points_get_number_of_points(self.ptr as *mut _) }
    }

    /// Remove all points from the collection
    pub fn reset(&mut self) {
        unsafe {
            points_reset(self.ptr);
        }
    }

    /// Pre-allocate space for a specific number of points
    ///
    /// # Arguments
    /// * `number` - Number of points to allocate space for
    pub fn set_number_of_points(&mut self, number: i64) {
        unsafe {
            points_set_number_of_points(self.ptr, number);
        }
    }

    /// Resize the points array
    ///
    /// # Arguments
    /// * `number` - New size
    pub fn resize(&mut self, number: i64) {
        unsafe {
            points_resize(self.ptr, number);
        }
    }

    /// Iterator over all points
    pub fn iter(&self) -> PointsIterator {
        PointsIterator {
            points: self,
            current: 0,
            count: self.get_number_of_points(),
        }
    }
}

impl Default for Points {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Points {
    fn drop(&mut self) {
        unsafe {
            points_delete(self.ptr);
        }
    }
}

/// Iterator over points in a Points collection
pub struct PointsIterator<'a> {
    points: &'a Points,
    current: i64,
    count: i64,
}

impl<'a> Iterator for PointsIterator<'a> {
    type Item = (i64, f64, f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.count {
            let id = self.current;
            let (x, y, z) = self.points.get_point(id);
            self.current += 1;
            Some((id, x, y, z))
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for PointsIterator<'a> {
    fn len(&self) -> usize {
        (self.count - self.current) as usize
    }
}
