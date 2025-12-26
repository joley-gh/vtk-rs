#include "vtk_tube_filter.h"
#include <vtkTubeFilter.h>
#include <vtkAlgorithmOutput.h>
#include <iostream>

extern "C" {

vtkTubeFilter* tube_filter_new() {
    vtkTubeFilter* filter = vtkTubeFilter::New();
    std::cout << "DEBUG: tube_filter_new() called" << std::endl;
    std::cout << "DEBUG: vtkTubeFilter::New() returned: " << filter << std::endl;
    return filter;
}

void tube_filter_delete(vtkTubeFilter* filter) {
    if (filter) {
        filter->Delete();
    }
}

void tube_filter_set_input_connection(vtkTubeFilter* filter, vtkAlgorithmOutput* input) {
    if (filter && input) {
        filter->SetInputConnection(input);
    }
}

vtkAlgorithmOutput* tube_filter_get_output_port(vtkTubeFilter* filter) {
    if (filter) {
        return filter->GetOutputPort();
    }
    return nullptr;
}

void tube_filter_set_radius(vtkTubeFilter* filter, double radius) {
    if (filter) {
        filter->SetRadius(radius);
    }
}

double tube_filter_get_radius(vtkTubeFilter* filter) {
    if (filter) {
        return filter->GetRadius();
    }
    return 0.0;
}

void tube_filter_set_number_of_sides(vtkTubeFilter* filter, int32_t sides) {
    if (filter) {
        filter->SetNumberOfSides(sides);
    }
}

int32_t tube_filter_get_number_of_sides(vtkTubeFilter* filter) {
    if (filter) {
        return filter->GetNumberOfSides();
    }
    return 0;
}

void tube_filter_set_radius_factor(vtkTubeFilter* filter, double factor) {
    if (filter) {
        filter->SetRadiusFactor(factor);
    }
}

double tube_filter_get_radius_factor(vtkTubeFilter* filter) {
    if (filter) {
        return filter->GetRadiusFactor();
    }
    return 0.0;
}

void tube_filter_set_capping(vtkTubeFilter* filter, int32_t capping) {
    if (filter) {
        filter->SetCapping(capping);
    }
}

int32_t tube_filter_get_capping(vtkTubeFilter* filter) {
    if (filter) {
        return filter->GetCapping();
    }
    return 0;
}

} // extern "C"
