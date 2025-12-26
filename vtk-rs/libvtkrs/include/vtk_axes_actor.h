#pragma once

#include <vtkAxesActor.h>

vtkAxesActor* axes_actor_new();
void axes_actor_delete(vtkAxesActor& actor);
