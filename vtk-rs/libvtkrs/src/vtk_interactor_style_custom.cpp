#include "vtk_interactor_style_custom.h"
#include <vtkRenderWindowInteractor.h>
#include <vtkRenderWindow.h>
#include <cstdio>

// Direct instantiation without VTK factory
vtkInteractorStyleCustom* vtkInteractorStyleCustom::New() {
    return new vtkInteractorStyleCustom();
}

vtkInteractorStyleCustom::vtkInteractorStyleCustom() {
    // Initialize callback IDs to zero (no callbacks set)
    this->left_press_callback_id = 0;
    this->left_release_callback_id = 0;
    this->mouse_move_callback_id = 0;
    this->key_press_callback_id = 0;
}

void vtkInteractorStyleCustom::SetLeftButtonPressCallbackId(int64_t callback_id) {
    this->left_press_callback_id = callback_id;
}

void vtkInteractorStyleCustom::SetLeftButtonReleaseCallbackId(int64_t callback_id) {
    this->left_release_callback_id = callback_id;
}

void vtkInteractorStyleCustom::SetMouseMoveCallbackId(int64_t callback_id) {
    this->mouse_move_callback_id = callback_id;
}

void vtkInteractorStyleCustom::SetKeyPressCallbackId(int64_t callback_id) {
    this->key_press_callback_id = callback_id;
}

void vtkInteractorStyleCustom::SetSelectionMode(bool enabled) {
    this->selection_mode = enabled;
}

bool vtkInteractorStyleCustom::GetSelectionMode() const {
    return this->selection_mode;
}

void vtkInteractorStyleCustom::StartSelect() {
    int* pos = this->Interactor->GetEventPosition();
    this->start_position[0] = pos[0];
    this->start_position[1] = pos[1];
    this->end_position[0] = pos[0];
    this->end_position[1] = pos[1];
    this->moving = true;
}

void vtkInteractorStyleCustom::EndSelect() {
    this->moving = false;
}

void vtkInteractorStyleCustom::OnLeftButtonDown() {
    if (this->selection_mode) {
        StartSelect();
    }
    if (this->left_press_callback_id != 0) {
        int* pos = this->Interactor->GetEventPosition();
        vtk_rs_left_button_press_callback(this->left_press_callback_id, pos[0], pos[1]);
    }
    // Only call parent (camera controls) if not in selection mode
    if (!this->selection_mode) {
        vtkInteractorStyleTrackballCamera::OnLeftButtonDown();
    }
}

void vtkInteractorStyleCustom::OnLeftButtonUp() {
    if (this->selection_mode) {
        EndSelect();
    }
    if (this->left_release_callback_id != 0) {
        int* pos = this->Interactor->GetEventPosition();
        vtk_rs_left_button_release_callback(this->left_release_callback_id, pos[0], pos[1]);
    }
    // Only call parent (camera controls) if not in selection mode
    if (!this->selection_mode) {
        vtkInteractorStyleTrackballCamera::OnLeftButtonUp();
    }
}

void vtkInteractorStyleCustom::OnMouseMove() {
    if (this->selection_mode && this->moving) {
        int* pos = this->Interactor->GetEventPosition();
        this->end_position[0] = pos[0];
        this->end_position[1] = pos[1];
        // Rust callback will handle rubber band drawing
    }
    if (this->mouse_move_callback_id != 0) {
        int* pos = this->Interactor->GetEventPosition();
        vtk_rs_mouse_move_callback(this->mouse_move_callback_id, pos[0], pos[1]);
    }
    // Only call parent (camera controls) if not in selection mode
    if (!this->selection_mode) {
        vtkInteractorStyleTrackballCamera::OnMouseMove();
    }
}

void vtkInteractorStyleCustom::OnKeyPress() {
    if (this->key_press_callback_id != 0) {
        vtkRenderWindowInteractor* rwi = this->Interactor;
        std::string key = rwi->GetKeySym();
        vtk_rs_key_press_callback(this->key_press_callback_id, key.c_str());
    }
    // Call parent implementation for default key handling
    vtkInteractorStyleTrackballCamera::OnKeyPress();
}

// C-style wrapper functions with extern "C" linkage
extern "C" {
    vtkInteractorStyleCustom* interactor_style_custom_new() {
        return vtkInteractorStyleCustom::New();
    }

    void interactor_style_custom_delete(vtkInteractorStyleCustom* style) {
        style->Delete();
    }

    void interactor_style_custom_set_left_button_press_callback_id(
        vtkInteractorStyleCustom* style,
        int64_t callback_id
    ) {
        style->SetLeftButtonPressCallbackId(callback_id);
    }

    void interactor_style_custom_set_left_button_release_callback_id(
        vtkInteractorStyleCustom* style,
        int64_t callback_id
    ) {
        style->SetLeftButtonReleaseCallbackId(callback_id);
    }

    void interactor_style_custom_set_mouse_move_callback_id(
        vtkInteractorStyleCustom* style,
        int64_t callback_id
    ) {
        style->SetMouseMoveCallbackId(callback_id);
    }

    void interactor_style_custom_set_key_press_callback_id(
        vtkInteractorStyleCustom* style,
        int64_t callback_id
    ) {
        style->SetKeyPressCallbackId(callback_id);
    }

    void interactor_style_custom_set_selection_mode(
        vtkInteractorStyleCustom* style,
        bool enabled
    ) {
        if (style) {
            style->SetSelectionMode(enabled);
        }
    }

    bool interactor_style_custom_is_moving(
        vtkInteractorStyleCustom* style
    ) {
        return style ? style->GetMoving() : false;
    }

    void interactor_style_custom_get_selection_positions(
        vtkInteractorStyleCustom* style,
        int* start_x, int* start_y,
        int* end_x, int* end_y
    ) {
        if (style) {
            style->GetSelectionPositions(start_x, start_y, end_x, end_y);
        }
    }

    // Test function to verify linking
    int vtk_test_link_interactor_style() {
        fprintf(stderr, "TEST: vtk_test_link_interactor_style called!\n");
        fflush(stderr);
        return 42;
    }
}
