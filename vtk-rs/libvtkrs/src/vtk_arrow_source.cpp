#include "vtk_arrow_source.h"
#include "vtk_arrow_source.rs.h"
#include "cxx.h"

vtkArrowSource* vtk_arrow_source_new() {
    return vtkArrowSource::New();
}

void vtk_arrow_source_delete(vtkArrowSource& arrow) {
    arrow.Delete();
}

void vtk_arrow_source_set_arrow_origin_to_default(vtkArrowSource& arrow) {
    arrow.SetArrowOriginToDefault();
}
void vtk_arrow_source_set_arrow_origin_to_center(vtkArrowSource& arrow) {
    arrow.SetArrowOriginToCenter();
}
rust::String vtk_arrow_source_get_arrow_origin_as_string(const vtkArrowSource& arrow) {
    return const_cast<vtkArrowSource&>(arrow).GetArrowOriginAsString();
}

void vtk_arrow_source_set_tip_length(vtkArrowSource& arrow, double length) {
    arrow.SetTipLength(length);
}
double vtk_arrow_source_get_tip_length(const vtkArrowSource& arrow) {
    return const_cast<vtkArrowSource&>(arrow).GetTipLength();
}

void vtk_arrow_source_set_tip_radius(vtkArrowSource& arrow, double radius) {
    arrow.SetTipRadius(radius);
}
double vtk_arrow_source_get_tip_radius(const vtkArrowSource& arrow) {
    return const_cast<vtkArrowSource&>(arrow).GetTipRadius();
}

void vtk_arrow_source_set_tip_resolution(vtkArrowSource& arrow, int resolution) {
    arrow.SetTipResolution(resolution);
}
int vtk_arrow_source_get_tip_resolution(const vtkArrowSource& arrow) {
    return const_cast<vtkArrowSource&>(arrow).GetTipResolution();
}

void vtk_arrow_source_set_shaft_radius(vtkArrowSource& arrow, double radius) {
    arrow.SetShaftRadius(radius);
}
double vtk_arrow_source_get_shaft_radius(const vtkArrowSource& arrow) {
    return const_cast<vtkArrowSource&>(arrow).GetShaftRadius();
}

void vtk_arrow_source_set_shaft_resolution(vtkArrowSource& arrow, int resolution) {
    arrow.SetShaftResolution(resolution);
}
int vtk_arrow_source_get_shaft_resolution(const vtkArrowSource& arrow) {
    return const_cast<vtkArrowSource&>(arrow).GetShaftResolution();
}

void vtk_arrow_source_set_invert(vtkArrowSource& arrow, bool invert) {
    arrow.SetInvert(invert);
}
void vtk_arrow_source_invert_on(vtkArrowSource& arrow) {
    arrow.InvertOn();
}
void vtk_arrow_source_invert_off(vtkArrowSource& arrow) {
    arrow.InvertOff();
}
bool vtk_arrow_source_get_invert(const vtkArrowSource& arrow) {
    return const_cast<vtkArrowSource&>(arrow).GetInvert();
}

vtkAlgorithmOutput* vtk_arrow_source_get_output_port(vtkArrowSource& arrow) {
    return arrow.GetOutputPort();
}
