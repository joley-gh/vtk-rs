#ifndef VTK_SUPERQUADRIC_SOURCE_H
#define VTK_SUPERQUADRIC_SOURCE_H

#include <vtkSuperquadricSource.h>

vtkSuperquadricSource* vtk_superquadric_source_new();
void vtk_superquadric_source_delete(vtkSuperquadricSource& ptr);

void superquadric_source_set_center(vtkSuperquadricSource& source, double x, double y, double z);
void superquadric_source_get_center(vtkSuperquadricSource& source, double& x, double& y, double& z);

void superquadric_source_set_scale(vtkSuperquadricSource& source, double x, double y, double z);
void superquadric_source_get_scale(vtkSuperquadricSource& source, double& x, double& y, double& z);

void superquadric_source_set_size(vtkSuperquadricSource& source, double size);
double superquadric_source_get_size(vtkSuperquadricSource& source);

void superquadric_source_set_theta_roundness(vtkSuperquadricSource& source, double roundness);
double superquadric_source_get_theta_roundness(vtkSuperquadricSource& source);

void superquadric_source_set_phi_roundness(vtkSuperquadricSource& source, double roundness);
double superquadric_source_get_phi_roundness(vtkSuperquadricSource& source);

void superquadric_source_set_thickness(vtkSuperquadricSource& source, double thickness);
double superquadric_source_get_thickness(vtkSuperquadricSource& source);

void superquadric_source_set_toroidal(vtkSuperquadricSource& source, bool toroidal);
bool superquadric_source_get_toroidal(vtkSuperquadricSource& source);
void superquadric_source_toroidal_on(vtkSuperquadricSource& source);
void superquadric_source_toroidal_off(vtkSuperquadricSource& source);

void superquadric_source_set_theta_resolution(vtkSuperquadricSource& source, int resolution);
int superquadric_source_get_theta_resolution(vtkSuperquadricSource& source);

void superquadric_source_set_phi_resolution(vtkSuperquadricSource& source, int resolution);
int superquadric_source_get_phi_resolution(vtkSuperquadricSource& source);

vtkAlgorithmOutput* superquadric_source_get_output_port(vtkSuperquadricSource& source);

#endif // VTK_SUPERQUADRIC_SOURCE_H
