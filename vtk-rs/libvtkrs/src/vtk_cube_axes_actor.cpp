#include "cxx.h"
#include "vtk_cube_axes_actor.h"
#include "vtk_cube_axes_actor.rs.h"

#include <vtkCubeAxesActor.h>
#include <vtkCamera.h>
#include <cstring>

vtkCubeAxesActor* cube_axes_actor_new() {
    vtkCubeAxesActor* obj = vtkCubeAxesActor::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkCubeAxesActor");
    }
    return obj;
}

void cube_axes_actor_delete(vtkCubeAxesActor& actor) {
    actor.Delete();
}

void cube_axes_actor_set_bounds(vtkCubeAxesActor& actor, double x_min, double x_max, double y_min, double y_max, double z_min, double z_max) {
    actor.SetBounds(x_min, x_max, y_min, y_max, z_min, z_max);
}

void cube_axes_actor_set_camera(vtkCubeAxesActor& actor, vtkCamera* camera) {
    actor.SetCamera(camera);
}

void cube_axes_actor_set_x_label(vtkCubeAxesActor& actor, rust::Str label) {
    std::string str(label);
    actor.SetXTitle(str.c_str());
}

void cube_axes_actor_set_y_label(vtkCubeAxesActor& actor, rust::Str label) {
    std::string str(label);
    actor.SetYTitle(str.c_str());
}

void cube_axes_actor_set_z_label(vtkCubeAxesActor& actor, rust::Str label) {
    std::string str(label);
    actor.SetZTitle(str.c_str());
}

void cube_axes_actor_draw_x_gridlines_on(vtkCubeAxesActor& actor) {
    actor.DrawXGridlinesOn();
}

void cube_axes_actor_draw_y_gridlines_on(vtkCubeAxesActor& actor) {
    actor.DrawYGridlinesOn();
}

void cube_axes_actor_draw_z_gridlines_on(vtkCubeAxesActor& actor) {
    actor.DrawZGridlinesOn();
}
