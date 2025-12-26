use vtk_rs as vtk;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a sphere
    let mut sphere_source = vtk::SphereSource::new();
    sphere_source.set_center([0.0; 3]);
    sphere_source.set_radius(5.0);
    sphere_source.set_phi_resolution(100);
    sphere_source.set_theta_resolution(100);

    // Create the mapper and actor
    let mut mapper = vtk::PolyDataMapper::new();
    mapper.set_input_connection(sphere_source.get_output_port());
    let mut actor = vtk::Actor::new();
    actor.set_mapper(&mut mapper);

    // Create the renderer and add actor
    let mut renderer = vtk::Renderer::new();
    renderer.set_background(0.1, 0.2, 0.4);
    renderer.add_actor(&mut actor);

    // Setup camera
    let mut camera = renderer.get_active_camera();
    camera.set_position(15.0, 15.0, 15.0);
    camera.set_focal_point(0.0, 0.0, 0.0);
    camera.set_view_up(0.0, 0.0, 1.0);

    // Create axes actor for orientation reference
    let mut axes = vtk::AxesActor::new();
    let mut axes_widget = vtk::OrientationMarkerWidget::new();
    axes_widget.set_orientation_marker(&mut axes);
    axes_widget.set_viewport(0.0, 0.0, 0.2, 0.2);
    axes_widget.interactive_off();

    // Create render window and interactor
    let mut render_window = vtk::RenderWindow::new();
    render_window.set_window_name("Property Animation Example");
    render_window.set_size(800, 600);
    render_window.add_renderer(&mut renderer);

    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);
    axes_widget.set_interactor(&mut interactor);
    axes_widget.set_enabled(true);

    let mut style = vtk::InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    println!("Property Animation Example");
    println!("Demonstrating various visual properties...\n");

    // Initial render
    render_window.render();
    thread::sleep(Duration::from_secs(1));

    // Get the property
    let mut property = actor.get_property();

    // Step 1: Change colors
    println!("Step 1: Animating colors (red → green → blue)...");
    for i in 0..60 {
        let t = (i as f64) / 59.0;
        let mut property = actor.get_property();
        if t < 0.33 {
            // Red to Green
            let local_t = t / 0.33;
            property.set_color(1.0 - local_t, local_t, 0.0);
        } else if t < 0.66 {
            // Green to Blue
            let local_t = (t - 0.33) / 0.33;
            property.set_color(0.0, 1.0 - local_t, local_t);
        } else {
            // Blue to Red
            let local_t = (t - 0.66) / 0.34;
            property.set_color(local_t, 0.0, 1.0 - local_t);
        }
        render_window.render();
        thread::sleep(Duration::from_millis(50));
    }

    thread::sleep(Duration::from_millis(500));

    // Step 2: Change opacity
    println!("Step 2: Changing opacity (opaque → transparent → opaque)...");
    property.set_color(0.3, 0.7, 0.9); // Nice blue color
    for i in 0..40 {
        let t = (i as f64) / 39.0;
        let mut property = actor.get_property();
        let opacity = if t < 0.5 {
            1.0 - t * 2.0 * 0.7 // Go from 1.0 to 0.3
        } else {
            0.3 + (t - 0.5) * 2.0 * 0.7 // Go from 0.3 to 1.0
        };
        property.set_opacity(opacity);
        render_window.render();
        thread::sleep(Duration::from_millis(50));
    }

    thread::sleep(Duration::from_millis(500));

    // Step 3: Change representation
    println!("Step 3: Surface representation with edges...");
    property.set_opacity(1.0);
    property.set_color(0.8, 0.3, 0.3); // Red
    property.set_representation(vtk::RepresentationType::Surface);
    property.set_edge_visibility(true);
    render_window.render();
    thread::sleep(Duration::from_secs(2));

    println!("Step 4: Wireframe representation...");
    property.set_color(0.3, 0.8, 0.3); // Green
    property.set_representation(vtk::RepresentationType::Wireframe);
    property.set_line_width(2.0);
    render_window.render();
    thread::sleep(Duration::from_secs(2));

    println!("Step 5: Points representation...");
    property.set_color(0.3, 0.3, 0.8); // Blue
    property.set_representation(vtk::RepresentationType::Points);
    property.set_point_size(5.0);
    render_window.render();
    thread::sleep(Duration::from_secs(2));

    // Step 6: Back to surface with animated line width on edges
    println!("Step 6: Animating edge thickness...");
    property.set_representation(vtk::RepresentationType::Surface);
    property.set_color(0.9, 0.9, 0.2); // Yellow
    property.set_edge_visibility(true);
    for i in 0..30 {
        let t = (i as f64) / 29.0;
        let mut property = actor.get_property();
        let width = 1.0 + t * 5.0; // Go from 1.0 to 6.0
        property.set_line_width(width);
        render_window.render();
        thread::sleep(Duration::from_millis(50));
    }
    for i in 0..30 {
        let t = (i as f64) / 29.0;
        let mut property = actor.get_property();
        let width = 6.0 - t * 5.0; // Go from 6.0 to 1.0
        property.set_line_width(width);
        render_window.render();
        thread::sleep(Duration::from_millis(50));
    }

    thread::sleep(Duration::from_millis(500));

    // Final state: Nice looking sphere
    println!("Step 7: Final appearance - smooth shaded sphere...");
    property.set_representation(vtk::RepresentationType::Surface);
    property.set_edge_visibility(false);
    property.set_color(0.4, 0.6, 0.9);
    property.set_opacity(1.0);
    render_window.render();

    println!("\nAnimation complete! Now entering interactive mode.");
    println!("You can interact with the scene using the mouse:");
    println!("  - Left mouse button: Rotate");
    println!("  - Middle mouse button: Pan");
    println!("  - Right mouse button: Zoom\n");

    // Start interactive mode
    interactor.initialize();
    interactor.start();
}
