#pragma once

#include <vtkNew.h>
#include <vtkPointPicker.h>
#include <vtkRenderer.h>

vtkPointPicker* vtk_point_picker_new();
void vtk_point_picker_delete(vtkPointPicker& picker);

int vtk_point_picker_pick(vtkPointPicker& picker, double x, double y, double z, vtkRenderer* renderer);
int vtk_point_picker_get_point_id(vtkPointPicker& picker);
void vtk_point_picker_get_pick_position(vtkPointPicker& picker, double& x, double& y, double& z);
