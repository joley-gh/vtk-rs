#pragma once

#include <vtkOrientationMarkerWidget.h>
#include <vtkRenderWindowInteractor.h>
#include <vtkProp.h>

vtkOrientationMarkerWidget* orientation_marker_widget_new();
void orientation_marker_widget_delete(vtkOrientationMarkerWidget& widget);
void orientation_marker_widget_set_orientation_marker(vtkOrientationMarkerWidget& widget, vtkProp* marker);
void orientation_marker_widget_set_interactor(vtkOrientationMarkerWidget& widget, vtkRenderWindowInteractor* interactor);
void orientation_marker_widget_set_viewport(vtkOrientationMarkerWidget& widget, double min_x, double min_y, double max_x, double max_y);
void orientation_marker_widget_set_enabled(vtkOrientationMarkerWidget& widget, int enabled);
void orientation_marker_widget_interactive_off(vtkOrientationMarkerWidget& widget);
