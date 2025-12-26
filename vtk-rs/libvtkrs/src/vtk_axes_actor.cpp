#include "cxx.h"
#include "vtk_axes_actor.h"
#include "vtk_axes_actor.rs.h"

#include <vtkAxesActor.h>

vtkAxesActor* axes_actor_new() {
    vtkAxesActor* obj = vtkAxesActor::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkAxesActor");
    }
    return obj;
}

void axes_actor_delete(vtkAxesActor& actor) {
    actor.Delete();
}
