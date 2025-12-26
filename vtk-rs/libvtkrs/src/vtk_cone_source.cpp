#include <cstring>
#include <sstream>

#include "cxx.h"
#include "vtk_cone_source.h"
#include "vtk_cone_source.rs.h"

#include <vtkNew.h>
#include <vtkConeSource.h>
#include <vtkAlgorithmOutput.h>

vtkConeSource* vtk_cone_source_new() {
    vtkConeSource* obj = vtkConeSource::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkConeSource");
    }
    return obj;
}

void vtk_cone_source_delete(vtkConeSource& cone) {
    cone.Delete();
}

void vtk_cone_source_set_radius(vtkConeSource& cone, double radius) {
    cone.SetRadius(radius);
}

double vtk_cone_source_get_radius(const vtkConeSource& cone) {
    return const_cast<vtkConeSource&>(cone).GetRadius();
}

void vtk_cone_source_set_height(vtkConeSource& cone, double height) {
    cone.SetHeight(height);
}

double vtk_cone_source_get_height(const vtkConeSource& cone) {
    return const_cast<vtkConeSource&>(cone).GetHeight();
}

void vtk_cone_source_set_resolution(vtkConeSource& cone, int resolution) {
    cone.SetResolution(resolution);
}

int vtk_cone_source_get_resolution(const vtkConeSource& cone) {
    return const_cast<vtkConeSource&>(cone).GetResolution();
}

void vtk_cone_source_set_direction(vtkConeSource& cone, double x, double y, double z) {
    cone.SetDirection(x, y, z);
}

void vtk_cone_source_get_direction(const vtkConeSource& cone, double& x, double& y, double& z) {
    double* dir = const_cast<vtkConeSource&>(cone).GetDirection();
    x = dir[0];
    y = dir[1];
    z = dir[2];
}

void vtk_cone_source_set_center(vtkConeSource& cone, double x, double y, double z) {
    cone.SetCenter(x, y, z);
}

void vtk_cone_source_get_center(const vtkConeSource& cone, double& x, double& y, double& z) {
    double* ctr = const_cast<vtkConeSource&>(cone).GetCenter();
    x = ctr[0];
    y = ctr[1];
    z = ctr[2];
}

void vtk_cone_source_set_capping(vtkConeSource& cone, bool cap) {
    cone.SetCapping(cap);
}

bool vtk_cone_source_get_capping(const vtkConeSource& cone) {
    return const_cast<vtkConeSource&>(cone).GetCapping();
}

void vtk_cone_source_set_angle(vtkConeSource& cone, double angle) {
    cone.SetAngle(angle);
}

double vtk_cone_source_get_angle(const vtkConeSource& cone) {
    return const_cast<vtkConeSource&>(cone).GetAngle();
}

vtkAlgorithmOutput* vtk_cone_source_get_output_port(vtkConeSource& cone) {
    return cone.GetOutputPort();
}
