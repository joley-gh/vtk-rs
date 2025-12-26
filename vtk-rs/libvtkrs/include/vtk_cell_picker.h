#pragma once

#include <vtkCellPicker.h>
#include <vtkRenderer.h>
#include <vtkActor.h>
#include <vtkDataSet.h>
#include <vtkProp3D.h>

vtkCellPicker* cell_picker_new();
void cell_picker_delete(vtkCellPicker& picker);
bool cell_picker_pick(vtkCellPicker& picker, double x, double y, double z, vtkRenderer* renderer);
int cell_picker_get_cell_id(vtkCellPicker& picker);
vtkDataSet* cell_picker_get_dataset(vtkCellPicker& picker);
void cell_picker_get_pick_position(vtkCellPicker& picker, double& x, double& y, double& z);
vtkActor* cell_picker_get_actor(vtkCellPicker& picker);
void cell_picker_add_pick_list(vtkCellPicker& picker, vtkProp3D* prop);
void cell_picker_set_pick_from_list(vtkCellPicker& picker, bool enabled);
void cell_picker_set_tolerance(vtkCellPicker& picker, double tolerance);
