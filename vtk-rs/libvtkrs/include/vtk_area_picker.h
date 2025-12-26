#pragma once

#include <vtkAreaPicker.h>
#include <vtkRenderer.h>

vtkAreaPicker* vtk_area_picker_new();
void vtk_area_picker_delete(vtkAreaPicker& picker);

int vtk_area_picker_area_pick(
    vtkAreaPicker& picker,
    double x0, double y0,
    double x1, double y1,
    vtkRenderer* renderer
);

vtkProp3DCollection* vtk_area_picker_get_prop3ds(vtkAreaPicker& picker);
