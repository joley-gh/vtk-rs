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

    // Create axes actor for orientation reference
    let mut axes = vtk::AxesActor::new();
    let mut axes_widget = vtk::OrientationMarkerWidget::new();
    axes_widget.set_orientation_marker(&mut axes);
    axes_widget.set_viewport(0.0, 0.0, 0.2, 0.2);
    axes_widget.interactive_off();

    // Create render window and interactor
    let mut render_window = vtk::RenderWindow::new();
    render_window.set_window_name("Animated Camera Control Example");
    render_window.set_size(800, 600);
    render_window.add_renderer(&mut renderer);

    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);
    axes_widget.set_interactor(&mut interactor);
    axes_widget.set_enabled(true);

    let mut style = vtk::InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    // Get the camera and configure it manually
    let mut camera = renderer.get_active_camera();

    // Set initial camera position - looking at sphere from an angle
    camera.set_position(15.0, 15.0, 15.0);
    camera.set_focal_point(0.0, 0.0, 0.0);
    camera.set_view_up(0.0, 0.0, 1.0);

    println!("Initial camera position: (15.00, 15.00, 15.00)");
    println!("Starting animated camera transformations...\n");
    println!("Note: Animation happens before interactive mode starts.\n");

    // Initial render
    render_window.render();

    // Perform animated transformations
    println!("Step 1: Rotating azimuth (360° in small steps)...");
    for _ in 0..72 {
        let mut camera = renderer.get_active_camera();
        camera.azimuth(5.0);
        render_window.render();
        thread::sleep(Duration::from_millis(30));
    }

    thread::sleep(Duration::from_millis(500));

    println!("Step 2: Changing elevation (up and down)...");
    for _ in 0..30 {
        let mut camera = renderer.get_active_camera();
        camera.elevation(2.0);
        render_window.render();
        thread::sleep(Duration::from_millis(30));
    }
    for _ in 0..30 {
        let mut camera = renderer.get_active_camera();
        camera.elevation(-2.0);
        render_window.render();
        thread::sleep(Duration::from_millis(30));
    }

    thread::sleep(Duration::from_millis(500));

    println!("Step 3: Zooming in...");
    for _ in 0..15 {
        let mut camera = renderer.get_active_camera();
        camera.zoom(1.05);
        render_window.render();
        thread::sleep(Duration::from_millis(60));
    }

    thread::sleep(Duration::from_millis(500));

    println!("Step 4: Zooming out...");
    for _ in 0..15 {
        let mut camera = renderer.get_active_camera();
        camera.zoom(0.95);
        render_window.render();
        thread::sleep(Duration::from_millis(60));
    }

    thread::sleep(Duration::from_millis(500));

    println!("Step 5: Rolling the camera (360°)...");
    for _ in 0..72 {
        let mut camera = renderer.get_active_camera();
        camera.roll(5.0);
        render_window.render();
        thread::sleep(Duration::from_millis(30));
    }

    thread::sleep(Duration::from_millis(500));

    println!("Step 6: Moving camera position smoothly...");
    for i in 0..40 {
        let t = (i as f64) / 39.0;
        let mut camera = renderer.get_active_camera();
        // Interpolate from (15, 15, 15) to (20, 5, 10)
        let x = 15.0 * (1.0 - t) + 20.0 * t;
        let y = 15.0 * (1.0 - t) + 5.0 * t;
        let z = 15.0 * (1.0 - t) + 10.0 * t;
        camera.set_position(x, y, z);
        render_window.render();
        thread::sleep(Duration::from_millis(50));
    }

    thread::sleep(Duration::from_millis(500));

    println!("Step 7: Dolly (move camera closer/farther)...");
    for _ in 0..20 {
        let mut camera = renderer.get_active_camera();
        camera.dolly(1.05);
        render_window.render();
        thread::sleep(Duration::from_millis(50));
    }
    for _ in 0..20 {
        let mut camera = renderer.get_active_camera();
        camera.dolly(0.95);
        render_window.render();
        thread::sleep(Duration::from_millis(50));
    }

    println!("\nAnimation complete! Now entering interactive mode.");
    println!("You can interact with the scene using the mouse:");
    println!("  - Left mouse button: Rotate");
    println!("  - Middle mouse button: Pan");
    println!("  - Right mouse button: Zoom");
    println!("  - Scroll wheel: Zoom\n");

    // Start interactive mode
    interactor.initialize();
    interactor.start();
}
