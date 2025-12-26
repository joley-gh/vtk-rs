#include "cxx.h"
#include "vtk_prop_picker.h"
#include "vtk_prop_picker.rs.h"

#include <vtkPropPicker.h>

vtkPropPicker* prop_picker_new() {
    vtkPropPicker* obj = vtkPropPicker::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkPropPicker");
    }
    return obj;
}

void prop_picker_delete(vtkPropPicker& picker) {
    picker.Delete();
}

bool prop_picker_pick(vtkPropPicker& picker, double x, double y, double z, vtkRenderer* renderer) {
    return picker.Pick(x, y, z, renderer) != 0;
}

void prop_picker_get_pick_position(vtkPropPicker& picker, double& x, double& y, double& z) {
    double* pos = picker.GetPickPosition();
    x = pos[0];
    y = pos[1];
    z = pos[2];
}

vtkProp* prop_picker_get_view_prop(vtkPropPicker& picker) {
    return picker.GetViewProp();
}

vtkActor* prop_picker_get_actor(vtkPropPicker& picker) {
    return picker.GetActor();
}

void prop_picker_add_pick_list(vtkPropPicker& picker, vtkProp* prop) {
    picker.AddPickList(prop);
}

void prop_picker_set_pick_from_list(vtkPropPicker& picker, bool enabled) {
    picker.SetPickFromList(enabled);
}
