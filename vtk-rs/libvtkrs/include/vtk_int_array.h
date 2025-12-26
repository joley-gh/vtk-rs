#ifndef VTK_INT_ARRAY_H
#define VTK_INT_ARRAY_H

#include <stdint.h>

extern "C" {
    // Create and destroy
    void* int_array_new();
    void int_array_delete(void* array);

    // Configuration
    void int_array_set_number_of_components(void* array, int64_t num_components);
    int64_t int_array_get_number_of_components(void* array);
    void int_array_set_number_of_tuples(void* array, int64_t num_tuples);
    int64_t int_array_get_number_of_tuples(void* array);
    int64_t int_array_get_number_of_values(void* array);

    // Name management
    void int_array_set_name(void* array, const char* name);
    const char* int_array_get_name(void* array);

    // Data insertion and modification
    int64_t int_array_insert_next_value(void* array, int32_t value);
    int64_t int_array_insert_next_tuple1(void* array, int32_t value);
    void int_array_set_value(void* array, int64_t id, int32_t value);
    void int_array_set_tuple1(void* array, int64_t id, int32_t value);

    // Data retrieval
    int32_t int_array_get_value(void* array, int64_t id);
    void int_array_get_tuple(void* array, int64_t id, int32_t* tuple);

    // Memory management
    void int_array_initialize(void* array);
    void int_array_squeeze(void* array);
}

#endif // VTK_INT_ARRAY_H
