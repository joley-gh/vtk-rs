#ifndef VTK_POINTS_H
#define VTK_POINTS_H

#include <vtkPoints.h>

extern "C" {
    vtkPoints* points_new();
    void points_delete(vtkPoints* points);
    vtkIdType points_insert_next_point(vtkPoints* points, double x, double y, double z);
    void points_set_point(vtkPoints* points, vtkIdType id, double x, double y, double z);
    void points_get_point(vtkPoints* points, vtkIdType id, double* x, double* y, double* z);
    vtkIdType points_get_number_of_points(vtkPoints* points);
    void points_reset(vtkPoints* points);
    void points_set_number_of_points(vtkPoints* points, vtkIdType number);
    void points_resize(vtkPoints* points, vtkIdType number);
}

#endif // VTK_POINTS_H
