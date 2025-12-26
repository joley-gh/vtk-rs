#include "vtk_point_picker.h"
#include "vtk_point_picker.rs.h"

vtkPointPicker* vtk_point_picker_new() {
    return vtkPointPicker::New();
}

void vtk_point_picker_delete(vtkPointPicker& picker) {
    picker.Delete();
}

int vtk_point_picker_pick(vtkPointPicker& picker, double x, double y, double z, vtkRenderer* renderer) {
    return picker.Pick(x, y, z, renderer);
}

int vtk_point_picker_get_point_id(vtkPointPicker& picker) {
    return picker.GetPointId();
}

void vtk_point_picker_get_pick_position(vtkPointPicker& picker, double& x, double& y, double& z) {
    double* pos = picker.GetPickPosition();
    x = pos[0];
    y = pos[1];
    z = pos[2];
}
