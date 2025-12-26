#ifndef VTK_TUBE_FILTER_H
#define VTK_TUBE_FILTER_H

#include <cstdint>

// Forward declarations
class vtkTubeFilter;
class vtkAlgorithmOutput;

extern "C" {
    // Lifecycle
    vtkTubeFilter* tube_filter_new();
    void tube_filter_delete(vtkTubeFilter* filter);
    
    // Input/Output
    void tube_filter_set_input_connection(vtkTubeFilter* filter, vtkAlgorithmOutput* input);
    vtkAlgorithmOutput* tube_filter_get_output_port(vtkTubeFilter* filter);
    
    // Radius control
    void tube_filter_set_radius(vtkTubeFilter* filter, double radius);
    double tube_filter_get_radius(vtkTubeFilter* filter);
    
    // Resolution control
    void tube_filter_set_number_of_sides(vtkTubeFilter* filter, int32_t sides);
    int32_t tube_filter_get_number_of_sides(vtkTubeFilter* filter);
    
    // Variable radius
    void tube_filter_set_radius_factor(vtkTubeFilter* filter, double factor);
    double tube_filter_get_radius_factor(vtkTubeFilter* filter);
    
    // Capping
    void tube_filter_set_capping(vtkTubeFilter* filter, int32_t capping);
    int32_t tube_filter_get_capping(vtkTubeFilter* filter);
}

#endif // VTK_TUBE_FILTER_H
