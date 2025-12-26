#ifndef VTK_CELL_ARRAY_H
#define VTK_CELL_ARRAY_H

#include <stdint.h>

extern "C" {
    // Create and destroy
    void* cell_array_new();
    void cell_array_delete(void* cells);

    // Insert cells
    int64_t cell_array_insert_next_cell(void* cells, int64_t npts, const int64_t* pts);
    
    // Query
    int64_t cell_array_get_number_of_cells(void* cells);
    int64_t cell_array_get_number_of_connectivity_ids(void* cells);
    
    // Access cells
    bool cell_array_get_cell(void* cells, int64_t loc, int64_t* npts, int64_t** pts);
    
    // Modify
    void cell_array_reset(void* cells);
    void cell_array_initialize(void* cells);
}

#endif // VTK_CELL_ARRAY_H
