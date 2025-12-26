use vtk_rs::*;

fn main() {
    // Create render window, renderer, and interactor
    let mut render_window = RenderWindow::new();
    render_window.set_size(1200, 800);
    render_window.set_window_name("VTK CylinderSource Demo");

    let mut renderer = Renderer::new();
    render_window.add_renderer(&mut renderer);
    renderer.set_background(0.1, 0.1, 0.2);

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    // Setup trackball camera interaction style for smooth rotation
    let mut style = InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    // 1. Vertical pipe - default orientation (along Y-axis)
    let mut cyl1 = CylinderSource::new();
    cyl1.set_radius(0.3);
    cyl1.set_height(2.0);
    cyl1.set_resolution(32);
    cyl1.set_center(-3.0, 0.0, 0.0);
    cyl1.set_capping(true);

    let mut mapper1 = PolyDataMapper::new();
    mapper1.set_input_connection(cyl1.get_output_port());

    let mut actor1 = Actor::new();
    actor1.set_mapper(&mut mapper1);
    let mut prop1 = actor1.get_property();
    prop1.set_color(0.8, 0.3, 0.1); // Bronze
    renderer.add_actor(&mut actor1);

    // 2. Wide column - thick and short
    let mut cyl2 = CylinderSource::new();
    cyl2.set_radius(0.6);
    cyl2.set_height(1.0);
    cyl2.set_resolution(48);
    cyl2.set_center(0.0, 0.0, 0.0);
    cyl2.set_capping(true);

    let mut mapper2 = PolyDataMapper::new();
    mapper2.set_input_connection(cyl2.get_output_port());

    let mut actor2 = Actor::new();
    actor2.set_mapper(&mut mapper2);
    let mut prop2 = actor2.get_property();
    prop2.set_color(0.7, 0.7, 0.7); // Gray stone
    renderer.add_actor(&mut actor2);

    // 3. Thin rod - uncapped ends
    let mut cyl3 = CylinderSource::new();
    cyl3.set_radius(0.1);
    cyl3.set_height(2.5);
    cyl3.set_resolution(24);
    cyl3.set_center(3.0, 0.0, 0.0);
    cyl3.set_capping(false); // Open ends

    let mut mapper3 = PolyDataMapper::new();
    mapper3.set_input_connection(cyl3.get_output_port());

    let mut actor3 = Actor::new();
    actor3.set_mapper(&mut mapper3);
    let mut prop3 = actor3.get_property();
    prop3.set_color(0.9, 0.1, 0.1); // Red
    renderer.add_actor(&mut actor3);

    // 4. Low-resolution cylinder - show faceting
    let mut cyl4 = CylinderSource::new();
    cyl4.set_radius(0.35);
    cyl4.set_height(1.5);
    cyl4.set_resolution(6); // Hexagonal
    cyl4.set_center(-3.0, -2.0, 0.0);
    cyl4.set_capping(true);

    let mut mapper4 = PolyDataMapper::new();
    mapper4.set_input_connection(cyl4.get_output_port());

    let mut actor4 = Actor::new();
    actor4.set_mapper(&mut mapper4);
    let mut prop4 = actor4.get_property();
    prop4.set_color(1.0, 0.8, 0.0); // Gold
    renderer.add_actor(&mut actor4);

    // 5. Thin tube - high resolution
    let mut cyl5 = CylinderSource::new();
    cyl5.set_radius(0.25);
    cyl5.set_height(1.8);
    cyl5.set_resolution(64);
    cyl5.set_center(0.0, -2.0, 0.0);
    cyl5.set_capping(true);

    let mut mapper5 = PolyDataMapper::new();
    mapper5.set_input_connection(cyl5.get_output_port());

    let mut actor5 = Actor::new();
    actor5.set_mapper(&mut mapper5);
    let mut prop5 = actor5.get_property();
    prop5.set_color(0.2, 0.8, 0.2); // Green
    renderer.add_actor(&mut actor5);

    // 6. Short disk-like cylinder
    let mut cyl6 = CylinderSource::new();
    cyl6.set_radius(0.5);
    cyl6.set_height(0.3);
    cyl6.set_resolution(32);
    cyl6.set_center(3.0, -2.0, 0.0);
    cyl6.set_capping(true);

    let mut mapper6 = PolyDataMapper::new();
    mapper6.set_input_connection(cyl6.get_output_port());

    let mut actor6 = Actor::new();
    actor6.set_mapper(&mut mapper6);
    let mut prop6 = actor6.get_property();
    prop6.set_color(0.3, 0.5, 1.0); // Blue
    renderer.add_actor(&mut actor6);

    // Add text information
    println!("VTK CylinderSource Demo");
    println!("=======================");
    println!("Top row (left to right):");
    println!("  1. Vertical pipe - Bronze, r=0.3, h=2.0, res=32, capped");
    println!("  2. Wide column - Gray, r=0.6, h=1.0, res=48, thick column");
    println!("  3. Thin rod - Red, r=0.1, h=2.5, res=24, NOT capped (open ends)");
    println!("\nBottom row (left to right):");
    println!("  4. Low-res cylinder - Gold, res=6 (hexagonal)");
    println!("  5. Smooth tube - Green, res=64 (high resolution)");
    println!("  6. Disk-like - Blue, r=0.5, h=0.3 (very short)");
    println!("\nNote: All cylinders are oriented along Y-axis (vertical)");
    println!("Interaction: Left-drag to rotate, right-drag to zoom, scroll wheel to zoom");

    // Reset camera and render
    renderer.reset_camera();
    render_window.render();
    interactor.start();
}
