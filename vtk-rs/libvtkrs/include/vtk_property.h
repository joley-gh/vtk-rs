#pragma once

#include <vtkProperty.h>

vtkProperty* property_new();
void property_delete(vtkProperty& property);

// Color and opacity
void property_set_color(vtkProperty& property, double r, double g, double b);
void property_get_color(vtkProperty& property, double& r, double& g, double& b);
void property_set_opacity(vtkProperty& property, double opacity);
double property_get_opacity(vtkProperty& property);

// Line and point size
void property_set_line_width(vtkProperty& property, double width);
double property_get_line_width(vtkProperty& property);
void property_set_point_size(vtkProperty& property, double size);
double property_get_point_size(vtkProperty& property);

// Representation type: 0=POINTS, 1=WIREFRAME, 2=SURFACE
void property_set_representation(vtkProperty& property, int representation);
int property_get_representation(vtkProperty& property);

// Edge visibility
void property_set_edge_visibility(vtkProperty& property, bool visible);
bool property_get_edge_visibility(vtkProperty& property);
void property_set_edge_color(vtkProperty& property, double r, double g, double b);
void property_get_edge_color(vtkProperty& property, double& r, double& g, double& b);

// Interpolation type: 0=FLAT, 1=GOURAUD, 2=PHONG
void property_set_interpolation(vtkProperty& property, int interpolation);
int property_get_interpolation(vtkProperty& property);

// Lighting properties
void property_set_ambient(vtkProperty& property, double ambient);
double property_get_ambient(vtkProperty& property);
void property_set_diffuse(vtkProperty& property, double diffuse);
double property_get_diffuse(vtkProperty& property);
void property_set_specular(vtkProperty& property, double specular);
double property_get_specular(vtkProperty& property);
void property_set_specular_power(vtkProperty& property, double power);
double property_get_specular_power(vtkProperty& property);
