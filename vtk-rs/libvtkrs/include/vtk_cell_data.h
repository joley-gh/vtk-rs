#ifndef VTK_CELL_DATA_H
#define VTK_CELL_DATA_H

#include <stdint.h>

extern "C" {
    // Get CellData from PolyData
    void* poly_data_get_cell_data(void* poly_data);

    // Array management
    void cell_data_add_array(void* cell_data, void* array);
    void cell_data_remove_array(void* cell_data, const char* name);
    void* cell_data_get_array(void* cell_data, const char* name);
    int64_t cell_data_get_number_of_arrays(void* cell_data);
    const char* cell_data_get_array_name(void* cell_data, int64_t index);

    // Scalars and vectors
    void cell_data_set_scalars(void* cell_data, void* array);
    void* cell_data_get_scalars(void* cell_data);
    void cell_data_set_vectors(void* cell_data, void* array);
    void* cell_data_get_vectors(void* cell_data);
    
    // Active attributes
    void cell_data_set_active_scalars(void* cell_data, const char* name);
    void cell_data_set_active_vectors(void* cell_data, const char* name);
}

#endif // VTK_CELL_DATA_H
