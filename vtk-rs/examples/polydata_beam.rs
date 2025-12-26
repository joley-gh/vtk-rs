use vtk_rs::*;

fn main() {
    println!("=== VTK PolyData Beam Structure Demo ===\n");

    // Create nodes (points) for a simple 3D truss
    let mut points = Points::new();
    
    // Bottom square (z=0)
    let n0 = points.insert_next_point(0.0, 0.0, 0.0);
    let n1 = points.insert_next_point(2.0, 0.0, 0.0);
    let n2 = points.insert_next_point(2.0, 2.0, 0.0);
    let n3 = points.insert_next_point(0.0, 2.0, 0.0);
    
    // Top square (z=3)
    let n4 = points.insert_next_point(0.0, 0.0, 3.0);
    let n5 = points.insert_next_point(2.0, 0.0, 3.0);
    let n6 = points.insert_next_point(2.0, 2.0, 3.0);
    let n7 = points.insert_next_point(0.0, 2.0, 3.0);

    println!("Created {} nodes:", points.get_number_of_points());
    for (id, x, y, z) in points.iter() {
        println!("  Node {}: ({:.1}, {:.1}, {:.1})", id, x, y, z);
    }

    // Create beam elements (lines connecting nodes)
    let mut beams = CellArray::new();
    
    // Bottom square
    beams.insert_next_cell(&[n0, n1]);
    beams.insert_next_cell(&[n1, n2]);
    beams.insert_next_cell(&[n2, n3]);
    beams.insert_next_cell(&[n3, n0]);
    
    // Top square
    beams.insert_next_cell(&[n4, n5]);
    beams.insert_next_cell(&[n5, n6]);
    beams.insert_next_cell(&[n6, n7]);
    beams.insert_next_cell(&[n7, n4]);
    
    // Vertical columns
    beams.insert_next_cell(&[n0, n4]);
    beams.insert_next_cell(&[n1, n5]);
    beams.insert_next_cell(&[n2, n6]);
    beams.insert_next_cell(&[n3, n7]);

    println!("\nCreated {} beam elements:", beams.get_number_of_cells());
    for (beam_id, node_ids) in beams.iter() {
        println!("  Beam {}: Node {} → Node {}", beam_id, node_ids[0], node_ids[1]);
    }

    // Create PolyData and combine points + beams
    println!("\n=== Creating PolyData ===");
    let poly_data = PolyData::from_beam_structure(&points, &beams);
    
    println!("PolyData created:");
    println!("  Points: {}", poly_data.get_number_of_points());
    println!("  Lines: {}", poly_data.get_number_of_lines());
    println!("  Total cells: {}", poly_data.get_number_of_cells());

    // Get and display bounds
    let (xmin, xmax, ymin, ymax, zmin, zmax) = poly_data.get_bounds();
    println!("\nSpatial bounds:");
    println!("  X: [{:.1}, {:.1}]", xmin, xmax);
    println!("  Y: [{:.1}, {:.1}]", ymin, ymax);
    println!("  Z: [{:.1}, {:.1}]", zmin, zmax);
    
    let width = xmax - xmin;
    let depth = ymax - ymin;
    let height = zmax - zmin;
    println!("  Dimensions: {:.1} × {:.1} × {:.1}", width, depth, height);

    // Test visualization setup
    println!("\n=== Testing Visualization Pipeline ===");
    
    // Create mapper and connect PolyData
    let mut mapper = PolyDataMapper::new();
    // Note: Need to create AlgorithmOutputPort from PolyData
    // For now, test that the data structure is correct
    
    println!("✅ PolyData beam structure created successfully!");
    println!("\nNext steps for full visualization:");
    println!("  1. Add PolyData → AlgorithmOutputPort conversion");
    println!("  2. Connect to PolyDataMapper");
    println!("  3. Add TubeFilter to visualize beams as 3D tubes");
    println!("  4. Add DataArrays for displacement/deformation properties");
}
