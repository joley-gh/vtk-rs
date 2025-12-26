#pragma once

#include <vtkRenderWindowInteractor.h>
#include <vtkRenderWindow.h>
#include <vtkInteractorStyle.h>

vtkRenderWindowInteractor* render_window_interactor_new();
void render_window_interactor_delete(vtkRenderWindowInteractor& interactor);
void render_window_interactor_set_render_window(vtkRenderWindowInteractor& interactor, vtkRenderWindow* window);
void render_window_interactor_set_interactor_style(vtkRenderWindowInteractor& interactor, vtkInteractorStyle* style);
void render_window_interactor_initialize(vtkRenderWindowInteractor& interactor);
void render_window_interactor_start(vtkRenderWindowInteractor& interactor);

// Event handling
void render_window_interactor_get_event_position(vtkRenderWindowInteractor& interactor, int& x, int& y);
void render_window_interactor_get_last_event_position(vtkRenderWindowInteractor& interactor, int& x, int& y);
char render_window_interactor_get_key_code(vtkRenderWindowInteractor& interactor);
const char* render_window_interactor_get_key_sym(vtkRenderWindowInteractor& interactor);
bool render_window_interactor_get_shift_key(vtkRenderWindowInteractor& interactor);
bool render_window_interactor_get_control_key(vtkRenderWindowInteractor& interactor);
bool render_window_interactor_get_alt_key(vtkRenderWindowInteractor& interactor);
