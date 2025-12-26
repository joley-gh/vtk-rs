use vtk_rs::*;

fn main() {
    println!("=== VTK DataArray Demo for FEM ===\n");

    // Scenario: 3-node cantilever beam with displacement results
    println!("Scenario: Cantilever beam FEM analysis");
    println!("  - 3 nodes along beam");
    println!("  - Node 0: Fixed (no displacement)");
    println!("  - Node 1: Middle (small displacement)");
    println!("  - Node 2: Free end (large displacement)\n");

    // === Scalar Data: Displacement Magnitude ===
    println!("=== Scalar Data: Displacement Magnitude ===");
    let mut disp_mag = DoubleArray::new_scalar("DisplacementMagnitude");
    
    // Displacement magnitudes in mm
    disp_mag.insert_next_value(0.0);    // Node 0: Fixed
    disp_mag.insert_next_value(2.5);    // Node 1: 2.5 mm
    disp_mag.insert_next_value(10.0);   // Node 2: 10 mm
    
    println!("Created scalar array: {}", disp_mag.get_name().unwrap());
    println!("  Components: {}", disp_mag.get_number_of_components());
    println!("  Tuples: {}", disp_mag.get_number_of_tuples());
    println!("  Values: {}", disp_mag.get_number_of_values());
    println!("\nDisplacement magnitudes (mm):");
    for i in 0..disp_mag.get_number_of_tuples() {
        println!("  Node {}: {:.2}", i, disp_mag.get_value(i));
    }

    // === Vector Data: Displacement Vectors ===
    println!("\n=== Vector Data: Displacement Vectors ===");
    let mut disp_vec = DoubleArray::new_vector("DisplacementVector");
    
    // Displacement vectors (dx, dy, dz) in mm
    disp_vec.insert_next_tuple3(0.0, 0.0, 0.0);      // Node 0: No movement
    disp_vec.insert_next_tuple3(0.5, -2.0, 0.1);     // Node 1: Slight deflection
    disp_vec.insert_next_tuple3(2.0, -8.0, 0.5);     // Node 2: Large deflection
    
    println!("Created vector array: {}", disp_vec.get_name().unwrap());
    println!("  Components: {}", disp_vec.get_number_of_components());
    println!("  Tuples: {}", disp_vec.get_number_of_tuples());
    println!("  Values: {}", disp_vec.get_number_of_values());
    println!("\nDisplacement vectors (dx, dy, dz) mm:");
    for i in 0..disp_vec.get_number_of_tuples() {
        let vec = disp_vec.get_tuple(i);
        println!("  Node {}: [{:.2}, {:.2}, {:.2}]", i, vec[0], vec[1], vec[2]);
    }

    // === Scalar Data: Stress ===
    println!("\n=== Scalar Data: Von Mises Stress ===");
    let mut stress = DoubleArray::new_scalar("VonMisesStress");
    
    // Stress in MPa (highest at fixed end)
    stress.insert_next_value(150.0);    // Node 0: High stress (fixed)
    stress.insert_next_value(75.0);     // Node 1: Medium stress
    stress.insert_next_value(10.0);     // Node 2: Low stress (free end)
    
    println!("Created stress array: {}", stress.get_name().unwrap());
    println!("\nVon Mises stress (MPa):");
    for i in 0..stress.get_number_of_tuples() {
        println!("  Node {}: {:.1}", i, stress.get_value(i));
    }

    // === Integer Data: Material IDs ===
    println!("\n=== Integer Data: Material IDs ===");
    let mut material_ids = IntArray::new_id_array("MaterialID");
    
    // Assign material IDs (1 = Steel, 2 = Aluminum)
    material_ids.insert_next_value(1);  // Node 0: Steel
    material_ids.insert_next_value(1);  // Node 1: Steel
    material_ids.insert_next_value(1);  // Node 2: Steel
    
    println!("Created material ID array: {}", material_ids.get_name().unwrap());
    println!("\nMaterial IDs:");
    for i in 0..material_ids.get_number_of_tuples() {
        let mat_id = material_ids.get_value(i);
        let mat_name = if mat_id == 1 { "Steel" } else { "Aluminum" };
        println!("  Node {}: {} (ID {})", i, mat_name, mat_id);
    }

    // === Integer Data: Boundary Conditions ===
    println!("\n=== Integer Data: Boundary Conditions ===");
    let mut bc_flags = IntArray::new_id_array("BoundaryCondition");
    
    // BC flags: 0 = Free, 1 = Fixed X, 2 = Fixed Y, 4 = Fixed Z, 7 = Fixed all
    bc_flags.insert_next_value(7);  // Node 0: Fixed in all directions
    bc_flags.insert_next_value(0);  // Node 1: Free
    bc_flags.insert_next_value(0);  // Node 2: Free
    
    println!("Created boundary condition array: {}", bc_flags.get_name().unwrap());
    println!("\nBoundary conditions:");
    for i in 0..bc_flags.get_number_of_tuples() {
        let bc = bc_flags.get_value(i);
        let bc_desc = match bc {
            0 => "Free",
            7 => "Fixed (all directions)",
            _ => "Partially constrained",
        };
        println!("  Node {}: {} (flag {})", i, bc_desc, bc);
    }

    // === Modifying Data ===
    println!("\n=== Modifying Data ===");
    println!("Updating Node 1 displacement magnitude from 2.5 to 3.0 mm");
    disp_mag.set_value(1, 3.0);
    println!("  Node 1 new value: {:.2} mm", disp_mag.get_value(1));

    println!("\n✅ DataArray modules working correctly!");
    println!("\nNext steps:");
    println!("  1. Attach arrays to PolyData using PointData/CellData");
    println!("  2. Visualize with color mapping (LookupTable)");
    println!("  3. Display with ScalarBar for legend");
}
