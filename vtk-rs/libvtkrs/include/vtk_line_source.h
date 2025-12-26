#ifndef VTK_LINE_SOURCE_H
#define VTK_LINE_SOURCE_H

#include <vtkLineSource.h>

extern "C" {
    vtkLineSource* line_source_new();
    void line_source_delete(vtkLineSource* line_source);
    void line_source_set_point1(vtkLineSource* line_source, double x, double y, double z);
    void line_source_get_point1(vtkLineSource* line_source, double* x, double* y, double* z);
    void line_source_set_point2(vtkLineSource* line_source, double x, double y, double z);
    void line_source_get_point2(vtkLineSource* line_source, double* x, double* y, double* z);
    void line_source_set_resolution(vtkLineSource* line_source, int resolution);
    int line_source_get_resolution(vtkLineSource* line_source);
    vtkAlgorithmOutput* line_source_get_output_port(vtkLineSource* line_source);
}

#endif // VTK_LINE_SOURCE_H
