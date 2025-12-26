#pragma once

#include <vtkNew.h>
#include <vtkRegularPolygonSource.h>
#include <vtkAlgorithmOutput.h>

vtkRegularPolygonSource* vtk_regular_polygon_source_new();
void vtk_regular_polygon_source_delete(vtkRegularPolygonSource& polygon);
void vtk_regular_polygon_source_set_number_of_sides(vtkRegularPolygonSource& polygon, int number_of_sides);
int vtk_regular_polygon_source_get_number_of_sides(const vtkRegularPolygonSource& polygon);

void vtk_regular_polygon_source_set_center(vtkRegularPolygonSource& polygon, double x, double y, double z);
void vtk_regular_polygon_source_get_center(const vtkRegularPolygonSource& polygon, double& x, double& y, double& z);

void vtk_regular_polygon_source_set_normal(vtkRegularPolygonSource& polygon, double x, double y, double z);
void vtk_regular_polygon_source_get_normal(const vtkRegularPolygonSource& polygon, double& x, double& y, double& z);

void vtk_regular_polygon_source_set_radius(vtkRegularPolygonSource& polygon, double radius);
double vtk_regular_polygon_source_get_radius(const vtkRegularPolygonSource& polygon);

void vtk_regular_polygon_source_set_generate_polygon(vtkRegularPolygonSource& polygon, bool generate);
bool vtk_regular_polygon_source_get_generate_polygon(const vtkRegularPolygonSource& polygon);

void vtk_regular_polygon_source_generate_polygon_on(vtkRegularPolygonSource& polygon);
void vtk_regular_polygon_source_generate_polygon_off(vtkRegularPolygonSource& polygon);

void vtk_regular_polygon_source_set_generate_polyline(vtkRegularPolygonSource& polygon, bool generate);
bool vtk_regular_polygon_source_get_generate_polyline(const vtkRegularPolygonSource& polygon);

void vtk_regular_polygon_source_generate_polyline_on(vtkRegularPolygonSource& polygon);
void vtk_regular_polygon_source_generate_polyline_off(vtkRegularPolygonSource& polygon);

void vtk_regular_polygon_source_set_output_points_precision(vtkRegularPolygonSource& polygon, int precision);
int vtk_regular_polygon_source_get_output_points_precision(const vtkRegularPolygonSource& polygon);

vtkAlgorithmOutput* vtk_regular_polygon_source_get_output_port(vtkRegularPolygonSource& polygon);