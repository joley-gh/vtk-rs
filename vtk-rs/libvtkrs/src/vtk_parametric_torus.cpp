#include <cstring>
#include <sstream>

#include "cxx.h"
#include "vtk_parametric_torus.h"
#include "vtk_parametric_torus.rs.h"

#include <vtkParametricTorus.h>
#include <vtkParametricFunction.h>

vtkParametricTorus* vtk_parametric_torus_new() {
    vtkParametricTorus* obj = vtkParametricTorus::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkParametricTorus");
    }
    return obj;
}

void vtk_parametric_torus_delete(vtkParametricTorus& torus) {
    torus.Delete();
}

void parametric_torus_set_ring_radius(vtkParametricTorus& torus, double radius) {
    torus.SetRingRadius(radius);
}

double parametric_torus_get_ring_radius(const vtkParametricTorus& torus) {
    return const_cast<vtkParametricTorus&>(torus).GetRingRadius();
}

void parametric_torus_set_cross_section_radius(vtkParametricTorus& torus, double radius) {
    torus.SetCrossSectionRadius(radius);
}

double parametric_torus_get_cross_section_radius(const vtkParametricTorus& torus) {
    return const_cast<vtkParametricTorus&>(torus).GetCrossSectionRadius();
}

vtkParametricFunction* parametric_torus_as_parametric_function(vtkParametricTorus& torus) {
    return static_cast<vtkParametricFunction*>(&torus);
}
