#include "../include/vtk_glyph_3d.h"
#include <vtkGlyph3D.h>
#include <vtkAlgorithmOutput.h>
#include <iostream>

extern "C" {

vtkGlyph3D* glyph_3d_new() {
    vtkGlyph3D* glyph = vtkGlyph3D::New();
    std::cout << "DEBUG: glyph_3d_new() called" << std::endl;
    std::cout << "DEBUG: vtkGlyph3D::New() returned: " << glyph << std::endl;
    return glyph;
}

void glyph_3d_delete(vtkGlyph3D* glyph) {
    if (glyph) {
        glyph->Delete();
    }
}

void glyph_3d_set_input_connection(vtkGlyph3D* glyph, vtkAlgorithmOutput* input) {
    if (glyph && input) {
        glyph->SetInputConnection(input);
    }
}

void glyph_3d_set_source_connection(vtkGlyph3D* glyph, vtkAlgorithmOutput* source) {
    if (glyph && source) {
        glyph->SetSourceConnection(source);
    }
}

vtkAlgorithmOutput* glyph_3d_get_output_port(vtkGlyph3D* glyph) {
    if (glyph) {
        return glyph->GetOutputPort();
    }
    return nullptr;
}

void glyph_3d_set_scale_factor(vtkGlyph3D* glyph, double factor) {
    if (glyph) {
        glyph->SetScaleFactor(factor);
    }
}

double glyph_3d_get_scale_factor(vtkGlyph3D* glyph) {
    if (glyph) {
        return glyph->GetScaleFactor();
    }
    return 1.0;
}

void glyph_3d_set_scale_mode_to_scale_by_scalar(vtkGlyph3D* glyph) {
    if (glyph) {
        glyph->SetScaleModeToScaleByScalar();
    }
}

void glyph_3d_set_scale_mode_to_scale_by_vector(vtkGlyph3D* glyph) {
    if (glyph) {
        glyph->SetScaleModeToScaleByVector();
    }
}

void glyph_3d_set_scale_mode_to_scale_by_vector_components(vtkGlyph3D* glyph) {
    if (glyph) {
        glyph->SetScaleModeToScaleByVectorComponents();
    }
}

void glyph_3d_set_scale_mode_to_data_scaling_off(vtkGlyph3D* glyph) {
    if (glyph) {
        glyph->SetScaleModeToDataScalingOff();
    }
}

int32_t glyph_3d_get_scale_mode(vtkGlyph3D* glyph) {
    if (glyph) {
        return glyph->GetScaleMode();
    }
    return 0;
}

void glyph_3d_set_scaling(vtkGlyph3D* glyph, int32_t enable) {
    if (glyph) {
        glyph->SetScaling(enable);
    }
}

int32_t glyph_3d_get_scaling(vtkGlyph3D* glyph) {
    if (glyph) {
        return glyph->GetScaling();
    }
    return 0;
}

void glyph_3d_set_orient(vtkGlyph3D* glyph, int32_t enable) {
    if (glyph) {
        glyph->SetOrient(enable);
    }
}

int32_t glyph_3d_get_orient(vtkGlyph3D* glyph) {
    if (glyph) {
        return glyph->GetOrient();
    }
    return 0;
}

void glyph_3d_set_clamping(vtkGlyph3D* glyph, int32_t enable) {
    if (glyph) {
        glyph->SetClamping(enable);
    }
}

int32_t glyph_3d_get_clamping(vtkGlyph3D* glyph) {
    if (glyph) {
        return glyph->GetClamping();
    }
    return 0;
}

void glyph_3d_set_color_mode_to_color_by_scale(vtkGlyph3D* glyph) {
    if (glyph) {
        glyph->SetColorModeToColorByScale();
    }
}

void glyph_3d_set_color_mode_to_color_by_scalar(vtkGlyph3D* glyph) {
    if (glyph) {
        glyph->SetColorModeToColorByScalar();
    }
}

void glyph_3d_set_color_mode_to_color_by_vector(vtkGlyph3D* glyph) {
    if (glyph) {
        glyph->SetColorModeToColorByVector();
    }
}

int32_t glyph_3d_get_color_mode(vtkGlyph3D* glyph) {
    if (glyph) {
        return glyph->GetColorMode();
    }
    return 0;
}

} // extern "C"
