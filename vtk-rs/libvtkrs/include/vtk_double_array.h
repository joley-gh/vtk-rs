#ifndef VTK_DOUBLE_ARRAY_H
#define VTK_DOUBLE_ARRAY_H

#include <stdint.h>

extern "C" {
    // Create and destroy
    void* double_array_new();
    void double_array_delete(void* array);

    // Configuration
    void double_array_set_number_of_components(void* array, int64_t num_components);
    int64_t double_array_get_number_of_components(void* array);
    void double_array_set_number_of_tuples(void* array, int64_t num_tuples);
    int64_t double_array_get_number_of_tuples(void* array);
    int64_t double_array_get_number_of_values(void* array);

    // Name management
    void double_array_set_name(void* array, const char* name);
    const char* double_array_get_name(void* array);

    // Data insertion and modification
    int64_t double_array_insert_next_value(void* array, double value);
    int64_t double_array_insert_next_tuple1(void* array, double value);
    int64_t double_array_insert_next_tuple2(void* array, double v0, double v1);
    int64_t double_array_insert_next_tuple3(void* array, double v0, double v1, double v2);
    void double_array_set_value(void* array, int64_t id, double value);
    void double_array_set_tuple1(void* array, int64_t id, double value);
    void double_array_set_tuple2(void* array, int64_t id, double v0, double v1);
    void double_array_set_tuple3(void* array, int64_t id, double v0, double v1, double v2);

    // Data retrieval
    double double_array_get_value(void* array, int64_t id);
    void double_array_get_tuple(void* array, int64_t id, double* tuple);

    // Memory management
    void double_array_initialize(void* array);
    void double_array_squeeze(void* array);
}

#endif // VTK_DOUBLE_ARRAY_H
