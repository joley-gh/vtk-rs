#pragma once

#include <vtkParametricTorus.h>
#include <vtkParametricFunction.h>

vtkParametricTorus* vtk_parametric_torus_new();
void vtk_parametric_torus_delete(vtkParametricTorus& torus);
void parametric_torus_set_ring_radius(vtkParametricTorus& torus, double radius);
double parametric_torus_get_ring_radius(const vtkParametricTorus& torus);
void parametric_torus_set_cross_section_radius(vtkParametricTorus& torus, double radius);
double parametric_torus_get_cross_section_radius(const vtkParametricTorus& torus);
vtkParametricFunction* parametric_torus_as_parametric_function(vtkParametricTorus& torus);
