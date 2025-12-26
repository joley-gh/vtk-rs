#include "vtk_superquadric_source.h"
#include "vtk_superquadric_source.rs.h"

vtkSuperquadricSource* vtk_superquadric_source_new() {
    return vtkSuperquadricSource::New();
}

void vtk_superquadric_source_delete(vtkSuperquadricSource& ptr) {
    ptr.Delete();
}

void superquadric_source_set_center(vtkSuperquadricSource& source, double x, double y, double z) {
    source.SetCenter(x, y, z);
}

void superquadric_source_get_center(vtkSuperquadricSource& source, double& x, double& y, double& z) {
    double* c = source.GetCenter();
    x = c[0];
    y = c[1];
    z = c[2];
}

void superquadric_source_set_scale(vtkSuperquadricSource& source, double x, double y, double z) {
    source.SetScale(x, y, z);
}

void superquadric_source_get_scale(vtkSuperquadricSource& source, double& x, double& y, double& z) {
    double* s = source.GetScale();
    x = s[0];
    y = s[1];
    z = s[2];
}

void superquadric_source_set_size(vtkSuperquadricSource& source, double size) {
    source.SetSize(size);
}

double superquadric_source_get_size(vtkSuperquadricSource& source) {
    return source.GetSize();
}

void superquadric_source_set_theta_roundness(vtkSuperquadricSource& source, double roundness) {
    source.SetThetaRoundness(roundness);
}

double superquadric_source_get_theta_roundness(vtkSuperquadricSource& source) {
    return source.GetThetaRoundness();
}

void superquadric_source_set_phi_roundness(vtkSuperquadricSource& source, double roundness) {
    source.SetPhiRoundness(roundness);
}

double superquadric_source_get_phi_roundness(vtkSuperquadricSource& source) {
    return source.GetPhiRoundness();
}

void superquadric_source_set_thickness(vtkSuperquadricSource& source, double thickness) {
    source.SetThickness(thickness);
}

double superquadric_source_get_thickness(vtkSuperquadricSource& source) {
    return source.GetThickness();
}

void superquadric_source_set_toroidal(vtkSuperquadricSource& source, bool toroidal) {
    source.SetToroidal(toroidal ? 1 : 0);
}

bool superquadric_source_get_toroidal(vtkSuperquadricSource& source) {
    return source.GetToroidal() != 0;
}

void superquadric_source_toroidal_on(vtkSuperquadricSource& source) {
    source.ToroidalOn();
}

void superquadric_source_toroidal_off(vtkSuperquadricSource& source) {
    source.ToroidalOff();
}

void superquadric_source_set_theta_resolution(vtkSuperquadricSource& source, int resolution) {
    source.SetThetaResolution(resolution);
}

int superquadric_source_get_theta_resolution(vtkSuperquadricSource& source) {
    return source.GetThetaResolution();
}

void superquadric_source_set_phi_resolution(vtkSuperquadricSource& source, int resolution) {
    source.SetPhiResolution(resolution);
}

int superquadric_source_get_phi_resolution(vtkSuperquadricSource& source) {
    return source.GetPhiResolution();
}

vtkAlgorithmOutput* superquadric_source_get_output_port(vtkSuperquadricSource& source) {
    return source.GetOutputPort();
}
