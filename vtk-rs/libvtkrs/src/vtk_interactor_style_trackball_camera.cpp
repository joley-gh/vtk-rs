#include "cxx.h"
#include "vtk_interactor_style_trackball_camera.h"
#include "vtk_interactor_style_trackball_camera.rs.h"

#include <vtkInteractorStyleTrackballCamera.h>

vtkInteractorStyleTrackballCamera* interactor_style_trackball_camera_new() {
    vtkInteractorStyleTrackballCamera* obj = vtkInteractorStyleTrackballCamera::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkInteractorStyleTrackballCamera");
    }
    return obj;
}

void interactor_style_trackball_camera_delete(vtkInteractorStyleTrackballCamera& style) {
    style.Delete();
}
