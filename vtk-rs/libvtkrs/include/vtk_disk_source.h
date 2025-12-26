#pragma once

#include <vtkNew.h>
#include <vtkDiskSource.h>
#include <vtkAlgorithmOutput.h>

vtkDiskSource* vtk_disk_source_new();
void vtk_disk_source_delete(vtkDiskSource& disk);

void vtk_disk_source_set_inner_radius(vtkDiskSource& disk, double radius);
double vtk_disk_source_get_inner_radius(const vtkDiskSource& disk);

void vtk_disk_source_set_outer_radius(vtkDiskSource& disk, double radius);
double vtk_disk_source_get_outer_radius(const vtkDiskSource& disk);

void vtk_disk_source_set_radial_resolution(vtkDiskSource& disk, int r);
int vtk_disk_source_get_radial_resolution(const vtkDiskSource& disk);

void vtk_disk_source_set_circumferential_resolution(vtkDiskSource& disk, int r);
int vtk_disk_source_get_circumferential_resolution(const vtkDiskSource& disk);

void vtk_disk_source_set_center(vtkDiskSource& disk, double x, double y, double z);
void vtk_disk_source_get_center(const vtkDiskSource& disk, double& x, double& y, double& z);

void vtk_disk_source_set_normal(vtkDiskSource& disk, double x, double y, double z);
void vtk_disk_source_get_normal(const vtkDiskSource& disk, double& x, double& y, double& z);

vtkAlgorithmOutput* vtk_disk_source_get_output_port(vtkDiskSource& disk);
