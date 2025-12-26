#include <cstring>
#include <sstream>

#include "cxx.h"
#include "vtk_cylinder_source.h"
#include "vtk_cylinder_source.rs.h"

#include <vtkNew.h>
#include <vtkCylinderSource.h>
#include <vtkAlgorithmOutput.h>

vtkCylinderSource* vtk_cylinder_source_new() {
    vtkCylinderSource* obj = vtkCylinderSource::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkCylinderSource");
    }
    return obj;
}

void vtk_cylinder_source_delete(vtkCylinderSource& cylinder) {
    cylinder.Delete();
}

void vtk_cylinder_source_set_radius(vtkCylinderSource& cylinder, double radius) {
    cylinder.SetRadius(radius);
}

double vtk_cylinder_source_get_radius(const vtkCylinderSource& cylinder) {
    return const_cast<vtkCylinderSource&>(cylinder).GetRadius();
}

void vtk_cylinder_source_set_height(vtkCylinderSource& cylinder, double height) {
    cylinder.SetHeight(height);
}

double vtk_cylinder_source_get_height(const vtkCylinderSource& cylinder) {
    return const_cast<vtkCylinderSource&>(cylinder).GetHeight();
}

void vtk_cylinder_source_set_resolution(vtkCylinderSource& cylinder, int resolution) {
    cylinder.SetResolution(resolution);
}

int vtk_cylinder_source_get_resolution(const vtkCylinderSource& cylinder) {
    return const_cast<vtkCylinderSource&>(cylinder).GetResolution();
}

void vtk_cylinder_source_set_center(vtkCylinderSource& cylinder, double x, double y, double z) {
    cylinder.SetCenter(x, y, z);
}

void vtk_cylinder_source_get_center(const vtkCylinderSource& cylinder, double& x, double& y, double& z) {
    double* ctr = const_cast<vtkCylinderSource&>(cylinder).GetCenter();
    x = ctr[0];
    y = ctr[1];
    z = ctr[2];
}

void vtk_cylinder_source_set_capping(vtkCylinderSource& cylinder, bool cap) {
    cylinder.SetCapping(cap);
}

bool vtk_cylinder_source_get_capping(const vtkCylinderSource& cylinder) {
    return const_cast<vtkCylinderSource&>(cylinder).GetCapping();
}

vtkAlgorithmOutput* vtk_cylinder_source_get_output_port(vtkCylinderSource& cylinder) {
    return cylinder.GetOutputPort();
}
