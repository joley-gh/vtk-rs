#include "vtk_area_picker.h"
#include "vtk_area_picker.rs.h"

vtkAreaPicker* vtk_area_picker_new() {
    return vtkAreaPicker::New();
}

void vtk_area_picker_delete(vtkAreaPicker& picker) {
    picker.Delete();
}

int vtk_area_picker_area_pick(
    vtkAreaPicker& picker,
    double x0, double y0,
    double x1, double y1,
    vtkRenderer* renderer
) {
    if (renderer) {
        return picker.AreaPick(x0, y0, x1, y1, renderer);
    }
    return 0;
}

vtkProp3DCollection* vtk_area_picker_get_prop3ds(vtkAreaPicker& picker) {
    return picker.GetProp3Ds();
}
