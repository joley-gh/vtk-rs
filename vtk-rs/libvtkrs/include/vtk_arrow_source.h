#pragma once

#include "cxx.h"
#include <vtkNew.h>
#include <vtkArrowSource.h>
#include <vtkAlgorithmOutput.h>

vtkArrowSource* vtk_arrow_source_new();
void vtk_arrow_source_delete(vtkArrowSource& arrow);

void vtk_arrow_source_set_arrow_origin_to_default(vtkArrowSource& arrow);
void vtk_arrow_source_set_arrow_origin_to_center(vtkArrowSource& arrow);
rust::String vtk_arrow_source_get_arrow_origin_as_string(const vtkArrowSource& arrow);

void vtk_arrow_source_set_tip_length(vtkArrowSource& arrow, double length);
double vtk_arrow_source_get_tip_length(const vtkArrowSource& arrow);

void vtk_arrow_source_set_tip_radius(vtkArrowSource& arrow, double radius);
double vtk_arrow_source_get_tip_radius(const vtkArrowSource& arrow);

void vtk_arrow_source_set_tip_resolution(vtkArrowSource& arrow, int resolution);
int vtk_arrow_source_get_tip_resolution(const vtkArrowSource& arrow);

void vtk_arrow_source_set_shaft_radius(vtkArrowSource& arrow, double radius);
double vtk_arrow_source_get_shaft_radius(const vtkArrowSource& arrow);

void vtk_arrow_source_set_shaft_resolution(vtkArrowSource& arrow, int resolution);
int vtk_arrow_source_get_shaft_resolution(const vtkArrowSource& arrow);

void vtk_arrow_source_set_invert(vtkArrowSource& arrow, bool invert);
void vtk_arrow_source_invert_on(vtkArrowSource& arrow);
void vtk_arrow_source_invert_off(vtkArrowSource& arrow);
bool vtk_arrow_source_get_invert(const vtkArrowSource& arrow);

vtkAlgorithmOutput* vtk_arrow_source_get_output_port(vtkArrowSource& arrow);
