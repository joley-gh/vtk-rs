#include "vtk_double_array.h"
#include <vtkDoubleArray.h>
#include <iostream>
#include <cstring>

extern "C" {

void* double_array_new() {
    std::cerr << "DEBUG: double_array_new() called" << std::endl;
    vtkDoubleArray* array = vtkDoubleArray::New();
    std::cerr << "DEBUG: vtkDoubleArray::New() returned: " << array << std::endl;
    return array;
}

void double_array_delete(void* array) {
    if (array) {
        static_cast<vtkDoubleArray*>(array)->Delete();
    }
}

void double_array_set_number_of_components(void* array, int64_t num_components) {
    if (!array) return;
    static_cast<vtkDoubleArray*>(array)->SetNumberOfComponents(static_cast<int>(num_components));
}

int64_t double_array_get_number_of_components(void* array) {
    if (!array) return 0;
    return static_cast<int64_t>(static_cast<vtkDoubleArray*>(array)->GetNumberOfComponents());
}

void double_array_set_number_of_tuples(void* array, int64_t num_tuples) {
    if (!array) return;
    static_cast<vtkDoubleArray*>(array)->SetNumberOfTuples(static_cast<vtkIdType>(num_tuples));
}

int64_t double_array_get_number_of_tuples(void* array) {
    if (!array) return 0;
    return static_cast<int64_t>(static_cast<vtkDoubleArray*>(array)->GetNumberOfTuples());
}

int64_t double_array_get_number_of_values(void* array) {
    if (!array) return 0;
    return static_cast<int64_t>(static_cast<vtkDoubleArray*>(array)->GetNumberOfValues());
}

void double_array_set_name(void* array, const char* name) {
    if (!array || !name) return;
    static_cast<vtkDoubleArray*>(array)->SetName(name);
}

const char* double_array_get_name(void* array) {
    if (!array) return nullptr;
    return static_cast<vtkDoubleArray*>(array)->GetName();
}

int64_t double_array_insert_next_value(void* array, double value) {
    if (!array) return -1;
    return static_cast<int64_t>(static_cast<vtkDoubleArray*>(array)->InsertNextValue(value));
}

int64_t double_array_insert_next_tuple1(void* array, double value) {
    if (!array) return -1;
    vtkIdType tuple_idx = static_cast<vtkDoubleArray*>(array)->GetNumberOfTuples();
    static_cast<vtkDoubleArray*>(array)->InsertNextTuple1(value);
    return static_cast<int64_t>(tuple_idx);
}

int64_t double_array_insert_next_tuple2(void* array, double v0, double v1) {
    if (!array) return -1;
    vtkIdType tuple_idx = static_cast<vtkDoubleArray*>(array)->GetNumberOfTuples();
    static_cast<vtkDoubleArray*>(array)->InsertNextTuple2(v0, v1);
    return static_cast<int64_t>(tuple_idx);
}

int64_t double_array_insert_next_tuple3(void* array, double v0, double v1, double v2) {
    if (!array) return -1;
    vtkIdType tuple_idx = static_cast<vtkDoubleArray*>(array)->GetNumberOfTuples();
    static_cast<vtkDoubleArray*>(array)->InsertNextTuple3(v0, v1, v2);
    return static_cast<int64_t>(tuple_idx);
}

void double_array_set_value(void* array, int64_t id, double value) {
    if (!array) return;
    static_cast<vtkDoubleArray*>(array)->SetValue(static_cast<vtkIdType>(id), value);
}

void double_array_set_tuple1(void* array, int64_t id, double value) {
    if (!array) return;
    static_cast<vtkDoubleArray*>(array)->SetTuple1(static_cast<vtkIdType>(id), value);
}

void double_array_set_tuple2(void* array, int64_t id, double v0, double v1) {
    if (!array) return;
    static_cast<vtkDoubleArray*>(array)->SetTuple2(static_cast<vtkIdType>(id), v0, v1);
}

void double_array_set_tuple3(void* array, int64_t id, double v0, double v1, double v2) {
    if (!array) return;
    static_cast<vtkDoubleArray*>(array)->SetTuple3(static_cast<vtkIdType>(id), v0, v1, v2);
}

double double_array_get_value(void* array, int64_t id) {
    if (!array) return 0.0;
    return static_cast<vtkDoubleArray*>(array)->GetValue(static_cast<vtkIdType>(id));
}

void double_array_get_tuple(void* array, int64_t id, double* tuple) {
    if (!array || !tuple) return;
    static_cast<vtkDoubleArray*>(array)->GetTuple(static_cast<vtkIdType>(id), tuple);
}

void double_array_initialize(void* array) {
    if (!array) return;
    static_cast<vtkDoubleArray*>(array)->Initialize();
}

void double_array_squeeze(void* array) {
    if (!array) return;
    static_cast<vtkDoubleArray*>(array)->Squeeze();
}

} // extern "C"
