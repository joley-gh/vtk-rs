#include "cxx.h"
#include "vtk_camera.h"
#include "vtk_camera.rs.h"

#include <vtkCamera.h>

vtkCamera* camera_new() {
    vtkCamera* obj = vtkCamera::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkCamera");
    }
    return obj;
}

void camera_delete(vtkCamera& camera) {
    camera.Delete();
}

// Position and orientation
void camera_set_position(vtkCamera& camera, double x, double y, double z) {
    camera.SetPosition(x, y, z);
}

void camera_get_position(vtkCamera& camera, double& x, double& y, double& z) {
    double pos[3];
    camera.GetPosition(pos);
    x = pos[0];
    y = pos[1];
    z = pos[2];
}

void camera_set_focal_point(vtkCamera& camera, double x, double y, double z) {
    camera.SetFocalPoint(x, y, z);
}

void camera_get_focal_point(vtkCamera& camera, double& x, double& y, double& z) {
    double pos[3];
    camera.GetFocalPoint(pos);
    x = pos[0];
    y = pos[1];
    z = pos[2];
}

void camera_set_view_up(vtkCamera& camera, double x, double y, double z) {
    camera.SetViewUp(x, y, z);
}

void camera_get_view_up(vtkCamera& camera, double& x, double& y, double& z) {
    double pos[3];
    camera.GetViewUp(pos);
    x = pos[0];
    y = pos[1];
    z = pos[2];
}

// Camera movements
void camera_azimuth(vtkCamera& camera, double angle) {
    camera.Azimuth(angle);
}

void camera_elevation(vtkCamera& camera, double angle) {
    camera.Elevation(angle);
}

void camera_roll(vtkCamera& camera, double angle) {
    camera.Roll(angle);
}

void camera_zoom(vtkCamera& camera, double factor) {
    camera.Zoom(factor);
}

void camera_dolly(vtkCamera& camera, double factor) {
    camera.Dolly(factor);
}

// Clipping planes
void camera_set_clipping_range(vtkCamera& camera, double near, double far) {
    camera.SetClippingRange(near, far);
}

void camera_get_clipping_range(vtkCamera& camera, double& near, double& far) {
    double range[2];
    camera.GetClippingRange(range);
    near = range[0];
    far = range[1];
}
