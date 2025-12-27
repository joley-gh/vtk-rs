use vtk_rs::*;

fn main() {
    println!("=== VTK ImageData Demo ===\n");

    // Create a 3D image data (volumetric data)
    println!("Creating ImageData...");
    let mut image_data = ImageData::new();
    println!("ImageData created");

    // Set dimensions (10x10x10 voxel grid)
    println!("Setting dimensions...");
    image_data.set_dimensions(10, 10, 10);
    println!("Getting dimensions...");
    println!("Dimensions: {:?}", image_data.get_dimensions());

    // Set spacing (1.0 units between voxels)
    image_data.set_spacing(1.0, 1.0, 1.0);
    println!("Spacing: {:?}", image_data.get_spacing());

    // Set origin (where the dataset starts in 3D space)
    image_data.set_origin(0.0, 0.0, 0.0);
    println!("Origin: {:?}", image_data.get_origin());

    // Allocate scalar data (one double value per voxel)
    image_data.allocate_scalars(VtkDataType::Double, 1);

    println!("\nNumber of points: {}", image_data.get_number_of_points());
    println!("Number of cells: {}", image_data.get_number_of_cells());

    // Fill the volume with a distance field from center
    let (nx, ny, nz) = image_data.get_dimensions();
    let center_x = (nx as f64) / 2.0;
    let center_y = (ny as f64) / 2.0;
    let center_z = (nz as f64) / 2.0;

    println!(
        "\nFilling volume with distance field from center ({}, {}, {})...",
        center_x,
        center_y,
        center_z
    );

    for z in 0..nz {
        for y in 0..ny {
            for x in 0..nx {
                let dx = (x as f64) - center_x;
                let dy = (y as f64) - center_y;
                let dz = (z as f64) - center_z;
                let distance = (dx * dx + dy * dy + dz * dz).sqrt();

                image_data.set_scalar_component(x, y, z, 0, distance);
            }
        }
    }

    // Read back some values
    println!("\nSample voxel values:");
    println!("  Voxel (0, 0, 0): {:.2}", image_data.get_scalar_component(0, 0, 0, 0));
    println!("  Voxel (5, 5, 5): {:.2}", image_data.get_scalar_component(5, 5, 5, 0));
    println!("  Voxel (9, 9, 9): {:.2}", image_data.get_scalar_component(9, 9, 9, 0));

    // Get bounds
    let bounds = image_data.get_bounds();
    println!(
        "\nBounds: [{:.1}, {:.1}, {:.1}, {:.1}, {:.1}, {:.1}]",
        bounds[0],
        bounds[1],
        bounds[2],
        bounds[3],
        bounds[4],
        bounds[5]
    );

    println!("\n✓ ImageData created and filled successfully!");
}
