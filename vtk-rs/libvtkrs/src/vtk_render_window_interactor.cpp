#include "cxx.h"
#include "vtk_render_window_interactor.h"
#include "vtk_render_window_interactor.rs.h"

#include <vtkRenderWindowInteractor.h>
#include <vtkRenderWindow.h>
#include <vtkInteractorStyle.h>

vtkRenderWindowInteractor* render_window_interactor_new() {
    vtkRenderWindowInteractor* obj = vtkRenderWindowInteractor::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkRenderWindowInteractor");
    }
    return obj;
}

void render_window_interactor_delete(vtkRenderWindowInteractor& interactor) {
    interactor.Delete();
}

void render_window_interactor_set_render_window(
    vtkRenderWindowInteractor& interactor,
    vtkRenderWindow* window
) {
    interactor.SetRenderWindow(window);
}

void render_window_interactor_set_interactor_style(
    vtkRenderWindowInteractor& interactor,
    vtkInteractorStyle* style
) {
    interactor.SetInteractorStyle(style);
}

void render_window_interactor_initialize(vtkRenderWindowInteractor& interactor) {
    interactor.Initialize();
}

void render_window_interactor_start(vtkRenderWindowInteractor& interactor) {
    interactor.Start();
}

// Event handling
void render_window_interactor_get_event_position(vtkRenderWindowInteractor& interactor, int& x, int& y) {
    int* pos = interactor.GetEventPosition();
    x = pos[0];
    y = pos[1];
}

void render_window_interactor_get_last_event_position(vtkRenderWindowInteractor& interactor, int& x, int& y) {
    int* pos = interactor.GetLastEventPosition();
    x = pos[0];
    y = pos[1];
}

char render_window_interactor_get_key_code(vtkRenderWindowInteractor& interactor) {
    return interactor.GetKeyCode();
}

const char* render_window_interactor_get_key_sym(vtkRenderWindowInteractor& interactor) {
    return interactor.GetKeySym();
}

bool render_window_interactor_get_shift_key(vtkRenderWindowInteractor& interactor) {
    return interactor.GetShiftKey() != 0;
}

bool render_window_interactor_get_control_key(vtkRenderWindowInteractor& interactor) {
    return interactor.GetControlKey() != 0;
}

bool render_window_interactor_get_alt_key(vtkRenderWindowInteractor& interactor) {
    return interactor.GetAltKey() != 0;
}
