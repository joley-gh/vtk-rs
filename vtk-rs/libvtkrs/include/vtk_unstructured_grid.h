#pragma once

#ifndef VTK_UNSTRUCTURED_GRID_H
#define VTK_UNSTRUCTURED_GRID_H

#include <vtkUnstructuredGrid.h>
#include <vtkPoints.h>

// Create/Delete
vtkUnstructuredGrid* vtk_unstructured_grid_new();
void vtk_unstructured_grid_delete(vtkUnstructuredGrid& grid);

// Points
void unstructured_grid_set_points(vtkUnstructuredGrid& grid, vtkPoints* points);
vtkPoints* unstructured_grid_get_points(vtkUnstructuredGrid& grid);

// Cell allocation
void unstructured_grid_allocate(vtkUnstructuredGrid& grid, int num_cells);

// Cell insertion
void unstructured_grid_insert_next_cell(
    vtkUnstructuredGrid& grid,
    int cell_type,
    int num_points,
    const int* point_ids
);

// Metadata
int unstructured_grid_get_number_of_points(const vtkUnstructuredGrid& grid);
int unstructured_grid_get_number_of_cells(const vtkUnstructuredGrid& grid);
void unstructured_grid_get_bounds(const vtkUnstructuredGrid& grid, double* bounds);

#endif // VTK_UNSTRUCTURED_GRID_H
