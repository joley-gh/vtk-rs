#pragma once

#include <vtkParametricFunctionSource.h>
#include <vtkParametricFunction.h>
#include <vtkAlgorithmOutput.h>

vtkParametricFunctionSource* vtk_parametric_function_source_new();
void vtk_parametric_function_source_delete(vtkParametricFunctionSource& source);
void parametric_function_source_set_parametric_function(
    vtkParametricFunctionSource& source,
    vtkParametricFunction* func
);
void parametric_function_source_set_u_resolution(
    vtkParametricFunctionSource& source,
    int32_t resolution
);
int32_t parametric_function_source_get_u_resolution(const vtkParametricFunctionSource& source);
void parametric_function_source_set_v_resolution(
    vtkParametricFunctionSource& source,
    int32_t resolution
);
int32_t parametric_function_source_get_v_resolution(const vtkParametricFunctionSource& source);
void parametric_function_source_set_w_resolution(
    vtkParametricFunctionSource& source,
    int32_t resolution
);
int32_t parametric_function_source_get_w_resolution(const vtkParametricFunctionSource& source);
vtkAlgorithmOutput* parametric_function_source_get_output_port(
    vtkParametricFunctionSource& source
);
