use vtk_rs::*;

fn main() {
    // Create render window, renderer, and interactor
    let mut render_window = RenderWindow::new();
    render_window.set_size(1200, 800);
    render_window.set_window_name("VTK ConeSource Demo");

    let mut renderer = Renderer::new();
    render_window.add_renderer(&mut renderer);
    renderer.set_background(0.1, 0.1, 0.2);

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    // Setup trackball camera interaction style for smooth rotation
    let mut style = InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    // 1. Traffic cone - wide base, orange
    let mut cone1 = ConeSource::new();
    cone1.set_radius(0.5);
    cone1.set_height(1.5);
    cone1.set_resolution(32);
    cone1.set_center(-3.0, 0.0, 0.0);
    cone1.set_direction(0.0, 1.0, 0.0); // Point upward
    cone1.set_capping(true); // Close the base

    let mut mapper1 = PolyDataMapper::new();
    mapper1.set_input_connection(cone1.get_output_port());

    let mut actor1 = Actor::new();
    actor1.set_mapper(&mut mapper1);
    let mut prop1 = actor1.get_property();
    prop1.set_color(1.0, 0.5, 0.0); // Orange
    renderer.add_actor(&mut actor1);

    // 2. Arrow - high resolution, pointing right
    let mut cone2 = ConeSource::new();
    cone2.set_radius(0.3);
    cone2.set_height(1.0);
    cone2.set_resolution(48);
    cone2.set_center(0.0, 0.0, 0.0);
    cone2.set_direction(1.0, 0.0, 0.0); // Point right
    cone2.set_capping(true);

    let mut mapper2 = PolyDataMapper::new();
    mapper2.set_input_connection(cone2.get_output_port());

    let mut actor2 = Actor::new();
    actor2.set_mapper(&mut mapper2);
    let mut prop2 = actor2.get_property();
    prop2.set_color(0.0, 1.0, 1.0); // Cyan
    renderer.add_actor(&mut actor2);

    // 3. Spike - tall and thin, no capping
    let mut cone3 = ConeSource::new();
    cone3.set_radius(0.2);
    cone3.set_height(2.0);
    cone3.set_resolution(24);
    cone3.set_center(3.0, 0.0, 0.0);
    cone3.set_direction(0.0, 1.0, 0.5); // Tilted
    cone3.set_capping(false); // Leave base open

    let mut mapper3 = PolyDataMapper::new();
    mapper3.set_input_connection(cone3.get_output_port());

    let mut actor3 = Actor::new();
    actor3.set_mapper(&mut mapper3);
    let mut prop3 = actor3.get_property();
    prop3.set_color(1.0, 0.0, 1.0); // Magenta
    renderer.add_actor(&mut actor3);

    // 4. Low-res cone - show faceted geometry
    let mut cone4 = ConeSource::new();
    cone4.set_radius(0.4);
    cone4.set_height(1.2);
    cone4.set_resolution(8); // Very low resolution
    cone4.set_center(-3.0, -2.0, 0.0);
    cone4.set_direction(0.0, 1.0, 0.0);
    cone4.set_capping(true);

    let mut mapper4 = PolyDataMapper::new();
    mapper4.set_input_connection(cone4.get_output_port());

    let mut actor4 = Actor::new();
    actor4.set_mapper(&mut mapper4);
    let mut prop4 = actor4.get_property();
    prop4.set_color(1.0, 1.0, 0.0); // Yellow
    renderer.add_actor(&mut actor4);

    // 5. Wide angle cone
    let mut cone5 = ConeSource::new();
    cone5.set_angle(60.0); // Set cone angle directly
    cone5.set_height(1.0);
    cone5.set_resolution(36);
    cone5.set_center(0.0, -2.0, 0.0);
    cone5.set_direction(0.0, 1.0, 0.0);
    cone5.set_capping(true);

    let mut mapper5 = PolyDataMapper::new();
    mapper5.set_input_connection(cone5.get_output_port());

    let mut actor5 = Actor::new();
    actor5.set_mapper(&mut mapper5);
    let mut prop5 = actor5.get_property();
    prop5.set_color(0.5, 1.0, 0.0); // Lime green
    renderer.add_actor(&mut actor5);

    // 6. Inverted cone (direction pointing down)
    let mut cone6 = ConeSource::new();
    cone6.set_radius(0.35);
    cone6.set_height(1.3);
    cone6.set_resolution(32);
    cone6.set_center(3.0, -2.0, 0.0);
    cone6.set_direction(0.0, -1.0, 0.0); // Point down
    cone6.set_capping(true);

    let mut mapper6 = PolyDataMapper::new();
    mapper6.set_input_connection(cone6.get_output_port());

    let mut actor6 = Actor::new();
    actor6.set_mapper(&mut mapper6);
    let mut prop6 = actor6.get_property();
    prop6.set_color(1.0, 0.0, 0.0); // Red
    renderer.add_actor(&mut actor6);

    // Add text information
    println!("VTK ConeSource Demo");
    println!("==================");
    println!("Top row (left to right):");
    println!("  1. Traffic cone - Orange, radius=0.5, height=1.5, res=32, capped");
    println!("  2. Arrow - Cyan, radius=0.3, height=1.0, res=48, pointing right");
    println!("  3. Spike - Magenta, radius=0.2, height=2.0, res=24, NOT capped, tilted");
    println!("\nBottom row (left to right):");
    println!("  4. Low-res cone - Yellow, res=8 (faceted geometry)");
    println!("  5. Wide angle cone - Green, angle=60°");
    println!("  6. Inverted cone - Red, pointing down");
    println!("\nInteraction: Rotate with mouse, zoom with scroll wheel");

    // Reset camera and render
    renderer.reset_camera();
    render_window.render();
    interactor.start();
}
