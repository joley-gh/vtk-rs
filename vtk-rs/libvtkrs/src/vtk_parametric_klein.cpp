#include <cstring>
#include <sstream>

#include "cxx.h"
#include "vtk_parametric_klein.h"
#include "vtk_parametric_klein.rs.h"

#include <vtkParametricKlein.h>
#include <vtkParametricFunction.h>

vtkParametricKlein* vtk_parametric_klein_new() {
    vtkParametricKlein* obj = vtkParametricKlein::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkParametricKlein");
    }
    return obj;
}

void vtk_parametric_klein_delete(vtkParametricKlein& klein) {
    klein.Delete();
}

vtkParametricFunction* parametric_klein_as_parametric_function(vtkParametricKlein& klein) {
    return static_cast<vtkParametricFunction*>(&klein);
}
