#include "cxx.h"
#include "vtk_regular_polygon_source.h"
#include "vtk_regular_polygon_source.rs.h"

vtkRegularPolygonSource* vtk_regular_polygon_source_new() {
     return vtkRegularPolygonSource::New();
}

void vtk_regular_polygon_source_delete(vtkRegularPolygonSource& polygon) {
    polygon.Delete();
}
void vtk_regular_polygon_source_set_number_of_sides(vtkRegularPolygonSource& polygon, int number_of_sides) {
    polygon.SetNumberOfSides(number_of_sides);
}
int vtk_regular_polygon_source_get_number_of_sides(const vtkRegularPolygonSource& polygon) {
    return const_cast<vtkRegularPolygonSource&>(polygon).GetNumberOfSides();
}

void vtk_regular_polygon_source_set_center(vtkRegularPolygonSource& polygon, double x, double y, double z) {
    polygon.SetCenter(x, y, z);
}
void vtk_regular_polygon_source_get_center(const vtkRegularPolygonSource& polygon, double& x, double& y, double& z) {
    double* center = const_cast<vtkRegularPolygonSource&>(polygon).GetCenter();
    x = center[0];
    y = center[1];
    z = center[2];
}

void vtk_regular_polygon_source_set_normal(vtkRegularPolygonSource& polygon, double x, double y, double z) {
    polygon.SetNormal(x, y, z);
}
void vtk_regular_polygon_source_get_normal(const vtkRegularPolygonSource& polygon, double& x, double& y, double& z) {
    double* normal = const_cast<vtkRegularPolygonSource&>(polygon).GetNormal();
    x = normal[0];
    y = normal[1];
    z = normal[2];
}

void vtk_regular_polygon_source_set_radius(vtkRegularPolygonSource& polygon, double radius) {
    polygon.SetRadius(radius);
}
double vtk_regular_polygon_source_get_radius(const vtkRegularPolygonSource& polygon) {
    return const_cast<vtkRegularPolygonSource&>(polygon).GetRadius();
}

void vtk_regular_polygon_source_set_generate_polygon(vtkRegularPolygonSource& polygon, bool generate) {
    polygon.SetGeneratePolygon(static_cast<vtkTypeBool>(generate));
}
bool vtk_regular_polygon_source_get_generate_polygon(const vtkRegularPolygonSource& polygon) {
    return static_cast<bool>(const_cast<vtkRegularPolygonSource&>(polygon).GetGeneratePolygon());
}

void vtk_regular_polygon_source_generate_polygon_on(vtkRegularPolygonSource& polygon) {
    polygon.GeneratePolygonOn();
}
void vtk_regular_polygon_source_generate_polygon_off(vtkRegularPolygonSource& polygon) {
    polygon.GeneratePolygonOff();
}

void vtk_regular_polygon_source_set_generate_polyline(vtkRegularPolygonSource& polygon, bool generate) {
    polygon.SetGeneratePolyline(static_cast<vtkTypeBool>(generate));
}
bool vtk_regular_polygon_source_get_generate_polyline(const vtkRegularPolygonSource& polygon) {
    return static_cast<bool>(const_cast<vtkRegularPolygonSource&>(polygon).GetGeneratePolyline());
}

void vtk_regular_polygon_source_generate_polyline_on(vtkRegularPolygonSource& polygon) {
    polygon.GeneratePolylineOn();
}
void vtk_regular_polygon_source_generate_polyline_off(vtkRegularPolygonSource& polygon) {
    polygon.GeneratePolylineOff();
}

void vtk_regular_polygon_source_set_output_points_precision(vtkRegularPolygonSource& polygon, int precision) {
    polygon.SetOutputPointsPrecision(precision);
}
int vtk_regular_polygon_source_get_output_points_precision(const vtkRegularPolygonSource& polygon) {
    return const_cast<vtkRegularPolygonSource&>(polygon).GetOutputPointsPrecision();
}

vtkAlgorithmOutput* vtk_regular_polygon_source_get_output_port(vtkRegularPolygonSource& polygon) {
    return polygon.GetOutputPort();
}
