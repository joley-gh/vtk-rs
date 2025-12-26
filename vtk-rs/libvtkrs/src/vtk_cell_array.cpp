#include "vtk_cell_array.h"
#include <vtkCellArray.h>
#include <vtkIdList.h>
#include <iostream>

extern "C" {

void* cell_array_new() {
    std::cerr << "DEBUG: cell_array_new() called" << std::endl;
    vtkCellArray* cells = vtkCellArray::New();
    std::cerr << "DEBUG: vtkCellArray::New() returned: " << cells << std::endl;
    return cells;
}

void cell_array_delete(void* cells) {
    if (cells) {
        static_cast<vtkCellArray*>(cells)->Delete();
    }
}

int64_t cell_array_insert_next_cell(void* cells, int64_t npts, const int64_t* pts) {
    if (!cells || !pts) return -1;
    
    vtkCellArray* ca = static_cast<vtkCellArray*>(cells);
    
    // Convert int64_t array to vtkIdType array
    vtkIdType* vtkPts = new vtkIdType[npts];
    for (int64_t i = 0; i < npts; i++) {
        vtkPts[i] = static_cast<vtkIdType>(pts[i]);
    }
    
    vtkIdType cellId = ca->InsertNextCell(npts, vtkPts);
    delete[] vtkPts;
    
    return static_cast<int64_t>(cellId);
}

int64_t cell_array_get_number_of_cells(void* cells) {
    if (!cells) return 0;
    return static_cast<int64_t>(static_cast<vtkCellArray*>(cells)->GetNumberOfCells());
}

int64_t cell_array_get_number_of_connectivity_ids(void* cells) {
    if (!cells) return 0;
    return static_cast<int64_t>(static_cast<vtkCellArray*>(cells)->GetNumberOfConnectivityIds());
}

bool cell_array_get_cell(void* cells, int64_t loc, int64_t* npts, int64_t** pts) {
    if (!cells || !npts || !pts) return false;
    
    vtkCellArray* ca = static_cast<vtkCellArray*>(cells);
    vtkIdType vtkNpts;
    const vtkIdType* vtkPts;
    
    // Use GetCellAtId for direct access
    ca->GetCellAtId(static_cast<vtkIdType>(loc), vtkNpts, vtkPts);
    
    *npts = static_cast<int64_t>(vtkNpts);
    
    // Allocate array and copy point IDs
    if (vtkNpts > 0) {
        *pts = new int64_t[vtkNpts];
        for (vtkIdType i = 0; i < vtkNpts; i++) {
            (*pts)[i] = static_cast<int64_t>(vtkPts[i]);
        }
        return true;
    }
    
    *pts = nullptr;
    return false;
}

void cell_array_reset(void* cells) {
    if (!cells) return;
    static_cast<vtkCellArray*>(cells)->Reset();
}

void cell_array_initialize(void* cells) {
    if (!cells) return;
    static_cast<vtkCellArray*>(cells)->Initialize();
}

} // extern "C"
