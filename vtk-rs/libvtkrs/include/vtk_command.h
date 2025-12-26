#pragma once

#include <vtkCommand.h>
#include <vtkObject.h>
#include <cxx.h>
#include <cstdint>

// Forward declare the Rust callback type
struct RustCallbackData;

// Custom command class that calls Rust callbacks
class RustCommand : public vtkCommand {
public:
    static RustCommand* New();
    
    vtkTypeMacro(RustCommand, vtkCommand);
    
    void Execute(vtkObject* caller, unsigned long eventId, void* callData) override;
    
    // Store the Rust callback function pointer and user data
    void SetCallback(uintptr_t callback_fn, uintptr_t user_data);

protected:
    RustCommand() : callback_fn(0), user_data(0) {}
    ~RustCommand() override = default;
    
private:
    uintptr_t callback_fn;
    uintptr_t user_data;
};

// C API for creating and managing commands
RustCommand* vtk_command_new();
void vtk_command_delete(RustCommand* command);
void vtk_command_set_callback(RustCommand& command, uintptr_t callback_fn, uintptr_t user_data);

// Add observer to vtkObject
unsigned long vtk_object_add_observer(vtkObject& obj, unsigned long event, RustCommand& command);
void vtk_object_remove_observer(vtkObject& obj, unsigned long tag);

// Event IDs from vtkCommand
constexpr unsigned long VTK_EVENT_NO_EVENT = 0;
constexpr unsigned long VTK_EVENT_ANY_EVENT = 1;
constexpr unsigned long VTK_EVENT_DELETE_EVENT = 2;
constexpr unsigned long VTK_EVENT_START_EVENT = 3;
constexpr unsigned long VTK_EVENT_END_EVENT = 4;
constexpr unsigned long VTK_EVENT_RENDER_EVENT = 5;
constexpr unsigned long VTK_EVENT_PROGRESS_EVENT = 6;
constexpr unsigned long VTK_EVENT_PICK_EVENT = 7;
constexpr unsigned long VTK_EVENT_START_PICK_EVENT = 8;
constexpr unsigned long VTK_EVENT_END_PICK_EVENT = 9;
constexpr unsigned long VTK_EVENT_ABORT_CHECK_EVENT = 10;
constexpr unsigned long VTK_EVENT_EXIT_EVENT = 11;
constexpr unsigned long VTK_EVENT_LEFT_BUTTON_PRESS_EVENT = 12;
constexpr unsigned long VTK_EVENT_LEFT_BUTTON_RELEASE_EVENT = 13;
constexpr unsigned long VTK_EVENT_MIDDLE_BUTTON_PRESS_EVENT = 14;
constexpr unsigned long VTK_EVENT_MIDDLE_BUTTON_RELEASE_EVENT = 15;
constexpr unsigned long VTK_EVENT_RIGHT_BUTTON_PRESS_EVENT = 16;
constexpr unsigned long VTK_EVENT_RIGHT_BUTTON_RELEASE_EVENT = 17;
constexpr unsigned long VTK_EVENT_ENTER_EVENT = 18;
constexpr unsigned long VTK_EVENT_LEAVE_EVENT = 19;
constexpr unsigned long VTK_EVENT_KEY_PRESS_EVENT = 20;
constexpr unsigned long VTK_EVENT_KEY_RELEASE_EVENT = 21;
constexpr unsigned long VTK_EVENT_CHAR_EVENT = 22;
constexpr unsigned long VTK_EVENT_EXPOSE_EVENT = 23;
constexpr unsigned long VTK_EVENT_CONFIGURE_EVENT = 24;
constexpr unsigned long VTK_EVENT_TIMER_EVENT = 25;
constexpr unsigned long VTK_EVENT_MOUSE_MOVE_EVENT = 26;
constexpr unsigned long VTK_EVENT_MOUSE_WHEEL_FORWARD_EVENT = 27;
constexpr unsigned long VTK_EVENT_MOUSE_WHEEL_BACKWARD_EVENT = 28;
constexpr unsigned long VTK_EVENT_RESET_CAMERA_EVENT = 29;
constexpr unsigned long VTK_EVENT_RESET_CAMERA_CLIPPING_RANGE_EVENT = 30;
constexpr unsigned long VTK_EVENT_MODIFIED_EVENT = 33;
constexpr unsigned long VTK_EVENT_WINDOW_IS_CURRENT_EVENT = 34;
constexpr unsigned long VTK_EVENT_WINDOW_IS_DIRECT_EVENT = 35;
constexpr unsigned long VTK_EVENT_WINDOW_SUPPORTSOPENG_EVENT = 36;
constexpr unsigned long VTK_EVENT_WINDOW_FRAME_EVENT = 37;
constexpr unsigned long VTK_EVENT_START_ANIMATION_CUE_EVENT = 38;
constexpr unsigned long VTK_EVENT_ANIMATION_CUE_TICK_EVENT = 39;
constexpr unsigned long VTK_EVENT_END_ANIMATION_CUE_EVENT = 40;
constexpr unsigned long VTK_EVENT_VOLUME_MAPPER_RENDER_END_EVENT = 41;
constexpr unsigned long VTK_EVENT_VOLUME_MAPPER_RENDER_PROGRESS_EVENT = 42;
constexpr unsigned long VTK_EVENT_VOLUME_MAPPER_RENDER_START_EVENT = 43;
constexpr unsigned long VTK_EVENT_VOLUME_MAPPER_COMPUTE_GRADIENTS_END_EVENT = 44;
constexpr unsigned long VTK_EVENT_VOLUME_MAPPER_COMPUTE_GRADIENTS_PROGRESS_EVENT = 45;
constexpr unsigned long VTK_EVENT_VOLUME_MAPPER_COMPUTE_GRADIENTS_START_EVENT = 46;
constexpr unsigned long VTK_EVENT_START_INTERACTION_EVENT = 47;
constexpr unsigned long VTK_EVENT_INTERACTION_EVENT = 48;
constexpr unsigned long VTK_EVENT_END_INTERACTION_EVENT = 49;
