#ifndef VTK_POLY_DATA_H
#define VTK_POLY_DATA_H

#include <stdint.h>

extern "C" {
    // Create and destroy
    void* poly_data_new();
    void poly_data_delete(void* poly_data);

    // Points management
    void poly_data_set_points(void* poly_data, void* points);
    void* poly_data_get_points(void* poly_data);
    int64_t poly_data_get_number_of_points(void* poly_data);

    // Lines (beam elements) management
    void poly_data_set_lines(void* poly_data, void* lines);
    void* poly_data_get_lines(void* poly_data);
    int64_t poly_data_get_number_of_lines(void* poly_data);

    // Cells (general) management
    int64_t poly_data_get_number_of_cells(void* poly_data);

    // Geometry queries
    void poly_data_get_bounds(void* poly_data, double bounds[6]);

    // Memory management
    void poly_data_allocate(void* poly_data, int64_t num_verts, int64_t connectivity_size);
    
    // Update notifications
    void poly_data_modified(void* poly_data);
    void poly_data_compute_bounds(void* poly_data);

    // Data attributes
    void* poly_data_get_point_data(void* poly_data);
    void* poly_data_get_cell_data(void* poly_data);

    // Algorithm output (for connecting to filters)
    void* poly_data_get_producer_port(void* poly_data);
}

#endif // VTK_POLY_DATA_H

