use vtk_rs as vtk;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("LineSource Demo - Distance Measurement");
    println!("═══════════════════════════════════════════════════════\n");
    println!("This demo shows three lines measuring distances");
    println!("between different points in 3D space.\n");

    // Create a sphere as a reference object
    let mut sphere_source = vtk::SphereSource::new();
    sphere_source.set_radius(2.0);
    sphere_source.set_theta_resolution(30);
    sphere_source.set_phi_resolution(30);

    let mut sphere_mapper = vtk::PolyDataMapper::new();
    sphere_mapper.set_input_connection(sphere_source.get_output_port());

    let mut sphere_actor = vtk::Actor::new();
    sphere_actor.set_mapper(&mut sphere_mapper);

    let mut sphere_prop = sphere_actor.get_property();
    sphere_prop.set_color(0.3, 0.7, 0.9);
    sphere_prop.set_opacity(0.5);

    // Create line 1: Horizontal line
    let mut line1 = vtk::LineSource::new();
    line1.set_point1(-4.0, 0.0, 0.0);
    line1.set_point2(4.0, 0.0, 0.0);
    line1.set_resolution(1);

    println!("Line 1 (Red - Horizontal): (-4, 0, 0) to (4, 0, 0)");

    let mut line1_mapper = vtk::PolyDataMapper::new();
    line1_mapper.set_input_connection(line1.get_output_port());

    let mut line1_actor = vtk::Actor::new();
    line1_actor.set_mapper(&mut line1_mapper);
    let mut line1_prop = line1_actor.get_property();
    line1_prop.set_color(1.0, 0.0, 0.0); // Red
    line1_prop.set_line_width(4.0);

    // Create line 2: Vertical line
    let mut line2 = vtk::LineSource::new();
    line2.set_point1(0.0, -3.0, 0.0);
    line2.set_point2(0.0, 3.0, 0.0);
    line2.set_resolution(1);

    println!("Line 2 (Green - Vertical): (0, -3, 0) to (0, 3, 0)");

    let mut line2_mapper = vtk::PolyDataMapper::new();
    line2_mapper.set_input_connection(line2.get_output_port());

    let mut line2_actor = vtk::Actor::new();
    line2_actor.set_mapper(&mut line2_mapper);
    let mut line2_prop = line2_actor.get_property();
    line2_prop.set_color(0.0, 1.0, 0.0); // Green
    line2_prop.set_line_width(4.0);

    // Create line 3: Diagonal line in 3D
    let mut line3 = vtk::LineSource::new();
    line3.set_point1(-2.0, -2.0, -2.0);
    line3.set_point2(2.0, 2.0, 2.0);
    line3.set_resolution(1);

    println!("Line 3 (Yellow - Diagonal 3D): (-2, -2, -2) to (2, 2, 2)\n");

    let mut line3_mapper = vtk::PolyDataMapper::new();
    line3_mapper.set_input_connection(line3.get_output_port());

    let mut line3_actor = vtk::Actor::new();
    line3_actor.set_mapper(&mut line3_mapper);
    let mut line3_prop = line3_actor.get_property();
    line3_prop.set_color(1.0, 1.0, 0.0); // Yellow
    line3_prop.set_line_width(4.0);

    // Create renderer
    let mut renderer = vtk::Renderer::new();
    renderer.add_actor(&mut sphere_actor);
    renderer.add_actor(&mut line1_actor);
    renderer.add_actor(&mut line2_actor);
    renderer.add_actor(&mut line3_actor);
    renderer.set_background(0.1, 0.1, 0.1);
    renderer.reset_camera();

    // Create render window
    let mut render_window = vtk::RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(800, 600);
    render_window.set_window_name("LineSource Demo - Distance Measurement");

    // Create interactor
    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    // Initialize and start
    interactor.initialize();
    render_window.render();

    println!("═══════════════════════════════════════════════════════");
    println!("Window opened. Use mouse to interact:");
    println!("  - Left button: Rotate");
    println!("  - Middle button: Pan");
    println!("  - Right button/Scroll: Zoom");
    println!("═══════════════════════════════════════════════════════\n");

    interactor.start();
}
