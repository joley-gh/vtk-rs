#include "vtk_point_data.h"
#include <vtkPolyData.h>
#include <vtkPointData.h>
#include <vtkDataArray.h>
#include <iostream>
#include <cstring>

extern "C" {

void* poly_data_get_point_data(void* poly_data) {
    if (!poly_data) return nullptr;
    return static_cast<vtkPolyData*>(poly_data)->GetPointData();
}

void point_data_add_array(void* point_data, void* array) {
    if (!point_data || !array) return;
    static_cast<vtkPointData*>(point_data)->AddArray(static_cast<vtkDataArray*>(array));
}

void point_data_remove_array(void* point_data, const char* name) {
    if (!point_data || !name) return;
    static_cast<vtkPointData*>(point_data)->RemoveArray(name);
}

void* point_data_get_array(void* point_data, const char* name) {
    if (!point_data || !name) return nullptr;
    return static_cast<vtkPointData*>(point_data)->GetArray(name);
}

int64_t point_data_get_number_of_arrays(void* point_data) {
    if (!point_data) return 0;
    return static_cast<int64_t>(static_cast<vtkPointData*>(point_data)->GetNumberOfArrays());
}

const char* point_data_get_array_name(void* point_data, int64_t index) {
    if (!point_data) return nullptr;
    return static_cast<vtkPointData*>(point_data)->GetArrayName(static_cast<int>(index));
}

void point_data_set_scalars(void* point_data, void* array) {
    if (!point_data || !array) return;
    static_cast<vtkPointData*>(point_data)->SetScalars(static_cast<vtkDataArray*>(array));
}

void* point_data_get_scalars(void* point_data) {
    if (!point_data) return nullptr;
    return static_cast<vtkPointData*>(point_data)->GetScalars();
}

void point_data_set_vectors(void* point_data, void* array) {
    if (!point_data || !array) return;
    static_cast<vtkPointData*>(point_data)->SetVectors(static_cast<vtkDataArray*>(array));
}

void* point_data_get_vectors(void* point_data) {
    if (!point_data) return nullptr;
    return static_cast<vtkPointData*>(point_data)->GetVectors();
}

void point_data_set_active_scalars(void* point_data, const char* name) {
    if (!point_data || !name) return;
    static_cast<vtkPointData*>(point_data)->SetActiveScalars(name);
}

void point_data_set_active_vectors(void* point_data, const char* name) {
    if (!point_data || !name) return;
    static_cast<vtkPointData*>(point_data)->SetActiveVectors(name);
}

} // extern "C"
