use vtk_rs::*;

fn main() {
    println!("=== FEM Beam Visualization ===\n");
    println!("Cantilever beam - Overlaid Initial and Deformed Shape:");
    println!("  - 4 nodes, 3 beam elements");
    println!("  - Gray: Initial geometry (reference)");
    println!("  - Colored: Deformed shape (overlaid, color-mapped by stress)");
    println!("  - Displacement scale: 0.1x (for visibility)\n");

    // Original node positions
    let original_positions = [
        (0.0, 0.0, 0.0), // Node 0: Fixed end
        (1.0, 0.0, 0.0), // Node 1
        (2.0, 0.0, 0.0), // Node 2
        (3.0, 0.0, 0.0), // Node 3: Free end
    ];

    // Displacement vectors (dy component - vertical deflection)
    let displacements = [
        (0.0, 0.0, 0.0), // Node 0: Fixed (no displacement)
        (0.0, 0.1, 0.0), // Node 1: 0.1 units down
        (0.0, 0.5, 0.0), // Node 2: 0.5 units down
        (0.0, 1.0, 0.0), // Node 3: 1.0 units down (max)
    ];

    // Displacement scale factor for visualization
    let scale = 0.1;

    println!("Original positions:");
    for (i, (x, y, z)) in original_positions.iter().enumerate() {
        println!("  Node {}: ({:.2}, {:.2}, {:.2})", i, x, y, z);
    }

    println!("\nDisplacement vectors:");
    for (i, (dx, dy, dz)) in displacements.iter().enumerate() {
        let mag = ((dx * dx + dy * dy + dz * dz) as f64).sqrt();
        println!("  Node {}: ({:.3}, {:.3}, {:.3}) - magnitude: {:.3}", i, dx, dy, dz, mag);
    }

    println!("\nDeformed positions (scale = {}):", scale);
    for (i, ((x, y, z), (dx, dy, dz))) in original_positions
        .iter()
        .zip(displacements.iter())
        .enumerate() {
        let def_x = x + dx * scale;
        let def_y = y + dy * scale;
        let def_z = z + dz * scale;
        println!("  Node {}: ({:.2}, {:.2}, {:.2})", i, def_x, def_y, def_z);
    }

    // Element connectivity
    let connectivity = [
        (0, 1),
        (1, 2),
        (2, 3),
    ];

    // Element stresses
    let stresses = [180.0, 120.0, 60.0]; // MPa

    println!(
        "\n✓ Created beam geometry with {} nodes, {} elements",
        original_positions.len(),
        connectivity.len()
    );

    // ========== INITIAL GEOMETRY (UNDEFORMED) ==========
    println!("\n=== Creating Initial Geometry (Left Side) ===");

    // Create lines for initial beam
    let mut line_initial_1 = LineSource::new();
    line_initial_1.set_point1(
        original_positions[0].0,
        original_positions[0].1,
        original_positions[0].2
    );
    line_initial_1.set_point2(
        original_positions[1].0,
        original_positions[1].1,
        original_positions[1].2
    );
    line_initial_1.set_resolution(10);

    let mut line_initial_2 = LineSource::new();
    line_initial_2.set_point1(
        original_positions[1].0,
        original_positions[1].1,
        original_positions[1].2
    );
    line_initial_2.set_point2(
        original_positions[2].0,
        original_positions[2].1,
        original_positions[2].2
    );
    line_initial_2.set_resolution(10);

    let mut line_initial_3 = LineSource::new();
    line_initial_3.set_point1(
        original_positions[2].0,
        original_positions[2].1,
        original_positions[2].2
    );
    line_initial_3.set_point2(
        original_positions[3].0,
        original_positions[3].1,
        original_positions[3].2
    );
    line_initial_3.set_resolution(10);

    // Create mappers and actors for initial geometry
    let mut mapper_initial_1 = PolyDataMapper::new();
    mapper_initial_1.set_input_connection(LineSource::get_output_port(&mut line_initial_1));
    let mut actor_initial_1 = Actor::new();
    actor_initial_1.set_mapper(&mut mapper_initial_1);
    let mut prop_initial_1 = actor_initial_1.get_property();
    prop_initial_1.set_color(0.5, 0.5, 0.5); // Gray
    prop_initial_1.set_line_width(4.0);

    let mut mapper_initial_2 = PolyDataMapper::new();
    mapper_initial_2.set_input_connection(LineSource::get_output_port(&mut line_initial_2));
    let mut actor_initial_2 = Actor::new();
    actor_initial_2.set_mapper(&mut mapper_initial_2);
    let mut prop_initial_2 = actor_initial_2.get_property();
    prop_initial_2.set_color(0.5, 0.5, 0.5); // Gray
    prop_initial_2.set_line_width(4.0);

    let mut mapper_initial_3 = PolyDataMapper::new();
    mapper_initial_3.set_input_connection(LineSource::get_output_port(&mut line_initial_3));
    let mut actor_initial_3 = Actor::new();
    actor_initial_3.set_mapper(&mut mapper_initial_3);
    let mut prop_initial_3 = actor_initial_3.get_property();
    prop_initial_3.set_color(0.5, 0.5, 0.5); // Gray
    prop_initial_3.set_line_width(4.0);

    // Create spheres for initial nodes
    let mut sphere_initial_1 = SphereSource::new();
    sphere_initial_1.set_center([
        original_positions[0].0,
        original_positions[0].1,
        original_positions[0].2,
    ]);
    sphere_initial_1.set_radius(0.06);
    sphere_initial_1.set_theta_resolution(12);
    sphere_initial_1.set_phi_resolution(12);

    let mut sphere_initial_2 = SphereSource::new();
    sphere_initial_2.set_center([
        original_positions[1].0,
        original_positions[1].1,
        original_positions[1].2,
    ]);
    sphere_initial_2.set_radius(0.06);
    sphere_initial_2.set_theta_resolution(12);
    sphere_initial_2.set_phi_resolution(12);

    let mut sphere_initial_3 = SphereSource::new();
    sphere_initial_3.set_center([
        original_positions[2].0,
        original_positions[2].1,
        original_positions[2].2,
    ]);
    sphere_initial_3.set_radius(0.06);
    sphere_initial_3.set_theta_resolution(12);
    sphere_initial_3.set_phi_resolution(12);

    let mut sphere_initial_4 = SphereSource::new();
    sphere_initial_4.set_center([
        original_positions[3].0,
        original_positions[3].1,
        original_positions[3].2,
    ]);
    sphere_initial_4.set_radius(0.06);
    sphere_initial_4.set_theta_resolution(12);
    sphere_initial_4.set_phi_resolution(12);

    let mut node_mapper_initial_1 = PolyDataMapper::new();
    node_mapper_initial_1.set_input_connection(
        SphereSource::get_output_port(&mut sphere_initial_1)
    );
    let mut node_actor_initial_1 = Actor::new();
    node_actor_initial_1.set_mapper(&mut node_mapper_initial_1);
    let mut node_prop_initial_1 = node_actor_initial_1.get_property();
    node_prop_initial_1.set_color(0.3, 0.3, 0.3); // Dark gray

    let mut node_mapper_initial_2 = PolyDataMapper::new();
    node_mapper_initial_2.set_input_connection(
        SphereSource::get_output_port(&mut sphere_initial_2)
    );
    let mut node_actor_initial_2 = Actor::new();
    node_actor_initial_2.set_mapper(&mut node_mapper_initial_2);
    let mut node_prop_initial_2 = node_actor_initial_2.get_property();
    node_prop_initial_2.set_color(0.3, 0.3, 0.3);

    let mut node_mapper_initial_3 = PolyDataMapper::new();
    node_mapper_initial_3.set_input_connection(
        SphereSource::get_output_port(&mut sphere_initial_3)
    );
    let mut node_actor_initial_3 = Actor::new();
    node_actor_initial_3.set_mapper(&mut node_mapper_initial_3);
    let mut node_prop_initial_3 = node_actor_initial_3.get_property();
    node_prop_initial_3.set_color(0.3, 0.3, 0.3);

    let mut node_mapper_initial_4 = PolyDataMapper::new();
    node_mapper_initial_4.set_input_connection(
        SphereSource::get_output_port(&mut sphere_initial_4)
    );
    let mut node_actor_initial_4 = Actor::new();
    node_actor_initial_4.set_mapper(&mut node_mapper_initial_4);
    let mut node_prop_initial_4 = node_actor_initial_4.get_property();
    node_prop_initial_4.set_color(0.3, 0.3, 0.3);

    println!("✓ Created initial geometry actors (gray)");

    // ========== DEFORMED GEOMETRY ==========
    println!("\n=== Creating Deformed Geometry (Overlaid) ===");

    // Calculate deformed positions (NO offset - overlay on initial geometry)
    let deformed_positions: Vec<(f64, f64, f64)> = original_positions
        .iter()
        .zip(displacements.iter())
        .map(|((x, y, z), (dx, dy, dz))| { (x + dx * scale, y + dy * scale, z + dz * scale) })
        .collect();

    // Create lines for deformed beam
    let mut line1 = LineSource::new();
    line1.set_point1(deformed_positions[0].0, deformed_positions[0].1, deformed_positions[0].2);
    line1.set_point2(deformed_positions[1].0, deformed_positions[1].1, deformed_positions[1].2);
    line1.set_resolution(10);

    let mut line2 = LineSource::new();
    line2.set_point1(deformed_positions[1].0, deformed_positions[1].1, deformed_positions[1].2);
    line2.set_point2(deformed_positions[2].0, deformed_positions[2].1, deformed_positions[2].2);
    line2.set_resolution(10);

    let mut line3 = LineSource::new();
    line3.set_point1(deformed_positions[2].0, deformed_positions[2].1, deformed_positions[2].2);
    line3.set_point2(deformed_positions[3].0, deformed_positions[3].1, deformed_positions[3].2);
    line3.set_resolution(10);

    // Create mappers for each deformed beam segment
    let mut mapper1 = PolyDataMapper::new();
    mapper1.set_input_connection(LineSource::get_output_port(&mut line1));

    let mut mapper2 = PolyDataMapper::new();
    mapper2.set_input_connection(LineSource::get_output_port(&mut line2));

    let mut mapper3 = PolyDataMapper::new();
    mapper3.set_input_connection(LineSource::get_output_port(&mut line3));

    // Create actors with different colors for stress visualization
    let mut actor1 = Actor::new();
    actor1.set_mapper(&mut mapper1);
    let mut prop1 = actor1.get_property();
    prop1.set_color(1.0, 0.0, 0.0); // Red - highest stress (180 MPa)
    prop1.set_line_width(6.0);

    let mut actor2 = Actor::new();
    actor2.set_mapper(&mut mapper2);
    let mut prop2 = actor2.get_property();
    prop2.set_color(1.0, 0.5, 0.0); // Orange - medium stress (120 MPa)
    prop2.set_line_width(6.0);

    let mut actor3 = Actor::new();
    actor3.set_mapper(&mut mapper3);
    let mut prop3 = actor3.get_property();
    prop3.set_color(0.0, 1.0, 0.0); // Green - low stress (60 MPa)
    prop3.set_line_width(6.0);

    println!("✓ Created deformed beam actors with stress-based colors");

    // Create sphere markers for deformed nodes
    let mut sphere1 = SphereSource::new();
    sphere1.set_center([deformed_positions[0].0, deformed_positions[0].1, deformed_positions[0].2]);
    sphere1.set_radius(0.08);
    sphere1.set_theta_resolution(16);
    sphere1.set_phi_resolution(16);

    let mut sphere2 = SphereSource::new();
    sphere2.set_center([deformed_positions[1].0, deformed_positions[1].1, deformed_positions[1].2]);
    sphere2.set_radius(0.08);
    sphere2.set_theta_resolution(16);
    sphere2.set_phi_resolution(16);

    let mut sphere3 = SphereSource::new();
    sphere3.set_center([deformed_positions[2].0, deformed_positions[2].1, deformed_positions[2].2]);
    sphere3.set_radius(0.08);
    sphere3.set_theta_resolution(16);
    sphere3.set_phi_resolution(16);

    let mut sphere4 = SphereSource::new();
    sphere4.set_center([deformed_positions[3].0, deformed_positions[3].1, deformed_positions[3].2]);
    sphere4.set_radius(0.08);
    sphere4.set_theta_resolution(16);
    sphere4.set_phi_resolution(16);

    // Create node actors
    let mut node_mapper1 = PolyDataMapper::new();
    node_mapper1.set_input_connection(SphereSource::get_output_port(&mut sphere1));
    let mut node_actor1 = Actor::new();
    node_actor1.set_mapper(&mut node_mapper1);
    let mut node_prop1 = node_actor1.get_property();
    node_prop1.set_color(0.2, 0.2, 0.8); // Blue nodes

    let mut node_mapper2 = PolyDataMapper::new();
    node_mapper2.set_input_connection(SphereSource::get_output_port(&mut sphere2));
    let mut node_actor2 = Actor::new();
    node_actor2.set_mapper(&mut node_mapper2);
    let mut node_prop2 = node_actor2.get_property();
    node_prop2.set_color(0.2, 0.2, 0.8);

    let mut node_mapper3 = PolyDataMapper::new();
    node_mapper3.set_input_connection(SphereSource::get_output_port(&mut sphere3));
    let mut node_actor3 = Actor::new();
    node_actor3.set_mapper(&mut node_mapper3);
    let mut node_prop3 = node_actor3.get_property();
    node_prop3.set_color(0.2, 0.2, 0.8);

    let mut node_mapper4 = PolyDataMapper::new();
    node_mapper4.set_input_connection(SphereSource::get_output_port(&mut sphere4));
    let mut node_actor4 = Actor::new();
    node_actor4.set_mapper(&mut node_mapper4);
    let mut node_prop4 = node_actor4.get_property();
    node_prop4.set_color(0.2, 0.2, 0.8);

    println!("✓ Created deformed node actors with sphere geometry");

    // Setup renderer
    let mut renderer = Renderer::new();

    // Add initial geometry actors (gray)
    renderer.add_actor(&mut actor_initial_1);
    renderer.add_actor(&mut actor_initial_2);
    renderer.add_actor(&mut actor_initial_3);
    renderer.add_actor(&mut node_actor_initial_1);
    renderer.add_actor(&mut node_actor_initial_2);
    renderer.add_actor(&mut node_actor_initial_3);
    renderer.add_actor(&mut node_actor_initial_4);

    // Add deformed geometry actors (colored by stress)
    renderer.add_actor(&mut actor1);
    renderer.add_actor(&mut actor2);
    renderer.add_actor(&mut actor3);
    renderer.add_actor(&mut node_actor1);
    renderer.add_actor(&mut node_actor2);
    renderer.add_actor(&mut node_actor3);
    renderer.add_actor(&mut node_actor4);

    renderer.set_background(0.1, 0.1, 0.15); // Dark blue background
    renderer.reset_camera();

    println!("✓ Setup renderer with all actors");

    // Setup render window
    let mut render_window = RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(1024, 768);
    render_window.set_window_name(
        "FEM Beam - Overlaid Initial and Deformed (Stress Visualization)"
    );

    // Setup interactor
    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    // Use trackball camera style for better interaction
    let mut style = InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    println!("✓ Setup render window and interactor\n");

    println!("=== Visualization Info ===");
    println!("OVERLAID GEOMETRY:");
    println!("  - Gray beam (thin): Initial/undeformed configuration");
    println!("  - Colored beam (thick): Deformed configuration");
    println!("    • Red: High stress (180 MPa) - Fixed end");
    println!("    • Orange: Medium stress (120 MPa)");
    println!("    • Green: Low stress (60 MPa) - Free end");
    println!("\nNode markers:");
    println!("  - Dark gray spheres (small): Original positions");
    println!("  - Blue spheres (large): Deformed positions");
    println!("\nDisplacement:");
    println!("  - Vertical deflection (downward)");
    println!("  - Scale factor: {} (for visibility)", scale);
    println!("  - Max displacement: {:.3} units at free end", displacements[3].1 * scale);
    println!("\nInteraction:");
    println!("  - Left mouse: Rotate");
    println!("  - Middle mouse: Pan");
    println!("  - Right mouse/Scroll: Zoom");
    println!("  - 'q' or close window to exit\n");

    // Start visualization
    render_window.render();
    interactor.start();
}
