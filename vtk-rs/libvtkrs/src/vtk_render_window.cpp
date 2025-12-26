#include "cxx.h"
#include "vtk_render_window.h"
#include "vtk_render_window.rs.h"

#include <vtkRenderWindow.h>
#include <vtkRenderer.h>
#include <string>

vtkRenderWindow* render_window_new() {
    vtkRenderWindow* obj = vtkRenderWindow::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkRenderWindow");
    }
    return obj;
}

void render_window_delete(vtkRenderWindow& window) {
    window.Delete();
}

void render_window_add_renderer(vtkRenderWindow& window, vtkRenderer* renderer) {
    window.AddRenderer(renderer);
}

void render_window_set_size(vtkRenderWindow& window, int width, int height) {
    window.SetSize(width, height);
}

void render_window_set_window_name(vtkRenderWindow& window, rust::Str name) {
    window.SetWindowName(std::string(name).c_str());
}

void render_window_render(vtkRenderWindow& window) {
    window.Render();
}
