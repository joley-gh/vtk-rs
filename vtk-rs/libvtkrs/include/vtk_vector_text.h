#pragma once

#include <vtkVectorText.h>
#include <vtkAlgorithmOutput.h>
#include <string>
#include <cxx.h>

vtkVectorText* vtk_vector_text_new();
void vtk_vector_text_delete(vtkVectorText& ptr);

void vector_text_set_text(vtkVectorText& source, rust::String text);
rust::String vector_text_get_text(vtkVectorText& source);

vtkAlgorithmOutput* vector_text_get_output_port(vtkVectorText& source);
