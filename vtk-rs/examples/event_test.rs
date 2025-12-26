use vtk_rs as vtk;

// Test direct FFI call
extern "C" {
    fn vtk_test_link_interactor_style() -> i32;
}

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("Interactive Picking Test");
    println!("═══════════════════════════════════════════════════════\n");

    // Test direct C call
    println!("Testing direct C call...");
    let test_result = unsafe { vtk_test_link_interactor_style() };
    println!("Test result: {}", test_result);

    // Create a simple sphere
    let mut sphere_source = vtk::SphereSource::new();
    sphere_source.set_radius(5.0);

    // Create mapper
    let mut mapper = vtk::PolyDataMapper::new();
    mapper.set_input_connection(sphere_source.get_output_port());

    // Create actor
    let mut actor = vtk::Actor::new();
    actor.set_mapper(&mut mapper);

    // Create renderer
    let mut renderer = vtk::Renderer::new();
    renderer.add_actor(&mut actor);
    renderer.set_background(0.1, 0.1, 0.1);

    // Create render window
    let mut render_window = vtk::RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(800, 600);

    // Create interactor
    let mut interactor = vtk::RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    println!("Creating custom interactor style...");
    eprintln!("DEBUG: About to call InteractorStyleCustom::new()");

    // Create custom style
    let mut style = vtk::InteractorStyleCustom::new();

    eprintln!("DEBUG: InteractorStyleCustom::new() succeeded");
    println!("Setting up callback...");

    // Set up callback
    style.set_left_button_press_callback(|x, y| {
        println!("CLICK! Position: ({}, {})", x, y);
    });

    println!("Setting interactor style...");
    interactor.set_interactor_style_custom(&mut style);

    println!("Initializing interactor...");
    interactor.initialize();

    println!("Rendering window...");
    render_window.render();

    println!("Window opened. Click to test event handling.\n");
    println!("Starting event loop...");

    interactor.start();

    println!("Event loop ended.");
}
