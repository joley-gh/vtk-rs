#include <vtkNew.h>
#include <vtkConeSource.h>
#include <vtkAlgorithmOutput.h>

vtkConeSource* vtk_cone_source_new();
void vtk_cone_source_delete(vtkConeSource& cone);
void vtk_cone_source_set_radius(vtkConeSource& cone, double radius);
double vtk_cone_source_get_radius(const vtkConeSource& cone);
void vtk_cone_source_set_height(vtkConeSource& cone, double height);
double vtk_cone_source_get_height(const vtkConeSource& cone);
void vtk_cone_source_set_resolution(vtkConeSource& cone, int resolution);
int vtk_cone_source_get_resolution(const vtkConeSource& cone);
void vtk_cone_source_set_direction(vtkConeSource& cone, double x, double y, double z);
void vtk_cone_source_get_direction(const vtkConeSource& cone, double& x, double& y, double& z);
void vtk_cone_source_set_center(vtkConeSource& cone, double x, double y, double z);
void vtk_cone_source_get_center(const vtkConeSource& cone, double& x, double& y, double& z);
void vtk_cone_source_set_capping(vtkConeSource& cone, bool cap);
bool vtk_cone_source_get_capping(const vtkConeSource& cone);
void vtk_cone_source_set_angle(vtkConeSource& cone, double angle);
double vtk_cone_source_get_angle(const vtkConeSource& cone);
vtkAlgorithmOutput* vtk_cone_source_get_output_port(vtkConeSource& cone);
