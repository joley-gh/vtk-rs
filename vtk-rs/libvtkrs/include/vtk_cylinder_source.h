#include <vtkNew.h>
#include <vtkCylinderSource.h>
#include <vtkAlgorithmOutput.h>

vtkCylinderSource* vtk_cylinder_source_new();
void vtk_cylinder_source_delete(vtkCylinderSource& cylinder);
void vtk_cylinder_source_set_radius(vtkCylinderSource& cylinder, double radius);
double vtk_cylinder_source_get_radius(const vtkCylinderSource& cylinder);
void vtk_cylinder_source_set_height(vtkCylinderSource& cylinder, double height);
double vtk_cylinder_source_get_height(const vtkCylinderSource& cylinder);
void vtk_cylinder_source_set_resolution(vtkCylinderSource& cylinder, int resolution);
int vtk_cylinder_source_get_resolution(const vtkCylinderSource& cylinder);
void vtk_cylinder_source_set_center(vtkCylinderSource& cylinder, double x, double y, double z);
void vtk_cylinder_source_get_center(const vtkCylinderSource& cylinder, double& x, double& y, double& z);
void vtk_cylinder_source_set_capping(vtkCylinderSource& cylinder, bool cap);
bool vtk_cylinder_source_get_capping(const vtkCylinderSource& cylinder);
vtkAlgorithmOutput* vtk_cylinder_source_get_output_port(vtkCylinderSource& cylinder);
