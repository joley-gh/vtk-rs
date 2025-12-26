#pragma once

#include <vtkCubeAxesActor.h>
#include <vtkCamera.h>
#include "cxx.h"

vtkCubeAxesActor* cube_axes_actor_new();
void cube_axes_actor_delete(vtkCubeAxesActor& actor);
void cube_axes_actor_set_bounds(vtkCubeAxesActor& actor, double x_min, double x_max, double y_min, double y_max, double z_min, double z_max);
void cube_axes_actor_set_camera(vtkCubeAxesActor& actor, vtkCamera* camera);
void cube_axes_actor_set_x_label(vtkCubeAxesActor& actor, rust::Str label);
void cube_axes_actor_set_y_label(vtkCubeAxesActor& actor, rust::Str label);
void cube_axes_actor_set_z_label(vtkCubeAxesActor& actor, rust::Str label);
void cube_axes_actor_draw_x_gridlines_on(vtkCubeAxesActor& actor);
void cube_axes_actor_draw_y_gridlines_on(vtkCubeAxesActor& actor);
void cube_axes_actor_draw_z_gridlines_on(vtkCubeAxesActor& actor);
