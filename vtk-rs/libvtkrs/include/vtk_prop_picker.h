#pragma once

#include <vtkPropPicker.h>
#include <vtkRenderer.h>
#include <vtkProp.h>
#include <vtkActor.h>

vtkPropPicker* prop_picker_new();
void prop_picker_delete(vtkPropPicker& picker);
bool prop_picker_pick(vtkPropPicker& picker, double x, double y, double z, vtkRenderer* renderer);
void prop_picker_get_pick_position(vtkPropPicker& picker, double& x, double& y, double& z);
vtkProp* prop_picker_get_view_prop(vtkPropPicker& picker);
vtkActor* prop_picker_get_actor(vtkPropPicker& picker);
void prop_picker_add_pick_list(vtkPropPicker& picker, vtkProp* prop);
void prop_picker_set_pick_from_list(vtkPropPicker& picker, bool enabled);
