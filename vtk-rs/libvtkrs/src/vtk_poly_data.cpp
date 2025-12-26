#include "vtk_poly_data.h"
#include <vtkPolyData.h>
#include <vtkPoints.h>
#include <vtkCellArray.h>
#include <vtkTrivialProducer.h>
#include <iostream>

extern "C" {

void* poly_data_new() {
    std::cerr << "DEBUG: poly_data_new() called" << std::endl;
    vtkPolyData* pd = vtkPolyData::New();
    std::cerr << "DEBUG: vtkPolyData::New() returned: " << pd << std::endl;
    return pd;
}

void poly_data_delete(void* poly_data) {
    if (poly_data) {
        static_cast<vtkPolyData*>(poly_data)->Delete();
    }
}

void poly_data_set_points(void* poly_data, void* points) {
    if (!poly_data || !points) return;
    
    vtkPolyData* pd = static_cast<vtkPolyData*>(poly_data);
    vtkPoints* pts = static_cast<vtkPoints*>(points);
    
    pd->SetPoints(pts);
}

void* poly_data_get_points(void* poly_data) {
    if (!poly_data) return nullptr;
    
    vtkPolyData* pd = static_cast<vtkPolyData*>(poly_data);
    return pd->GetPoints();
}

int64_t poly_data_get_number_of_points(void* poly_data) {
    if (!poly_data) return 0;
    
    vtkPolyData* pd = static_cast<vtkPolyData*>(poly_data);
    return static_cast<int64_t>(pd->GetNumberOfPoints());
}

void poly_data_set_lines(void* poly_data, void* lines) {
    if (!poly_data || !lines) return;
    
    vtkPolyData* pd = static_cast<vtkPolyData*>(poly_data);
    vtkCellArray* ca = static_cast<vtkCellArray*>(lines);
    
    pd->SetLines(ca);
}

void* poly_data_get_lines(void* poly_data) {
    if (!poly_data) return nullptr;
    
    vtkPolyData* pd = static_cast<vtkPolyData*>(poly_data);
    return pd->GetLines();
}

int64_t poly_data_get_number_of_lines(void* poly_data) {
    if (!poly_data) return 0;
    
    vtkPolyData* pd = static_cast<vtkPolyData*>(poly_data);
    return static_cast<int64_t>(pd->GetNumberOfLines());
}

int64_t poly_data_get_number_of_cells(void* poly_data) {
    if (!poly_data) return 0;
    
    vtkPolyData* pd = static_cast<vtkPolyData*>(poly_data);
    return static_cast<int64_t>(pd->GetNumberOfCells());
}

void poly_data_get_bounds(void* poly_data, double bounds[6]) {
    if (!poly_data || !bounds) return;
    
    vtkPolyData* pd = static_cast<vtkPolyData*>(poly_data);
    double* b = pd->GetBounds();
    
    for (int i = 0; i < 6; i++) {
        bounds[i] = b[i];
    }
}

void poly_data_allocate(void* poly_data, int64_t num_verts, int64_t connectivity_size) {
    if (!poly_data) return;
    
    vtkPolyData* pd = static_cast<vtkPolyData*>(poly_data);
    pd->Allocate(static_cast<vtkIdType>(num_verts), 
                 static_cast<vtkIdType>(connectivity_size));
}

void poly_data_modified(void* poly_data) {
    if (!poly_data) return;
    
    vtkPolyData* pd = static_cast<vtkPolyData*>(poly_data);
    pd->Modified();
}

void poly_data_compute_bounds(void* poly_data) {
    if (!poly_data) return;
    
    vtkPolyData* pd = static_cast<vtkPolyData*>(poly_data);
    pd->ComputeBounds();
}

void* poly_data_get_producer_port(void* poly_data) {
    if (!poly_data) return nullptr;
    
    vtkPolyData* pd = static_cast<vtkPolyData*>(poly_data);
    vtkTrivialProducer* producer = vtkTrivialProducer::New();
    producer->SetOutput(pd);
    return producer->GetOutputPort();
}

} // extern "C"

