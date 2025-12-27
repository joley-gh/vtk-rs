use vtk_rs::*;

fn main() {
    println!("=== VTK UnstructuredGrid Demo ===\n");
    println!("Building a tetrahedral mesh...\n");

    // Create points for a tetrahedron (4 vertices)
    let mut points = Points::new();
    points.insert_next_point(0.0, 0.0, 0.0); // Point 0: origin
    points.insert_next_point(1.0, 0.0, 0.0); // Point 1: +X
    points.insert_next_point(0.5, 1.0, 0.0); // Point 2: above XY plane
    points.insert_next_point(0.5, 0.5, 1.0); // Point 3: apex
    
    println!("Created {} points", points.get_number_of_points());

    // Create unstructured grid
    let mut grid = UnstructuredGrid::new();
    
    // Set the points
    grid.set_points(&mut points);
    
    // Allocate space for 1 cell
    grid.allocate(1);
    
    // Insert a tetrahedral cell (4 points)
    // VTK_TETRA = 10
    let tetra_points = vec![0, 1, 2, 3];
    grid.insert_next_cell(VtkCellType::Tetra, &tetra_points);
    
    println!("Created {} cells", grid.get_number_of_cells());
    
    // Get grid information
    println!("\n=== Grid Information ===");
    println!("Number of points: {}", grid.get_number_of_points());
    println!("Number of cells: {}", grid.get_number_of_cells());
    
    let bounds = grid.get_bounds();
    println!(
        "Bounds: X[{:.1}, {:.1}] Y[{:.1}, {:.1}] Z[{:.1}, {:.1}]",
        bounds[0], bounds[1], bounds[2], bounds[3], bounds[4], bounds[5]
    );

    println!("\n=== Building a more complex mesh (cube with hexahedra) ===\n");
    
    // Create a 2x2x2 structured grid of points (27 points total: 3x3x3)
    let mut cube_points = Points::new();
    for z in 0..3 {
        for y in 0..3 {
            for x in 0..3 {
                cube_points.insert_next_point(x as f64, y as f64, z as f64);
            }
        }
    }
    
    println!("Created {} points for cube grid", cube_points.get_number_of_points());
    
    // Create grid for the cube
    let mut cube_grid = UnstructuredGrid::new();
    cube_grid.set_points(&mut cube_points);
    
    // Allocate for 8 hexahedral cells (2x2x2 cube of cells)
    cube_grid.allocate(8);
    
    // Helper function to get point index in 3D grid
    let point_idx = |x: i32, y: i32, z: i32| -> i32 { x + y * 3 + z * 9 };
    
    // Insert 8 hexahedral cells
    let mut cell_count = 0;
    for z in 0..2 {
        for y in 0..2 {
            for x in 0..2 {
                // Hexahedron has 8 vertices in specific VTK ordering
                let hex_points = vec![
                    point_idx(x, y, z),         // 0: bottom-front-left
                    point_idx(x + 1, y, z),     // 1: bottom-front-right
                    point_idx(x + 1, y + 1, z), // 2: bottom-back-right
                    point_idx(x, y + 1, z),     // 3: bottom-back-left
                    point_idx(x, y, z + 1),     // 4: top-front-left
                    point_idx(x + 1, y, z + 1), // 5: top-front-right
                    point_idx(x + 1, y + 1, z + 1), // 6: top-back-right
                    point_idx(x, y + 1, z + 1), // 7: top-back-left
                ];
                cube_grid.insert_next_cell(VtkCellType::Hexahedron, &hex_points);
                cell_count += 1;
            }
        }
    }
    
    println!("Inserted {} hexahedral cells", cell_count);
    
    // Get cube grid information
    println!("\n=== Cube Grid Information ===");
    println!("Number of points: {}", cube_grid.get_number_of_points());
    println!("Number of cells: {}", cube_grid.get_number_of_cells());
    
    let cube_bounds = cube_grid.get_bounds();
    println!(
        "Bounds: X[{:.1}, {:.1}] Y[{:.1}, {:.1}] Z[{:.1}, {:.1}]",
        cube_bounds[0], cube_bounds[1], cube_bounds[2], 
        cube_bounds[3], cube_bounds[4], cube_bounds[5]
    );
    
    println!("\n✓ UnstructuredGrid meshes created successfully!");
    println!("\nSupported cell types:");
    println!("  - Vertex, Line, Triangle, Quad");
    println!("  - Tetrahedron, Hexahedron, Wedge, Pyramid");
    println!("  - PolyVertex, PolyLine, Polygon");
    println!("  - And more quadratic cell types...");
}
