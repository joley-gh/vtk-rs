#include "vtk_command.h"
#include "vtk_command.rs.h"
#include <vtkObjectFactory.h>
#include <iostream>

vtkStandardNewMacro(RustCommand);

void RustCommand::Execute(vtkObject* caller, unsigned long eventId, void* callData) {
    if (callback_fn != 0) {
        // Call the Rust callback function
        // The callback signature in Rust: extern "C" fn(caller: usize, event_id: u64, user_data: usize)
        typedef void (*RustCallbackFn)(uintptr_t, unsigned long, uintptr_t);
        RustCallbackFn callback = reinterpret_cast<RustCallbackFn>(callback_fn);
        callback(reinterpret_cast<uintptr_t>(caller), eventId, user_data);
    }
}

void RustCommand::SetCallback(uintptr_t callback_fn, uintptr_t user_data) {
    this->callback_fn = callback_fn;
    this->user_data = user_data;
}

RustCommand* vtk_command_new() {
    std::cout << "DEBUG C++: vtk_command_new() called" << std::endl;
    RustCommand* cmd = RustCommand::New();
    std::cout << "DEBUG C++: RustCommand::New() returned: " << cmd << std::endl;
    return cmd;
}

void vtk_command_delete(RustCommand* command) {
    if (command) {
        command->Delete();
    }
}

void vtk_command_set_callback(RustCommand& command, uintptr_t callback_fn, uintptr_t user_data) {
    command.SetCallback(callback_fn, user_data);
}

unsigned long vtk_object_add_observer(vtkObject& obj, unsigned long event, RustCommand& command) {
    return obj.AddObserver(event, &command);
}

void vtk_object_remove_observer(vtkObject& obj, unsigned long tag) {
    obj.RemoveObserver(tag);
}
