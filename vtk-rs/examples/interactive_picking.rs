use vtk_rs as vtk;
use std::sync::{ Arc, Mutex };

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("Interactive Picking Example with Cell Picker");
    println!("═══════════════════════════════════════════════════════\n");
    println!("Instructions:");
    println!("  - Left-click on the sphere to pick it");
    println!("  - Click outside to see background detection");
    println!("  - Picked cell ID and world position will be displayed");
    println!("  - Right mouse button: Rotate view");
    println!("  - Middle mouse button: Pan");
    println!("  - Scroll: Zoom\n");
    println!("═══════════════════════════════════════════════════════\n");

    // Create a sphere
    let mut sphere_source = vtk::SphereSource::new();
    sphere_source.set_radius(5.0);
    sphere_source.set_theta_resolution(30);
    sphere_source.set_phi_resolution(30);

    // Create mapper
    let mut mapper = vtk::PolyDataMapper::new();
    mapper.set_input_connection(sphere_source.get_output_port());

    // Create actor
    let mut actor = vtk::Actor::new();
    actor.set_mapper(&mut mapper);

    // Set nice appearance
    let mut prop = actor.get_property();
    prop.set_color(0.3, 0.7, 0.9); // Nice blue color
    prop.set_representation(vtk::RepresentationType::Surface);

    // Create renderer
    let mut renderer = vtk::Renderer::new();
    renderer.add_actor(&mut actor);
    renderer.set_background(0.1, 0.1, 0.1);
    renderer.reset_camera();

    // Share picker and renderer with callback using Arc<Mutex<>>
    // Must create Arc BEFORE adding renderer to window
    let picker = Arc::new(Mutex::new(vtk::CellPicker::new()));
    let renderer_shared = Arc::new(Mutex::new(renderer));

    let picker_clone = Arc::clone(&picker);
    let renderer_clone = Arc::clone(&renderer_shared);

    // Create render window
    let mut render_window = vtk::RenderWindow::new();
    // Extract renderer temporarily to add to window
    {
        let mut renderer = renderer_shared.lock().unwrap();
        render_window.add_renderer(&mut renderer);
    }
    render_window.set_size(800, 600);
    render_window.set_window_name("Interactive Picking - Click to Pick!");

    // Create interactor
    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    // Create custom interactor style with picking callback
    let mut style = vtk::InteractorStyleCustom::new();

    // Track click count
    let click_count = Arc::new(Mutex::new(0));
    let click_count_clone = Arc::clone(&click_count);

    style.set_left_button_press_callback(move |x, y| {
        let mut count = click_count_clone.lock().unwrap();
        *count += 1;

        println!("\n─────────────────────────────────────────────────────");
        println!("🖱️  Click #{}: Display position ({}, {})", *count, x, y);

        // Perform picking
        let mut picker = picker_clone.lock().unwrap();
        let mut renderer = renderer_clone.lock().unwrap();

        let pick_success = picker.pick(x as f64, y as f64, 0.0, &mut renderer);

        if pick_success {
            println!("✅ HIT! Sphere was picked!");

            // Get world position of the pick
            let pos = picker.get_pick_position();
            println!("   📍 World Position: ({:.3}, {:.3}, {:.3})", pos.0, pos.1, pos.2);

            // Get cell ID
            let cell_id = picker.get_cell_id();
            println!("   🔢 Cell ID: {}", cell_id);
        } else {
            println!("❌ MISS - Clicked on background (no geometry picked)");
        }

        println!("─────────────────────────────────────────────────────");
    });

    interactor.set_interactor_style_custom(&mut style);

    // Initialize and start
    interactor.initialize();
    render_window.render();

    println!("Window opened. Try clicking ON the sphere and OFF the sphere!\n");

    interactor.start();
}
