#include "vtk_text_source.h"
#include "vtk_text_source.rs.h"

vtkTextSource* vtk_text_source_new() {
    return vtkTextSource::New();
}

void vtk_text_source_delete(vtkTextSource& ptr) {
    ptr.Delete();
}

void text_source_set_text(vtkTextSource& source, rust::String text) {
    std::string cpp_text(text.data(), text.size());
    source.SetText(cpp_text.c_str());
}

rust::String text_source_get_text(vtkTextSource& source) {
    const char* text = source.GetText();
    return std::string(text ? text : "");
}

void text_source_set_backing(vtkTextSource& source, bool backing) {
    source.SetBacking(backing ? 1 : 0);
}

bool text_source_get_backing(vtkTextSource& source) {
    return source.GetBacking() != 0;
}

vtkAlgorithmOutput* text_source_get_output_port(vtkTextSource& source) {
    return source.GetOutputPort();
}
