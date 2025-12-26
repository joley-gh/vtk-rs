#include "vtk_points.h"
#include <vtkPoints.h>
#include <iostream>

extern "C" {
    vtkPoints* points_new() {
        std::cerr << "DEBUG: points_new() called" << std::endl;
        vtkPoints* pts = vtkPoints::New();
        std::cerr << "DEBUG: vtkPoints::New() returned: " << pts << std::endl;
        return pts;
    }

    void points_delete(vtkPoints* points) {
        if (points) {
            points->Delete();
        }
    }

    vtkIdType points_insert_next_point(vtkPoints* points, double x, double y, double z) {
        if (points) {
            return points->InsertNextPoint(x, y, z);
        }
        return -1;
    }

    void points_set_point(vtkPoints* points, vtkIdType id, double x, double y, double z) {
        if (points) {
            points->SetPoint(id, x, y, z);
        }
    }

    void points_get_point(vtkPoints* points, vtkIdType id, double* x, double* y, double* z) {
        if (points && x && y && z) {
            double point[3];
            points->GetPoint(id, point);
            *x = point[0];
            *y = point[1];
            *z = point[2];
        }
    }

    vtkIdType points_get_number_of_points(vtkPoints* points) {
        if (points) {
            return points->GetNumberOfPoints();
        }
        return 0;
    }

    void points_reset(vtkPoints* points) {
        if (points) {
            points->Reset();
        }
    }

    void points_set_number_of_points(vtkPoints* points, vtkIdType number) {
        if (points) {
            points->SetNumberOfPoints(number);
        }
    }

    void points_resize(vtkPoints* points, vtkIdType number) {
        if (points) {
            points->Resize(number);
        }
    }
}
