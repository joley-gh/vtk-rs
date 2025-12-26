#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_parametric_function.h");

        type vtkParametricFunction;

        // No new/delete here - these are abstract base class references
        // Specific parametric functions will have their own new/delete
    }
}

// This is just the base type for parametric functions
// Individual parametric functions (torus, klein, mobius) will have their own wrappers
