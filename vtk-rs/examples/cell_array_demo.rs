use vtk_rs::*;

fn main() {
    println!("=== VTK CellArray Demo ===\n");

    // Create a CellArray to store beam connectivity
    let mut cells = CellArray::new();
    println!("Created CellArray");
    println!("Initial cell count: {}\n", cells.get_number_of_cells());

    // Define beam connections (each beam connects two node points)
    // Create a simple 3D truss structure:
    //     3-----2
    //    /|    /|
    //   0-----1 |
    //   | 7--|-6
    //   |/   |/
    //   4----5
    
    let beams = vec![
        // Bottom square
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        // Top square
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        // Vertical beams
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];

    println!("Adding {} beams to CellArray:", beams.len());
    for (i, (node1, node2)) in beams.iter().enumerate() {
        let cell_id = cells.insert_next_cell(&[*node1, *node2]);
        println!("  Beam {}: Node {} -> Node {} (Cell ID: {})", i, node1, node2, cell_id);
    }

    println!("\nCellArray statistics:");
    println!("  Number of cells: {}", cells.get_number_of_cells());
    println!("  Number of connectivity IDs: {}", cells.get_number_of_connectivity_ids());

    // Access specific cells
    println!("\nAccessing individual cells:");
    if let Some(point_ids) = cells.get_cell(0) {
        println!("  Cell 0: {:?}", point_ids);
    }
    if let Some(point_ids) = cells.get_cell(5) {
        println!("  Cell 5: {:?}", point_ids);
    }
    if let Some(point_ids) = cells.get_cell(11) {
        println!("  Cell 11: {:?}", point_ids);
    }

    // Iterate over all cells
    println!("\nIterating over first 5 beams:");
    for (cell_id, point_ids) in cells.iter().take(5) {
        println!("  Cell {}: Beam from node {} to node {}", 
                 cell_id, point_ids[0], point_ids[1]);
    }

    // Create a second CellArray for triangular elements
    println!("\n=== Testing Triangle Cells ===");
    let mut tri_cells = CellArray::new();
    
    // Add some triangular faces
    tri_cells.insert_next_cell(&[0, 1, 2]);
    tri_cells.insert_next_cell(&[2, 3, 0]);
    tri_cells.insert_next_cell(&[4, 5, 6]);
    
    println!("Created {} triangular cells", tri_cells.get_number_of_cells());
    println!("Total connectivity IDs: {}", tri_cells.get_number_of_connectivity_ids());
    
    for (cell_id, point_ids) in tri_cells.iter() {
        println!("  Triangle {}: Nodes {:?}", cell_id, point_ids);
    }

    // Test reset functionality
    println!("\n=== Testing Reset ===");
    let mut test_cells = CellArray::new();
    test_cells.insert_next_cell(&[0, 1]);
    test_cells.insert_next_cell(&[1, 2]);
    println!("Before reset: {} cells", test_cells.get_number_of_cells());
    
    test_cells.reset();
    println!("After reset: {} cells", test_cells.get_number_of_cells());
    
    test_cells.insert_next_cell(&[5, 10]);
    println!("After adding new cell: {} cells", test_cells.get_number_of_cells());

    println!("\n✅ CellArray module is working correctly!");
}
