#ifndef VTK_GLYPH_3D_H
#define VTK_GLYPH_3D_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Forward declarations
typedef struct vtkGlyph3D vtkGlyph3D;
typedef struct vtkAlgorithmOutput vtkAlgorithmOutput;

// Lifecycle
vtkGlyph3D* glyph_3d_new();
void glyph_3d_delete(vtkGlyph3D* glyph);

// Input/Output connections
void glyph_3d_set_input_connection(vtkGlyph3D* glyph, vtkAlgorithmOutput* input);
void glyph_3d_set_source_connection(vtkGlyph3D* glyph, vtkAlgorithmOutput* source);
vtkAlgorithmOutput* glyph_3d_get_output_port(vtkGlyph3D* glyph);

// Scaling control
void glyph_3d_set_scale_factor(vtkGlyph3D* glyph, double factor);
double glyph_3d_get_scale_factor(vtkGlyph3D* glyph);

// Scale mode: controls how glyphs are sized
void glyph_3d_set_scale_mode_to_scale_by_scalar(vtkGlyph3D* glyph);
void glyph_3d_set_scale_mode_to_scale_by_vector(vtkGlyph3D* glyph);
void glyph_3d_set_scale_mode_to_scale_by_vector_components(vtkGlyph3D* glyph);
void glyph_3d_set_scale_mode_to_data_scaling_off(vtkGlyph3D* glyph);
int32_t glyph_3d_get_scale_mode(vtkGlyph3D* glyph);

// Scaling enable/disable
void glyph_3d_set_scaling(vtkGlyph3D* glyph, int32_t enable);
int32_t glyph_3d_get_scaling(vtkGlyph3D* glyph);

// Orientation control
void glyph_3d_set_orient(vtkGlyph3D* glyph, int32_t enable);
int32_t glyph_3d_get_orient(vtkGlyph3D* glyph);

// Clamping: prevent glyphs from being too small/large
void glyph_3d_set_clamping(vtkGlyph3D* glyph, int32_t enable);
int32_t glyph_3d_get_clamping(vtkGlyph3D* glyph);

// Color mode: how glyph colors are determined
void glyph_3d_set_color_mode_to_color_by_scale(vtkGlyph3D* glyph);
void glyph_3d_set_color_mode_to_color_by_scalar(vtkGlyph3D* glyph);
void glyph_3d_set_color_mode_to_color_by_vector(vtkGlyph3D* glyph);
int32_t glyph_3d_get_color_mode(vtkGlyph3D* glyph);

#ifdef __cplusplus
}
#endif

#endif // VTK_GLYPH_3D_H
