#pragma once

#include <vtkNew.h>
#include <vtkPlaneSource.h>
#include <vtkAlgorithmOutput.h>

vtkPlaneSource* vtk_plane_source_new();
void vtk_plane_source_delete(vtkPlaneSource& plane);

void vtk_plane_source_set_origin(vtkPlaneSource& plane, double x, double y, double z);
void vtk_plane_source_get_origin(const vtkPlaneSource& plane, double& x, double& y, double& z);

void vtk_plane_source_set_point1(vtkPlaneSource& plane, double x, double y, double z);
void vtk_plane_source_get_point1(const vtkPlaneSource& plane, double& x, double& y, double& z);

void vtk_plane_source_set_point2(vtkPlaneSource& plane, double x, double y, double z);
void vtk_plane_source_get_point2(const vtkPlaneSource& plane, double& x, double& y, double& z);

void vtk_plane_source_set_x_resolution(vtkPlaneSource& plane, int r);
int vtk_plane_source_get_x_resolution(const vtkPlaneSource& plane);

void vtk_plane_source_set_y_resolution(vtkPlaneSource& plane, int r);
int vtk_plane_source_get_y_resolution(const vtkPlaneSource& plane);

void vtk_plane_source_set_center(vtkPlaneSource& plane, double x, double y, double z);
void vtk_plane_source_set_normal(vtkPlaneSource& plane, double x, double y, double z);
void vtk_plane_source_push(vtkPlaneSource& plane, double distance);

vtkAlgorithmOutput* vtk_plane_source_get_output_port(vtkPlaneSource& plane);
