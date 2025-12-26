#include <vtkNew.h>
#include <vtkCubeSource.h>
#include <vtkAlgorithmOutput.h>

vtkCubeSource* vtk_cube_source_new();
void vtk_cube_source_delete(vtkCubeSource& cube);
void vtk_cube_source_set_x_length(vtkCubeSource& cube, double length);
double vtk_cube_source_get_x_length(const vtkCubeSource& cube);
void vtk_cube_source_set_y_length(vtkCubeSource& cube, double length);
double vtk_cube_source_get_y_length(const vtkCubeSource& cube);
void vtk_cube_source_set_z_length(vtkCubeSource& cube, double length);
double vtk_cube_source_get_z_length(const vtkCubeSource& cube);
void vtk_cube_source_set_center(vtkCubeSource& cube, double x, double y, double z);
void vtk_cube_source_get_center(const vtkCubeSource& cube, double& x, double& y, double& z);
void vtk_cube_source_set_bounds(vtkCubeSource& cube, double xmin, double xmax, double ymin, double ymax, double zmin, double zmax);
void vtk_cube_source_get_bounds(const vtkCubeSource& cube, double& xmin, double& xmax, double& ymin, double& ymax, double& zmin, double& zmax);
vtkAlgorithmOutput* vtk_cube_source_get_output_port(vtkCubeSource& cube);
