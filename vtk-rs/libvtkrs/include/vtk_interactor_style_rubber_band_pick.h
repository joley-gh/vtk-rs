#pragma once

#include "cxx.h"
#include <vtkInteractorStyleRubberBandPick.h>
#include <vtkRenderWindowInteractor.h>

extern "C" {
    vtkInteractorStyleRubberBandPick* interactor_style_rubber_band_pick_new();
    void interactor_style_rubber_band_pick_delete(vtkInteractorStyleRubberBandPick* style);
    void interactor_style_rubber_band_pick_set_interactor(
        vtkInteractorStyleRubberBandPick* style,
        vtkRenderWindowInteractor* interactor
    );
}
