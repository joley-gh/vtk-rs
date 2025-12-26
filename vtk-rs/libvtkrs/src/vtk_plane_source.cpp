#include "vtk_plane_source.h"
#include "vtk_plane_source.rs.h"

vtkPlaneSource* vtk_plane_source_new() {
    return vtkPlaneSource::New();
}

void vtk_plane_source_delete(vtkPlaneSource& plane) {
    plane.Delete();
}

void vtk_plane_source_set_origin(vtkPlaneSource& plane, double x, double y, double z) {
    plane.SetOrigin(x, y, z);
}

void vtk_plane_source_get_origin(const vtkPlaneSource& plane, double& x, double& y, double& z) {
    double* origin = const_cast<vtkPlaneSource&>(plane).GetOrigin();
    x = origin[0];
    y = origin[1];
    z = origin[2];
}

void vtk_plane_source_set_point1(vtkPlaneSource& plane, double x, double y, double z) {
    plane.SetPoint1(x, y, z);
}

void vtk_plane_source_get_point1(const vtkPlaneSource& plane, double& x, double& y, double& z) {
    double* point1 = const_cast<vtkPlaneSource&>(plane).GetPoint1();
    x = point1[0];
    y = point1[1];
    z = point1[2];
}

void vtk_plane_source_set_point2(vtkPlaneSource& plane, double x, double y, double z) {
    plane.SetPoint2(x, y, z);
}

void vtk_plane_source_get_point2(const vtkPlaneSource& plane, double& x, double& y, double& z) {
    double* point2 = const_cast<vtkPlaneSource&>(plane).GetPoint2();
    x = point2[0];
    y = point2[1];
    z = point2[2];
}

void vtk_plane_source_set_x_resolution(vtkPlaneSource& plane, int r) {
    plane.SetXResolution(r);
}

int vtk_plane_source_get_x_resolution(const vtkPlaneSource& plane) {
    return const_cast<vtkPlaneSource&>(plane).GetXResolution();
}

void vtk_plane_source_set_y_resolution(vtkPlaneSource& plane, int r) {
    plane.SetYResolution(r);
}

int vtk_plane_source_get_y_resolution(const vtkPlaneSource& plane) {
    return const_cast<vtkPlaneSource&>(plane).GetYResolution();
}

void vtk_plane_source_set_center(vtkPlaneSource& plane, double x, double y, double z) {
    plane.SetCenter(x, y, z);
}

void vtk_plane_source_set_normal(vtkPlaneSource& plane, double x, double y, double z) {
    plane.SetNormal(x, y, z);
}

void vtk_plane_source_push(vtkPlaneSource& plane, double distance) {
    plane.Push(distance);
}

vtkAlgorithmOutput* vtk_plane_source_get_output_port(vtkPlaneSource& plane) {
    return plane.GetOutputPort();
}
