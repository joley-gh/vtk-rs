#pragma once

#include <vtkParametricKlein.h>
#include <vtkParametricFunction.h>

vtkParametricKlein* vtk_parametric_klein_new();
void vtk_parametric_klein_delete(vtkParametricKlein& klein);
vtkParametricFunction* parametric_klein_as_parametric_function(vtkParametricKlein& klein);
