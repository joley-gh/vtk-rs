#pragma once

#include <vtkCamera.h>

vtkCamera* camera_new();
void camera_delete(vtkCamera& camera);

// Position and orientation
void camera_set_position(vtkCamera& camera, double x, double y, double z);
void camera_get_position(vtkCamera& camera, double& x, double& y, double& z);
void camera_set_focal_point(vtkCamera& camera, double x, double y, double z);
void camera_get_focal_point(vtkCamera& camera, double& x, double& y, double& z);
void camera_set_view_up(vtkCamera& camera, double x, double y, double z);
void camera_get_view_up(vtkCamera& camera, double& x, double& y, double& z);

// Camera movements
void camera_azimuth(vtkCamera& camera, double angle);
void camera_elevation(vtkCamera& camera, double angle);
void camera_roll(vtkCamera& camera, double angle);
void camera_zoom(vtkCamera& camera, double factor);
void camera_dolly(vtkCamera& camera, double factor);

// Clipping planes
void camera_set_clipping_range(vtkCamera& camera, double near, double far);
void camera_get_clipping_range(vtkCamera& camera, double& near, double& far);
