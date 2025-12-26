#include <cstring>
#include <sstream>

#include "cxx.h"
#include "vtk_parametric_mobius.h"
#include "vtk_parametric_mobius.rs.h"

#include <vtkParametricMobius.h>
#include <vtkParametricFunction.h>

vtkParametricMobius* vtk_parametric_mobius_new() {
    vtkParametricMobius* obj = vtkParametricMobius::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkParametricMobius");
    }
    return obj;
}

void vtk_parametric_mobius_delete(vtkParametricMobius& mobius) {
    mobius.Delete();
}

void parametric_mobius_set_radius(vtkParametricMobius& mobius, double radius) {
    mobius.SetRadius(radius);
}

double parametric_mobius_get_radius(const vtkParametricMobius& mobius) {
    return const_cast<vtkParametricMobius&>(mobius).GetRadius();
}

vtkParametricFunction* parametric_mobius_as_parametric_function(vtkParametricMobius& mobius) {
    return static_cast<vtkParametricFunction*>(&mobius);
}
