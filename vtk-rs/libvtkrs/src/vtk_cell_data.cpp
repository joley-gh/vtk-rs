#include "vtk_cell_data.h"
#include <vtkPolyData.h>
#include <vtkCellData.h>
#include <vtkDataArray.h>
#include <iostream>
#include <cstring>

extern "C" {

void* poly_data_get_cell_data(void* poly_data) {
    if (!poly_data) return nullptr;
    return static_cast<vtkPolyData*>(poly_data)->GetCellData();
}

void cell_data_add_array(void* cell_data, void* array) {
    if (!cell_data || !array) return;
    static_cast<vtkCellData*>(cell_data)->AddArray(static_cast<vtkDataArray*>(array));
}

void cell_data_remove_array(void* cell_data, const char* name) {
    if (!cell_data || !name) return;
    static_cast<vtkCellData*>(cell_data)->RemoveArray(name);
}

void* cell_data_get_array(void* cell_data, const char* name) {
    if (!cell_data || !name) return nullptr;
    return static_cast<vtkCellData*>(cell_data)->GetArray(name);
}

int64_t cell_data_get_number_of_arrays(void* cell_data) {
    if (!cell_data) return 0;
    return static_cast<int64_t>(static_cast<vtkCellData*>(cell_data)->GetNumberOfArrays());
}

const char* cell_data_get_array_name(void* cell_data, int64_t index) {
    if (!cell_data) return nullptr;
    return static_cast<vtkCellData*>(cell_data)->GetArrayName(static_cast<int>(index));
}

void cell_data_set_scalars(void* cell_data, void* array) {
    if (!cell_data || !array) return;
    static_cast<vtkCellData*>(cell_data)->SetScalars(static_cast<vtkDataArray*>(array));
}

void* cell_data_get_scalars(void* cell_data) {
    if (!cell_data) return nullptr;
    return static_cast<vtkCellData*>(cell_data)->GetScalars();
}

void cell_data_set_vectors(void* cell_data, void* array) {
    if (!cell_data || !array) return;
    static_cast<vtkCellData*>(cell_data)->SetVectors(static_cast<vtkDataArray*>(array));
}

void* cell_data_get_vectors(void* cell_data) {
    if (!cell_data) return nullptr;
    return static_cast<vtkCellData*>(cell_data)->GetVectors();
}

void cell_data_set_active_scalars(void* cell_data, const char* name) {
    if (!cell_data || !name) return;
    static_cast<vtkCellData*>(cell_data)->SetActiveScalars(name);
}

void cell_data_set_active_vectors(void* cell_data, const char* name) {
    if (!cell_data || !name) return;
    static_cast<vtkCellData*>(cell_data)->SetActiveVectors(name);
}

} // extern "C"
