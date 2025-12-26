use vtk_rs::*;

fn main() {
    println!("=== FEM Beam Structure with Data Arrays ===\n");
    println!("Creating a cantilever beam:");
    println!("  - 4 nodes along the beam");
    println!("  - 3 beam elements connecting nodes");
    println!("  - Node data: Displacements, boundary conditions");
    println!("  - Element data: Material IDs, element stresses\n");

    // === Create Geometry ===
    println!("=== Creating Geometry ===");
    
    // Define nodes
    let mut points = Points::new();
    let n0 = points.insert_next_point(0.0, 0.0, 0.0);  // Fixed end
    let n1 = points.insert_next_point(1.0, 0.0, 0.0);
    let n2 = points.insert_next_point(2.0, 0.0, 0.0);
    let n3 = points.insert_next_point(3.0, 0.0, 0.0);  // Free end
    println!("Created {} nodes", points.get_number_of_points());

    // Define beam elements
    let mut beams = CellArray::new();
    beams.insert_next_cell(&[n0, n1]);
    beams.insert_next_cell(&[n1, n2]);
    beams.insert_next_cell(&[n2, n3]);
    println!("Created {} beam elements", beams.get_number_of_cells());

    // Create PolyData
    let poly_data = PolyData::from_beam_structure(&points, &beams);
    println!("PolyData: {} points, {} cells\n", 
             poly_data.get_number_of_points(), 
             poly_data.get_number_of_cells());

    // === Attach Node Data (PointData) ===
    println!("=== Attaching Node Data (PointData) ===");
    
    // Displacement magnitudes (mm)
    let mut disp_mag = DoubleArray::new_scalar("DisplacementMagnitude");
    disp_mag.insert_next_value(0.0);    // Node 0: Fixed
    disp_mag.insert_next_value(2.5);    // Node 1
    disp_mag.insert_next_value(7.5);    // Node 2
    disp_mag.insert_next_value(15.0);   // Node 3: Free end
    println!("Created displacement magnitude array");

    // Displacement vectors (dx, dy, dz) in mm
    let mut disp_vec = DoubleArray::new_vector("DisplacementVector");
    disp_vec.insert_next_tuple3(0.0, 0.0, 0.0);       // Node 0: No movement
    disp_vec.insert_next_tuple3(0.5, -2.4, 0.0);      // Node 1
    disp_vec.insert_next_tuple3(1.5, -7.2, 0.1);      // Node 2
    disp_vec.insert_next_tuple3(3.0, -14.5, 0.2);     // Node 3
    println!("Created displacement vector array");

    // Boundary condition flags (7 = fixed all, 0 = free)
    let mut bc_flags = IntArray::new_id_array("BoundaryCondition");
    bc_flags.insert_next_value(7);  // Node 0: Fixed
    bc_flags.insert_next_value(0);  // Node 1: Free
    bc_flags.insert_next_value(0);  // Node 2: Free
    bc_flags.insert_next_value(0);  // Node 3: Free
    println!("Created boundary condition array");

    // Attach arrays to PointData
    let mut point_data = poly_data.get_point_data();
    point_data.add_array(&disp_mag);
    point_data.add_array(&disp_vec);
    point_data.add_int_array(&bc_flags);
    point_data.set_active_scalars("DisplacementMagnitude");
    println!("Attached {} arrays to PointData", point_data.get_number_of_arrays());
    println!("Active scalars: DisplacementMagnitude (for color mapping)\n");

    // === Attach Element Data (CellData) ===
    println!("=== Attaching Element Data (CellData) ===");
    
    // Material IDs (1 = Steel, 2 = Aluminum)
    let mut material_ids = IntArray::new_id_array("MaterialID");
    material_ids.insert_next_value(1);  // Element 0: Steel
    material_ids.insert_next_value(1);  // Element 1: Steel
    material_ids.insert_next_value(1);  // Element 2: Steel
    println!("Created material ID array");

    // Element stresses (MPa)
    let mut elem_stress = DoubleArray::new_scalar("ElementStress");
    elem_stress.insert_next_value(180.0);   // Element 0: High stress (near fixed end)
    elem_stress.insert_next_value(120.0);   // Element 1: Medium stress
    elem_stress.insert_next_value(60.0);    // Element 2: Low stress (near free end)
    println!("Created element stress array");

    // Cross-section types (1 = Rectangle, 2 = Circle, 3 = I-beam)
    let mut cross_section = IntArray::new_id_array("CrossSectionType");
    cross_section.insert_next_value(1);  // Element 0: Rectangle
    cross_section.insert_next_value(1);  // Element 1: Rectangle
    cross_section.insert_next_value(1);  // Element 2: Rectangle
    println!("Created cross-section type array");

    // Attach arrays to CellData
    let mut cell_data = poly_data.get_cell_data();
    cell_data.add_int_array(&material_ids);
    cell_data.add_array(&elem_stress);
    cell_data.add_int_array(&cross_section);
    cell_data.set_active_scalars("ElementStress");
    println!("Attached {} arrays to CellData", cell_data.get_number_of_arrays());
    println!("Active scalars: ElementStress (for color mapping)\n");

    // === Summary ===
    println!("=== Summary ===");
    println!("PolyData Structure:");
    println!("  Points: {}", poly_data.get_number_of_points());
    println!("  Cells: {}", poly_data.get_number_of_cells());
    
    println!("\nPointData (Node Attributes):");
    for i in 0..point_data.get_number_of_arrays() {
        if let Some(name) = point_data.get_array_name(i) {
            println!("  - {}", name);
        }
    }
    
    println!("\nCellData (Element Attributes):");
    for i in 0..cell_data.get_number_of_arrays() {
        if let Some(name) = cell_data.get_array_name(i) {
            println!("  - {}", name);
        }
    }

    println!("\n✅ Complete FEM beam structure with data arrays!");
    println!("\nNext steps:");
    println!("  1. Visualize with PolyDataMapper (color by displacement/stress)");
    println!("  2. Add TubeFilter to render beams as 3D tubes");
    println!("  3. Add Glyph3D to show nodes as spheres");
    println!("  4. Add LookupTable for custom color mapping");
    println!("  5. Add ScalarBar for color legend");
}
