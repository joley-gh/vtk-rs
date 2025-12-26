#include "cxx.h"
#include "vtk_cell_picker.h"
#include "vtk_cell_picker.rs.h"

#include <vtkCellPicker.h>
#include <iostream>

vtkCellPicker* cell_picker_new() {
    vtkCellPicker* obj = vtkCellPicker::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkCellPicker");
    }
    return obj;
}

void cell_picker_delete(vtkCellPicker& picker) {
    picker.Delete();
}

bool cell_picker_pick(vtkCellPicker& picker, double x, double y, double z, vtkRenderer* renderer) {
    return picker.Pick(x, y, z, renderer) != 0;
}

int cell_picker_get_cell_id(vtkCellPicker& picker) {
    return picker.GetCellId();
}

vtkDataSet* cell_picker_get_dataset(vtkCellPicker& picker) {
    return picker.GetDataSet();
}

void cell_picker_get_pick_position(vtkCellPicker& picker, double& x, double& y, double& z) {
    double* pos = picker.GetPickPosition();
    x = pos[0];
    y = pos[1];
    z = pos[2];
}

vtkActor* cell_picker_get_actor(vtkCellPicker& picker) {
    return picker.GetActor();
}

void cell_picker_add_pick_list(vtkCellPicker& picker, vtkProp3D* prop) {
    picker.AddPickList(prop);
}

void cell_picker_set_pick_from_list(vtkCellPicker& picker, bool enabled) {
    picker.SetPickFromList(enabled ? 1 : 0);
}

void cell_picker_set_tolerance(vtkCellPicker& picker, double tolerance) {
    picker.SetTolerance(tolerance);
}
