use vtk_rs::*;

fn main() {
    // Create render window, renderer, and interactor
    let mut render_window = RenderWindow::new();
    render_window.set_size(1200, 800);
    render_window.set_window_name("VTK CubeSource Demo");

    let mut renderer = Renderer::new();
    render_window.add_renderer(&mut renderer);
    renderer.set_background(0.1, 0.1, 0.2);

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    // Setup trackball camera interaction style
    let mut style = InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    // 1. Unit cube - default 1x1x1
    let mut cube1 = CubeSource::new();
    cube1.set_center(-3.0, 0.0, 0.0);

    let mut mapper1 = PolyDataMapper::new();
    mapper1.set_input_connection(cube1.get_output_port());

    let mut actor1 = Actor::new();
    actor1.set_mapper(&mut mapper1);
    let mut prop1 = actor1.get_property();
    prop1.set_color(0.8, 0.2, 0.2); // Red
    renderer.add_actor(&mut actor1);

    // 2. Tall box - stretched in Y
    let mut cube2 = CubeSource::new();
    cube2.set_x_length(0.8);
    cube2.set_y_length(2.5);
    cube2.set_z_length(0.8);
    cube2.set_center(0.0, 0.0, 0.0);

    let mut mapper2 = PolyDataMapper::new();
    mapper2.set_input_connection(cube2.get_output_port());

    let mut actor2 = Actor::new();
    actor2.set_mapper(&mut mapper2);
    let mut prop2 = actor2.get_property();
    prop2.set_color(0.2, 0.8, 0.2); // Green
    renderer.add_actor(&mut actor2);

    // 3. Flat platform - stretched in XZ plane
    let mut cube3 = CubeSource::new();
    cube3.set_x_length(2.0);
    cube3.set_y_length(0.3);
    cube3.set_z_length(1.5);
    cube3.set_center(3.0, 0.0, 0.0);

    let mut mapper3 = PolyDataMapper::new();
    mapper3.set_input_connection(cube3.get_output_port());

    let mut actor3 = Actor::new();
    actor3.set_mapper(&mut mapper3);
    let mut prop3 = actor3.get_property();
    prop3.set_color(0.2, 0.5, 0.9); // Blue
    renderer.add_actor(&mut actor3);

    // 4. Small block - using bounds
    let mut cube4 = CubeSource::new();
    cube4.set_bounds(-3.5, -2.5, -2.5, -1.5, -0.3, 0.3);

    let mut mapper4 = PolyDataMapper::new();
    mapper4.set_input_connection(cube4.get_output_port());

    let mut actor4 = Actor::new();
    actor4.set_mapper(&mut mapper4);
    let mut prop4 = actor4.get_property();
    prop4.set_color(0.9, 0.7, 0.1); // Gold
    renderer.add_actor(&mut actor4);

    // 5. Long beam - stretched in X
    let mut cube5 = CubeSource::new();
    cube5.set_x_length(2.5);
    cube5.set_y_length(0.4);
    cube5.set_z_length(0.4);
    cube5.set_center(0.0, -2.0, 0.0);

    let mut mapper5 = PolyDataMapper::new();
    mapper5.set_input_connection(cube5.get_output_port());

    let mut actor5 = Actor::new();
    actor5.set_mapper(&mut mapper5);
    let mut prop5 = actor5.get_property();
    prop5.set_color(0.7, 0.3, 0.9); // Purple
    renderer.add_actor(&mut actor5);

    // 6. Wide thin sheet - like a door
    let mut cube6 = CubeSource::new();
    cube6.set_x_length(1.5);
    cube6.set_y_length(2.0);
    cube6.set_z_length(0.1);
    cube6.set_center(3.0, -2.0, 0.0);

    let mut mapper6 = PolyDataMapper::new();
    mapper6.set_input_connection(cube6.get_output_port());

    let mut actor6 = Actor::new();
    actor6.set_mapper(&mut mapper6);
    let mut prop6 = actor6.get_property();
    prop6.set_color(0.1, 0.9, 0.9); // Cyan
    renderer.add_actor(&mut actor6);

    // Add text information
    println!("VTK CubeSource Demo");
    println!("===================");
    println!("Top row (left to right):");
    println!("  1. Unit cube - Red, 1x1x1 (default)");
    println!("  2. Tall box - Green, 0.8x2.5x0.8 (stretched in Y)");
    println!("  3. Flat platform - Blue, 2.0x0.3x1.5 (stretched in XZ)");
    println!("\nBottom row (left to right):");
    println!("  4. Small block - Gold, defined using bounds");
    println!("  5. Long beam - Purple, 2.5x0.4x0.4 (stretched in X)");
    println!("  6. Thin sheet - Cyan, 1.5x2.0x0.1 (door-like)");
    println!("\nInteraction: Left-drag to rotate, right-drag/scroll to zoom");

    // Reset camera and render
    renderer.reset_camera();
    render_window.render();
    interactor.start();
}
