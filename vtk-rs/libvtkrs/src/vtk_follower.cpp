#include "vtk_follower.h"
#include "vtk_follower.rs.h"

vtkFollower* vtk_follower_new() {
    return vtkFollower::New();
}

void vtk_follower_delete(vtkFollower& ptr) {
    ptr.Delete();
}

void follower_set_camera(vtkFollower& follower, uintptr_t camera) {
    follower.SetCamera(reinterpret_cast<vtkCamera*>(camera));
}

void follower_set_mapper(vtkFollower& follower, uintptr_t mapper) {
    follower.SetMapper(reinterpret_cast<vtkMapper*>(mapper));
}

void follower_set_position(vtkFollower& follower, double x, double y, double z) {
    follower.SetPosition(x, y, z);
}

void follower_get_position(vtkFollower& follower, double& x, double& y, double& z) {
    double* pos = follower.GetPosition();
    x = pos[0];
    y = pos[1];
    z = pos[2];
}

void follower_set_scale(vtkFollower& follower, double x, double y, double z) {
    follower.SetScale(x, y, z);
}

void follower_get_scale(vtkFollower& follower, double& x, double& y, double& z) {
    double* scale = follower.GetScale();
    x = scale[0];
    y = scale[1];
    z = scale[2];
}
