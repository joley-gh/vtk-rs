#pragma once

#include <vtkInteractorStyleTrackballCamera.h>

vtkInteractorStyleTrackballCamera* interactor_style_trackball_camera_new();
void interactor_style_trackball_camera_delete(vtkInteractorStyleTrackballCamera& style);
