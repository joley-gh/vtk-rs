#pragma once

#include <vtkRenderWindow.h>
#include <vtkRenderer.h>
#include "cxx.h"

vtkRenderWindow* render_window_new();
void render_window_delete(vtkRenderWindow& window);
void render_window_add_renderer(vtkRenderWindow& window, vtkRenderer* renderer);
void render_window_set_size(vtkRenderWindow& window, int width, int height);
void render_window_set_window_name(vtkRenderWindow& window, rust::Str name);
void render_window_render(vtkRenderWindow& window);
