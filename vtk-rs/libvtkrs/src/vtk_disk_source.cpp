#include "vtk_disk_source.h"
#include "vtk_disk_source.rs.h"

vtkDiskSource* vtk_disk_source_new() {
    return vtkDiskSource::New();
}

void vtk_disk_source_delete(vtkDiskSource& disk) {
    disk.Delete();
}

void vtk_disk_source_set_inner_radius(vtkDiskSource& disk, double radius) {
    disk.SetInnerRadius(radius);
}
double vtk_disk_source_get_inner_radius(const vtkDiskSource& disk) {
    return const_cast<vtkDiskSource&>(disk).GetInnerRadius();
}

void vtk_disk_source_set_outer_radius(vtkDiskSource& disk, double radius) {
    disk.SetOuterRadius(radius);
}
double vtk_disk_source_get_outer_radius(const vtkDiskSource& disk) {
    return const_cast<vtkDiskSource&>(disk).GetOuterRadius();
}

void vtk_disk_source_set_radial_resolution(vtkDiskSource& disk, int r) {
    disk.SetRadialResolution(r);
}
int vtk_disk_source_get_radial_resolution(const vtkDiskSource& disk) {
    return const_cast<vtkDiskSource&>(disk).GetRadialResolution();
}

void vtk_disk_source_set_circumferential_resolution(vtkDiskSource& disk, int r) {
    disk.SetCircumferentialResolution(r);
}
int vtk_disk_source_get_circumferential_resolution(const vtkDiskSource& disk) {
    return const_cast<vtkDiskSource&>(disk).GetCircumferentialResolution();
}

void vtk_disk_source_set_center(vtkDiskSource& disk, double x, double y, double z) {
    disk.SetCenter(x, y, z);
}
void vtk_disk_source_get_center(const vtkDiskSource& disk, double& x, double& y, double& z) {
    double* center = const_cast<vtkDiskSource&>(disk).GetCenter();
    x = center[0];
    y = center[1];
    z = center[2];
}

void vtk_disk_source_set_normal(vtkDiskSource& disk, double x, double y, double z) {
    disk.SetNormal(x, y, z);
}
void vtk_disk_source_get_normal(const vtkDiskSource& disk, double& x, double& y, double& z) {
    double* normal = const_cast<vtkDiskSource&>(disk).GetNormal();
    x = normal[0];
    y = normal[1];
    z = normal[2];
}

vtkAlgorithmOutput* vtk_disk_source_get_output_port(vtkDiskSource& disk) {
    return disk.GetOutputPort();
}
