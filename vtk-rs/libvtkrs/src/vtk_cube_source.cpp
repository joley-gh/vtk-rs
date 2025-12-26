#include <cstring>
#include <sstream>

#include "cxx.h"
#include "vtk_cube_source.h"
#include "vtk_cube_source.rs.h"

#include <vtkNew.h>
#include <vtkCubeSource.h>
#include <vtkAlgorithmOutput.h>

vtkCubeSource* vtk_cube_source_new() {
    vtkCubeSource* obj = vtkCubeSource::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkCubeSource");
    }
    return obj;
}

void vtk_cube_source_delete(vtkCubeSource& cube) {
    cube.Delete();
}

void vtk_cube_source_set_x_length(vtkCubeSource& cube, double length) {
    cube.SetXLength(length);
}

double vtk_cube_source_get_x_length(const vtkCubeSource& cube) {
    return const_cast<vtkCubeSource&>(cube).GetXLength();
}

void vtk_cube_source_set_y_length(vtkCubeSource& cube, double length) {
    cube.SetYLength(length);
}

double vtk_cube_source_get_y_length(const vtkCubeSource& cube) {
    return const_cast<vtkCubeSource&>(cube).GetYLength();
}

void vtk_cube_source_set_z_length(vtkCubeSource& cube, double length) {
    cube.SetZLength(length);
}

double vtk_cube_source_get_z_length(const vtkCubeSource& cube) {
    return const_cast<vtkCubeSource&>(cube).GetZLength();
}

void vtk_cube_source_set_center(vtkCubeSource& cube, double x, double y, double z) {
    cube.SetCenter(x, y, z);
}

void vtk_cube_source_get_center(const vtkCubeSource& cube, double& x, double& y, double& z) {
    double* ctr = const_cast<vtkCubeSource&>(cube).GetCenter();
    x = ctr[0];
    y = ctr[1];
    z = ctr[2];
}

void vtk_cube_source_set_bounds(vtkCubeSource& cube, double xmin, double xmax, double ymin, double ymax, double zmin, double zmax) {
    cube.SetBounds(xmin, xmax, ymin, ymax, zmin, zmax);
}

void vtk_cube_source_get_bounds(const vtkCubeSource& cube, double& xmin, double& xmax, double& ymin, double& ymax, double& zmin, double& zmax) {
    double bounds[6];
    const_cast<vtkCubeSource&>(cube).GetBounds(bounds);
    xmin = bounds[0];
    xmax = bounds[1];
    ymin = bounds[2];
    ymax = bounds[3];
    zmin = bounds[4];
    zmax = bounds[5];
}

vtkAlgorithmOutput* vtk_cube_source_get_output_port(vtkCubeSource& cube) {
    return cube.GetOutputPort();
}
