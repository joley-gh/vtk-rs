#pragma once

#include <vtkTextSource.h>
#include <vtkAlgorithmOutput.h>
#include <string>
#include "cxx.h"

vtkTextSource* vtk_text_source_new();
void vtk_text_source_delete(vtkTextSource& ptr);

void text_source_set_text(vtkTextSource& source, rust::String text);
rust::String text_source_get_text(vtkTextSource& source);

void text_source_set_backing(vtkTextSource& source, bool backing);
bool text_source_get_backing(vtkTextSource& source);

vtkAlgorithmOutput* text_source_get_output_port(vtkTextSource& source);
