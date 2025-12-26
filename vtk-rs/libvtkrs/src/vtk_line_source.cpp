#include "vtk_line_source.h"
#include <vtkLineSource.h>
#include <vtkAlgorithmOutput.h>
#include <iostream>

extern "C" {
    vtkLineSource* line_source_new() {
        std::cerr << "DEBUG: line_source_new() called" << std::endl;
        vtkLineSource* line = vtkLineSource::New();
        std::cerr << "DEBUG: vtkLineSource::New() returned: " << line << std::endl;
        return line;
    }

    void line_source_delete(vtkLineSource* line_source) {
        if (line_source) {
            line_source->Delete();
        }
    }

    void line_source_set_point1(vtkLineSource* line_source, double x, double y, double z) {
        if (line_source) {
            line_source->SetPoint1(x, y, z);
        }
    }

    void line_source_get_point1(vtkLineSource* line_source, double* x, double* y, double* z) {
        if (line_source && x && y && z) {
            double point[3];
            line_source->GetPoint1(point);
            *x = point[0];
            *y = point[1];
            *z = point[2];
        }
    }

    void line_source_set_point2(vtkLineSource* line_source, double x, double y, double z) {
        if (line_source) {
            line_source->SetPoint2(x, y, z);
        }
    }

    void line_source_get_point2(vtkLineSource* line_source, double* x, double* y, double* z) {
        if (line_source && x && y && z) {
            double point[3];
            line_source->GetPoint2(point);
            *x = point[0];
            *y = point[1];
            *z = point[2];
        }
    }

    void line_source_set_resolution(vtkLineSource* line_source, int resolution) {
        if (line_source) {
            line_source->SetResolution(resolution);
        }
    }

    int line_source_get_resolution(vtkLineSource* line_source) {
        if (line_source) {
            return line_source->GetResolution();
        }
        return 0;
    }

    vtkAlgorithmOutput* line_source_get_output_port(vtkLineSource* line_source) {
        if (line_source) {
            return line_source->GetOutputPort();
        }
        return nullptr;
    }
}
