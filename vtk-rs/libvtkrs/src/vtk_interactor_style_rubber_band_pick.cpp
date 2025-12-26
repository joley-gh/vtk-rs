#include "vtk_interactor_style_rubber_band_pick.h"
#include <vtkInteractorStyleRubberBandPick.h>
#include <vtkRenderWindowInteractor.h>

extern "C" {

vtkInteractorStyleRubberBandPick* interactor_style_rubber_band_pick_new() {
    return vtkInteractorStyleRubberBandPick::New();
}

void interactor_style_rubber_band_pick_delete(vtkInteractorStyleRubberBandPick* style) {
    if (style) {
        style->Delete();
    }
}

void interactor_style_rubber_band_pick_set_interactor(
    vtkInteractorStyleRubberBandPick* style,
    vtkRenderWindowInteractor* interactor
) {
    if (style && interactor) {
        style->SetInteractor(interactor);
    }
}

} // extern "C"
