use vtk_rs::*;

fn main() {
    println!("=== FEM Beam with Tube Visualization ===\n");
    println!("Cantilever beam with 3D tube rendering:");
    println!("  - 4 nodes, 3 beam elements");
    println!("  - TubeFilter for realistic 3D beam appearance");
    println!("  - Color-mapped by stress\n");

    // Original node positions
    let original_positions = [
        (0.0, 0.0, 0.0),  // Node 0: Fixed end
        (1.0, 0.0, 0.0),  // Node 1
        (2.0, 0.0, 0.0),  // Node 2
        (3.0, 0.0, 0.0),  // Node 3: Free end
    ];

    // Displacement vectors (dy component - vertical deflection)
    let displacements = [
        (0.0, 0.0, 0.0),    // Node 0: Fixed (no displacement)
        (0.0, 0.1, 0.0),    // Node 1: 0.1 units down
        (0.0, 0.5, 0.0),    // Node 2: 0.5 units down
        (0.0, 1.0, 0.0),    // Node 3: 1.0 units down (max)
    ];

    // Displacement scale factor for visualization
    let scale = 0.1;

    // Calculate deformed positions (overlay on initial geometry)
    let deformed_positions: Vec<(f64, f64, f64)> = original_positions
        .iter()
        .zip(displacements.iter())
        .map(|((x, y, z), (dx, dy, dz))| (x + dx * scale, y + dy * scale, z + dz * scale))
        .collect();

    println!("✓ Creating geometry with deformed positions");

    // ========== INITIAL GEOMETRY (GRAY LINES) ==========
    println!("\n=== Creating Initial Geometry ===");

    let mut line_initial_1 = LineSource::new();
    line_initial_1.set_point1(original_positions[0].0, original_positions[0].1, original_positions[0].2);
    line_initial_1.set_point2(original_positions[1].0, original_positions[1].1, original_positions[1].2);

    let mut line_initial_2 = LineSource::new();
    line_initial_2.set_point1(original_positions[1].0, original_positions[1].1, original_positions[1].2);
    line_initial_2.set_point2(original_positions[2].0, original_positions[2].1, original_positions[2].2);

    let mut line_initial_3 = LineSource::new();
    line_initial_3.set_point1(original_positions[2].0, original_positions[2].1, original_positions[2].2);
    line_initial_3.set_point2(original_positions[3].0, original_positions[3].1, original_positions[3].2);

    // Simple line rendering for initial geometry
    let mut mapper_initial_1 = PolyDataMapper::new();
    mapper_initial_1.set_input_connection(LineSource::get_output_port(&mut line_initial_1));
    let mut actor_initial_1 = Actor::new();
    actor_initial_1.set_mapper(&mut mapper_initial_1);
    let mut prop_initial_1 = actor_initial_1.get_property();
    prop_initial_1.set_color(0.5, 0.5, 0.5);
    prop_initial_1.set_line_width(2.0);

    let mut mapper_initial_2 = PolyDataMapper::new();
    mapper_initial_2.set_input_connection(LineSource::get_output_port(&mut line_initial_2));
    let mut actor_initial_2 = Actor::new();
    actor_initial_2.set_mapper(&mut mapper_initial_2);
    let mut prop_initial_2 = actor_initial_2.get_property();
    prop_initial_2.set_color(0.5, 0.5, 0.5);
    prop_initial_2.set_line_width(2.0);

    let mut mapper_initial_3 = PolyDataMapper::new();
    mapper_initial_3.set_input_connection(LineSource::get_output_port(&mut line_initial_3));
    let mut actor_initial_3 = Actor::new();
    actor_initial_3.set_mapper(&mut mapper_initial_3);
    let mut prop_initial_3 = actor_initial_3.get_property();
    prop_initial_3.set_color(0.5, 0.5, 0.5);
    prop_initial_3.set_line_width(2.0);

    println!("✓ Created initial geometry (thin gray lines)");

    // ========== DEFORMED GEOMETRY WITH TUBES ==========
    println!("\n=== Creating Deformed Geometry with TubeFilter ===");

    // Create line sources for deformed beam
    let mut line1 = LineSource::new();
    line1.set_point1(deformed_positions[0].0, deformed_positions[0].1, deformed_positions[0].2);
    line1.set_point2(deformed_positions[1].0, deformed_positions[1].1, deformed_positions[1].2);

    let mut line2 = LineSource::new();
    line2.set_point1(deformed_positions[1].0, deformed_positions[1].1, deformed_positions[1].2);
    line2.set_point2(deformed_positions[2].0, deformed_positions[2].1, deformed_positions[2].2);

    let mut line3 = LineSource::new();
    line3.set_point1(deformed_positions[2].0, deformed_positions[2].1, deformed_positions[2].2);
    line3.set_point2(deformed_positions[3].0, deformed_positions[3].1, deformed_positions[3].2);

    // Create tube filters
    let mut tube1 = TubeFilter::new();
    tube1.set_input_connection(LineSource::get_output_port(&mut line1));
    tube1.set_radius(0.05);
    tube1.set_number_of_sides(12);
    tube1.set_capping(true);

    let mut tube2 = TubeFilter::new();
    tube2.set_input_connection(LineSource::get_output_port(&mut line2));
    tube2.set_radius(0.05);
    tube2.set_number_of_sides(12);
    tube2.set_capping(true);

    let mut tube3 = TubeFilter::new();
    tube3.set_input_connection(LineSource::get_output_port(&mut line3));
    tube3.set_radius(0.05);
    tube3.set_number_of_sides(12);
    tube3.set_capping(true);

    // Create mappers and actors with tube input
    let mut mapper1 = PolyDataMapper::new();
    mapper1.set_input_connection(TubeFilter::get_output_port(&mut tube1));
    let mut actor1 = Actor::new();
    actor1.set_mapper(&mut mapper1);
    let mut prop1 = actor1.get_property();
    prop1.set_color(1.0, 0.0, 0.0);  // Red - highest stress (180 MPa)

    let mut mapper2 = PolyDataMapper::new();
    mapper2.set_input_connection(TubeFilter::get_output_port(&mut tube2));
    let mut actor2 = Actor::new();
    actor2.set_mapper(&mut mapper2);
    let mut prop2 = actor2.get_property();
    prop2.set_color(1.0, 0.5, 0.0);  // Orange - medium stress (120 MPa)

    let mut mapper3 = PolyDataMapper::new();
    mapper3.set_input_connection(TubeFilter::get_output_port(&mut tube3));
    let mut actor3 = Actor::new();
    actor3.set_mapper(&mut mapper3);
    let mut prop3 = actor3.get_property();
    prop3.set_color(0.0, 1.0, 0.0);  // Green - low stress (60 MPa)

    println!("✓ Created tube actors with stress-based colors");

    // Create sphere markers for nodes
    let mut sphere_markers = Vec::new();
    for (i, pos) in deformed_positions.iter().enumerate() {
        let mut sphere = SphereSource::new();
        sphere.set_center([pos.0, pos.1, pos.2]);
        sphere.set_radius(0.06);
        sphere.set_theta_resolution(16);
        sphere.set_phi_resolution(16);

        let mut mapper = PolyDataMapper::new();
        mapper.set_input_connection(SphereSource::get_output_port(&mut sphere));
        let mut actor = Actor::new();
        actor.set_mapper(&mut mapper);
        let mut prop = actor.get_property();
        prop.set_color(0.2, 0.2, 0.8);  // Blue nodes

        sphere_markers.push((sphere, mapper, actor));
    }

    println!("✓ Created node markers");

    // Setup renderer
    let mut renderer = Renderer::new();
    
    // Add initial geometry (gray lines)
    renderer.add_actor(&mut actor_initial_1);
    renderer.add_actor(&mut actor_initial_2);
    renderer.add_actor(&mut actor_initial_3);
    
    // Add deformed geometry (colored tubes)
    renderer.add_actor(&mut actor1);
    renderer.add_actor(&mut actor2);
    renderer.add_actor(&mut actor3);
    
    // Add node markers
    for (_, _, ref mut actor) in &mut sphere_markers {
        renderer.add_actor(actor);
    }
    
    renderer.set_background(0.1, 0.1, 0.15);
    renderer.reset_camera();

    println!("✓ Setup renderer");

    // Setup render window
    let mut render_window = RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(1024, 768);
    render_window.set_window_name("FEM Beam - TubeFilter Visualization");

    // Setup interactor
    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);
    
    let mut style = InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    println!("✓ Setup render window and interactor\n");

    println!("=== Visualization Info ===");
    println!("OVERLAID GEOMETRY:");
    println!("  - Gray lines: Initial/undeformed configuration");
    println!("  - Colored tubes: Deformed configuration (3D beams)");
    println!("    • Red: High stress (180 MPa) - Fixed end");
    println!("    • Orange: Medium stress (120 MPa)");
    println!("    • Green: Low stress (60 MPa) - Free end");
    println!("  - Blue spheres: Node positions");
    println!("\nTube properties:");
    println!("  - Radius: 0.05 units");
    println!("  - Sides: 12 (smooth circular cross-section)");
    println!("  - Capped ends for solid appearance");
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
