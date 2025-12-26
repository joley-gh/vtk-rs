#ifndef VTK_POINT_DATA_H
#define VTK_POINT_DATA_H

#include <stdint.h>

extern "C" {
    // Get PointData from PolyData
    void* poly_data_get_point_data(void* poly_data);

    // Array management
    void point_data_add_array(void* point_data, void* array);
    void point_data_remove_array(void* point_data, const char* name);
    void* point_data_get_array(void* point_data, const char* name);
    int64_t point_data_get_number_of_arrays(void* point_data);
    const char* point_data_get_array_name(void* point_data, int64_t index);

    // Scalars and vectors
    void point_data_set_scalars(void* point_data, void* array);
    void* point_data_get_scalars(void* point_data);
    void point_data_set_vectors(void* point_data, void* array);
    void* point_data_get_vectors(void* point_data);
    
    // Active attributes
    void point_data_set_active_scalars(void* point_data, const char* name);
    void point_data_set_active_vectors(void* point_data, const char* name);
}

#endif // VTK_POINT_DATA_H
