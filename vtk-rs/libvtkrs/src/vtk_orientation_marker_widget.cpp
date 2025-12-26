#include "cxx.h"
#include "vtk_orientation_marker_widget.h"
#include "vtk_orientation_marker_widget.rs.h"

#include <vtkOrientationMarkerWidget.h>
#include <vtkRenderWindowInteractor.h>
#include <vtkProp.h>

vtkOrientationMarkerWidget* orientation_marker_widget_new() {
    vtkOrientationMarkerWidget* obj = vtkOrientationMarkerWidget::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkOrientationMarkerWidget");
    }
    return obj;
}

void orientation_marker_widget_delete(vtkOrientationMarkerWidget& widget) {
    widget.Delete();
}

void orientation_marker_widget_set_orientation_marker(vtkOrientationMarkerWidget& widget, vtkProp* marker) {
    widget.SetOrientationMarker(marker);
}

void orientation_marker_widget_set_interactor(vtkOrientationMarkerWidget& widget, vtkRenderWindowInteractor* interactor) {
    widget.SetInteractor(interactor);
}

void orientation_marker_widget_set_viewport(vtkOrientationMarkerWidget& widget, double min_x, double min_y, double max_x, double max_y) {
    widget.SetViewport(min_x, min_y, max_x, max_y);
}

void orientation_marker_widget_set_enabled(vtkOrientationMarkerWidget& widget, int enabled) {
    widget.SetEnabled(enabled);
}

void orientation_marker_widget_interactive_off(vtkOrientationMarkerWidget& widget) {
    widget.InteractiveOff();
}
