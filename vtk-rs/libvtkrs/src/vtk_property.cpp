#include "cxx.h"
#include "vtk_property.h"
#include "vtk_property.rs.h"

#include <vtkProperty.h>

vtkProperty* property_new() {
    vtkProperty* obj = vtkProperty::New();
    if (!obj) {
        throw std::runtime_error("Failed to create vtkProperty");
    }
    return obj;
}

void property_delete(vtkProperty& property) {
    property.Delete();
}

// Color and opacity
void property_set_color(vtkProperty& property, double r, double g, double b) {
    property.SetColor(r, g, b);
}

void property_get_color(vtkProperty& property, double& r, double& g, double& b) {
    double color[3];
    property.GetColor(color);
    r = color[0];
    g = color[1];
    b = color[2];
}

void property_set_opacity(vtkProperty& property, double opacity) {
    property.SetOpacity(opacity);
}

double property_get_opacity(vtkProperty& property) {
    return property.GetOpacity();
}

// Line and point size
void property_set_line_width(vtkProperty& property, double width) {
    property.SetLineWidth(width);
}

double property_get_line_width(vtkProperty& property) {
    return property.GetLineWidth();
}

void property_set_point_size(vtkProperty& property, double size) {
    property.SetPointSize(size);
}

double property_get_point_size(vtkProperty& property) {
    return property.GetPointSize();
}

// Representation type
void property_set_representation(vtkProperty& property, int representation) {
    property.SetRepresentation(representation);
}

int property_get_representation(vtkProperty& property) {
    return property.GetRepresentation();
}

// Edge visibility
void property_set_edge_visibility(vtkProperty& property, bool visible) {
    property.SetEdgeVisibility(visible ? 1 : 0);
}

bool property_get_edge_visibility(vtkProperty& property) {
    return property.GetEdgeVisibility() != 0;
}

void property_set_edge_color(vtkProperty& property, double r, double g, double b) {
    property.SetEdgeColor(r, g, b);
}

void property_get_edge_color(vtkProperty& property, double& r, double& g, double& b) {
    double color[3];
    property.GetEdgeColor(color);
    r = color[0];
    g = color[1];
    b = color[2];
}

// Interpolation type
void property_set_interpolation(vtkProperty& property, int interpolation) {
    property.SetInterpolation(interpolation);
}

int property_get_interpolation(vtkProperty& property) {
    return property.GetInterpolation();
}

// Lighting properties
void property_set_ambient(vtkProperty& property, double ambient) {
    property.SetAmbient(ambient);
}

double property_get_ambient(vtkProperty& property) {
    return property.GetAmbient();
}

void property_set_diffuse(vtkProperty& property, double diffuse) {
    property.SetDiffuse(diffuse);
}

double property_get_diffuse(vtkProperty& property) {
    return property.GetDiffuse();
}

void property_set_specular(vtkProperty& property, double specular) {
    property.SetSpecular(specular);
}

double property_get_specular(vtkProperty& property) {
    return property.GetSpecular();
}

void property_set_specular_power(vtkProperty& property, double power) {
    property.SetSpecularPower(power);
}

double property_get_specular_power(vtkProperty& property) {
    return property.GetSpecularPower();
}
