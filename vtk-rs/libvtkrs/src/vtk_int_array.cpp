#include "vtk_int_array.h"
#include <vtkIntArray.h>
#include <iostream>
#include <cstring>

extern "C" {

void* int_array_new() {
    std::cerr << "DEBUG: int_array_new() called" << std::endl;
    vtkIntArray* array = vtkIntArray::New();
    std::cerr << "DEBUG: vtkIntArray::New() returned: " << array << std::endl;
    return array;
}

void int_array_delete(void* array) {
    if (array) {
        static_cast<vtkIntArray*>(array)->Delete();
    }
}

void int_array_set_number_of_components(void* array, int64_t num_components) {
    if (!array) return;
    static_cast<vtkIntArray*>(array)->SetNumberOfComponents(static_cast<int>(num_components));
}

int64_t int_array_get_number_of_components(void* array) {
    if (!array) return 0;
    return static_cast<int64_t>(static_cast<vtkIntArray*>(array)->GetNumberOfComponents());
}

void int_array_set_number_of_tuples(void* array, int64_t num_tuples) {
    if (!array) return;
    static_cast<vtkIntArray*>(array)->SetNumberOfTuples(static_cast<vtkIdType>(num_tuples));
}

int64_t int_array_get_number_of_tuples(void* array) {
    if (!array) return 0;
    return static_cast<int64_t>(static_cast<vtkIntArray*>(array)->GetNumberOfTuples());
}

int64_t int_array_get_number_of_values(void* array) {
    if (!array) return 0;
    return static_cast<int64_t>(static_cast<vtkIntArray*>(array)->GetNumberOfValues());
}

void int_array_set_name(void* array, const char* name) {
    if (!array || !name) return;
    static_cast<vtkIntArray*>(array)->SetName(name);
}

const char* int_array_get_name(void* array) {
    if (!array) return nullptr;
    return static_cast<vtkIntArray*>(array)->GetName();
}

int64_t int_array_insert_next_value(void* array, int32_t value) {
    if (!array) return -1;
    return static_cast<int64_t>(static_cast<vtkIntArray*>(array)->InsertNextValue(value));
}

int64_t int_array_insert_next_tuple1(void* array, int32_t value) {
    if (!array) return -1;
    vtkIdType tuple_idx = static_cast<vtkIntArray*>(array)->GetNumberOfTuples();
    static_cast<vtkIntArray*>(array)->InsertNextTuple1(value);
    return static_cast<int64_t>(tuple_idx);
}

void int_array_set_value(void* array, int64_t id, int32_t value) {
    if (!array) return;
    static_cast<vtkIntArray*>(array)->SetValue(static_cast<vtkIdType>(id), value);
}

void int_array_set_tuple1(void* array, int64_t id, int32_t value) {
    if (!array) return;
    static_cast<vtkIntArray*>(array)->SetTuple1(static_cast<vtkIdType>(id), value);
}

int32_t int_array_get_value(void* array, int64_t id) {
    if (!array) return 0;
    return static_cast<vtkIntArray*>(array)->GetValue(static_cast<vtkIdType>(id));
}

void int_array_get_tuple(void* array, int64_t id, int32_t* tuple) {
    if (!array || !tuple) return;
    vtkIntArray* arr = static_cast<vtkIntArray*>(array);
    int num_components = arr->GetNumberOfComponents();
    for (int i = 0; i < num_components; i++) {
        tuple[i] = arr->GetComponent(static_cast<vtkIdType>(id), i);
    }
}

void int_array_initialize(void* array) {
    if (!array) return;
    static_cast<vtkIntArray*>(array)->Initialize();
}

void int_array_squeeze(void* array) {
    if (!array) return;
    static_cast<vtkIntArray*>(array)->Squeeze();
}

} // extern "C"
