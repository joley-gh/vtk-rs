#include "vtk_interactor_style_custom.h"
#include <vtkRenderWindowInteractor.h>
#include <cstdio>

// Static initializer to confirm this file is being linked
struct DebugInit {
    DebugInit() {
        fprintf(stderr, "DEBUG: vtk_interactor_style_custom.cpp loaded\n");
        fflush(stderr);
    }
};
static DebugInit debug_init;

// Direct instantiation without VTK factory
vtkInteractorStyleCustom* vtkInteractorStyleCustom::New() {
    fprintf(stderr, "DEBUG: vtkInteractorStyleCustom::New() called\n");
    fflush(stderr);
    
    // Just use plain new without any VTK machinery
    vtkInteractorStyleCustom* result = new vtkInteractorStyleCustom();
    
    fprintf(stderr, "DEBUG: Created object at %p\n", (void*)result);
    fflush(stderr);
    
    return result;
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

void vtkInteractorStyleCustom::OnLeftButtonDown() {
    if (this->left_press_callback_id != 0) {
        int* pos = this->Interactor->GetEventPosition();
        vtk_rs_left_button_press_callback(this->left_press_callback_id, pos[0], pos[1]);
    }
    // Call parent implementation for default camera controls
    vtkInteractorStyleTrackballCamera::OnLeftButtonDown();
}

void vtkInteractorStyleCustom::OnLeftButtonUp() {
    if (this->left_release_callback_id != 0) {
        int* pos = this->Interactor->GetEventPosition();
        vtk_rs_left_button_release_callback(this->left_release_callback_id, pos[0], pos[1]);
    }
    // Call parent implementation for default camera controls
    vtkInteractorStyleTrackballCamera::OnLeftButtonUp();
}

void vtkInteractorStyleCustom::OnMouseMove() {
    if (this->mouse_move_callback_id != 0) {
        int* pos = this->Interactor->GetEventPosition();
        vtk_rs_mouse_move_callback(this->mouse_move_callback_id, pos[0], pos[1]);
    }
    // Call parent implementation for default camera controls
    vtkInteractorStyleTrackballCamera::OnMouseMove();
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
        fprintf(stderr, "DEBUG: C wrapper interactor_style_custom_new() called\n");
        fflush(stderr);
        vtkInteractorStyleCustom* result = vtkInteractorStyleCustom::New();
        fprintf(stderr, "DEBUG: C wrapper got result: %p\n", (void*)result);
        fflush(stderr);
        return result;
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

    // Test function to verify linking
    int vtk_test_link_interactor_style() {
        fprintf(stderr, "TEST: vtk_test_link_interactor_style called!\n");
        fflush(stderr);
        return 42;
    }
}
