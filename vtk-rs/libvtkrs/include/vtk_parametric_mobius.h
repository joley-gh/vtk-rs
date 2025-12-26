#pragma once

#include <vtkParametricMobius.h>
#include <vtkParametricFunction.h>

vtkParametricMobius* vtk_parametric_mobius_new();
void vtk_parametric_mobius_delete(vtkParametricMobius& mobius);
void parametric_mobius_set_radius(vtkParametricMobius& mobius, double radius);
double parametric_mobius_get_radius(const vtkParametricMobius& mobius);
vtkParametricFunction* parametric_mobius_as_parametric_function(vtkParametricMobius& mobius);
