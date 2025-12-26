#pragma once

#include <vtkInteractorStyleTrackballCamera.h>
#include <vtkRenderWindowInteractor.h>
#include <vtkRenderer.h>
#include <cstdint>

// Extern C callbacks defined in Rust
extern "C" {
    void vtk_rs_left_button_press_callback(int64_t callback_id, int x, int y);
    void vtk_rs_left_button_release_callback(int64_t callback_id, int x, int y);
    void vtk_rs_mouse_move_callback(int64_t callback_id, int x, int y);
    void vtk_rs_key_press_callback(int64_t callback_id, const char* key);
}

// Custom interactor style that allows Rust callbacks
class vtkInteractorStyleCustom : public vtkInteractorStyleTrackballCamera {
public:
    static vtkInteractorStyleCustom* New();
    
    // Type information (simplified, not using vtkTypeMacro to avoid issues)
    const char* GetClassName() { return "vtkInteractorStyleCustom"; }

    void SetLeftButtonPressCallbackId(int64_t callback_id);
    void SetLeftButtonReleaseCallbackId(int64_t callback_id);
    void SetMouseMoveCallbackId(int64_t callback_id);
    void SetKeyPressCallbackId(int64_t callback_id);

    void OnLeftButtonDown() override;
    void OnLeftButtonUp() override;
    void OnMouseMove() override;
    void OnKeyPress() override;

protected:
    vtkInteractorStyleCustom();
    ~vtkInteractorStyleCustom() override = default;

private:
    int64_t left_press_callback_id = 0;
    int64_t left_release_callback_id = 0;
    int64_t mouse_move_callback_id = 0;
    int64_t key_press_callback_id = 0;
};

// C-style wrapper functions (extern "C" for direct Rust FFI)
extern "C" {
    vtkInteractorStyleCustom* interactor_style_custom_new();
    void interactor_style_custom_delete(vtkInteractorStyleCustom* style);
    void interactor_style_custom_set_left_button_press_callback_id(
        vtkInteractorStyleCustom* style,
        int64_t callback_id
    );
    void interactor_style_custom_set_left_button_release_callback_id(
        vtkInteractorStyleCustom* style,
        int64_t callback_id
    );
    void interactor_style_custom_set_mouse_move_callback_id(
        vtkInteractorStyleCustom* style,
        int64_t callback_id
    );
    void interactor_style_custom_set_key_press_callback_id(
        vtkInteractorStyleCustom* style,
        int64_t callback_id
    );
}
