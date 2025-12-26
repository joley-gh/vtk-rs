#include <cstring>
#include <sstream>

#include "cxx.h"
#include "vtk_parametric_function_source.h"
#include "vtk_parametric_function_source.rs.h"

#include <vtkParametricFunctionSource.h>
#include <vtkAlgorithmOutput.h>
#include <vtkParametricFunction.h>

vtkParametricFunctionSource* vtk_parametric_function_source_new() {
    vtkParametricFunctionSource* obj = vtkParametricFunctionSource::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkParametricFunctionSource");
    }
    return obj;
}

void vtk_parametric_function_source_delete(vtkParametricFunctionSource& source) {
    source.Delete();
}

void parametric_function_source_set_parametric_function(
    vtkParametricFunctionSource& source,
    vtkParametricFunction* func
) {
    source.SetParametricFunction(func);
}

void parametric_function_source_set_u_resolution(
    vtkParametricFunctionSource& source,
    int32_t resolution
) {
    source.SetUResolution(resolution);
}

int32_t parametric_function_source_get_u_resolution(const vtkParametricFunctionSource& source) {
    return const_cast<vtkParametricFunctionSource&>(source).GetUResolution();
}

void parametric_function_source_set_v_resolution(
    vtkParametricFunctionSource& source,
    int32_t resolution
) {
    source.SetVResolution(resolution);
}

int32_t parametric_function_source_get_v_resolution(const vtkParametricFunctionSource& source) {
    return const_cast<vtkParametricFunctionSource&>(source).GetVResolution();
}

void parametric_function_source_set_w_resolution(
    vtkParametricFunctionSource& source,
    int32_t resolution
) {
    source.SetWResolution(resolution);
}

int32_t parametric_function_source_get_w_resolution(const vtkParametricFunctionSource& source) {
    return const_cast<vtkParametricFunctionSource&>(source).GetWResolution();
}

vtkAlgorithmOutput* parametric_function_source_get_output_port(
    vtkParametricFunctionSource& source
) {
    return source.GetOutputPort();
}
