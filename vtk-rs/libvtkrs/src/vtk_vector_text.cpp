#include "vtk_vector_text.h"
#include "vtk_vector_text.rs.h"

vtkVectorText* vtk_vector_text_new() {
    return vtkVectorText::New();
}

void vtk_vector_text_delete(vtkVectorText& ptr) {
    ptr.Delete();
}

void vector_text_set_text(vtkVectorText& source, rust::String text) {
    std::string cpp_text(text.begin(), text.end());
    source.SetText(cpp_text.c_str());
}

rust::String vector_text_get_text(vtkVectorText& source) {
    const char* text = source.GetText();
    return rust::String(text ? text : "");
}

vtkAlgorithmOutput* vector_text_get_output_port(vtkVectorText& source) {
    return source.GetOutputPort();
}
