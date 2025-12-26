#pragma once
#include "cxx.h"

#include <vtkPolyDataMapper.h>
#include <vtkAlgorithmOutput.h>

vtkPolyDataMapper* poly_data_mapper_new();
void poly_data_mapper_delete(vtkPolyDataMapper& pdm);
void poly_data_mapper_set_input_connection(vtkPolyDataMapper& mapper, vtkAlgorithmOutput* output);
