use vtk_rs::*;

fn main() {
    println!("\n=== VTK PlaneSource Demo ===\n");
    println!("Demonstrating 6 different plane configurations:");
    println!("1. Ground plane (XY plane at z=0)");
    println!("2. Wall plane (XZ vertical plane)");
    println!("3. Side plane (YZ vertical plane)");
    println!("4. High-resolution mesh (10x10)");
    println!("5. Rotated plane (angled with normal)");
    println!("6. Pushed plane (offset along normal)\n");

    let mut renderer = Renderer::new();
    renderer.set_background(0.1, 0.1, 0.1);

    // 1. Ground plane - XY plane at z=0 (RED)
    let mut plane1 = PlaneSource::default();
    plane1.set_origin(-4.0, -1.0, 0.0);
    plane1.set_point1(-2.0, -1.0, 0.0); // 2 units along X
    plane1.set_point2(-4.0, 1.0, 0.0); // 2 units along Y
    plane1.set_x_resolution(2);
    plane1.set_y_resolution(2);

    let mut mapper1 = PolyDataMapper::new();
    mapper1.set_input_connection(plane1.get_output_port());

    let mut actor1 = Actor::new();
    actor1.set_mapper(&mut mapper1);
    let mut prop1 = actor1.get_property();
    prop1.set_color(0.8, 0.2, 0.2); // Red
    renderer.add_actor(&mut actor1);
    println!("1. Ground plane - XY at z=0, 2x2 resolution, RED");

    // 2. Wall plane - XZ vertical plane (GREEN)
    let mut plane2 = PlaneSource::default();
    plane2.set_origin(-1.0, 0.0, -1.0);
    plane2.set_point1(1.0, 0.0, -1.0); // 2 units along X
    plane2.set_point2(-1.0, 0.0, 1.0); // 2 units along Z
    plane2.set_x_resolution(2);
    plane2.set_y_resolution(2);

    let mut mapper2 = PolyDataMapper::new();
    mapper2.set_input_connection(plane2.get_output_port());

    let mut actor2 = Actor::new();
    actor2.set_mapper(&mut mapper2);
    let mut prop2 = actor2.get_property();
    prop2.set_color(0.2, 0.8, 0.2); // Green
    renderer.add_actor(&mut actor2);
    println!("2. Wall plane - XZ vertical, 2x2 resolution, GREEN");

    // 3. Side plane - YZ vertical plane (BLUE)
    let mut plane3 = PlaneSource::default();
    plane3.set_origin(0.0, -1.0, -1.0);
    plane3.set_point1(0.0, 1.0, -1.0); // 2 units along Y
    plane3.set_point2(0.0, -1.0, 1.0); // 2 units along Z
    plane3.set_x_resolution(2);
    plane3.set_y_resolution(2);
    plane3.set_center(3.0, 0.0, 0.0); // Move to the right

    let mut mapper3 = PolyDataMapper::new();
    mapper3.set_input_connection(plane3.get_output_port());

    let mut actor3 = Actor::new();
    actor3.set_mapper(&mut mapper3);
    let mut prop3 = actor3.get_property();
    prop3.set_color(0.2, 0.5, 0.9); // Blue
    renderer.add_actor(&mut actor3);
    println!("3. Side plane - YZ vertical, 2x2 resolution, BLUE");

    // 4. High-resolution mesh plane (GOLD)
    let mut plane4 = PlaneSource::default();
    plane4.set_center(-3.0, 0.0, 3.0); // Position at top-left
    plane4.set_normal(0.0, 0.0, 1.0); // Face up
    plane4.set_x_resolution(10); // Fine mesh
    plane4.set_y_resolution(10);

    let mut mapper4 = PolyDataMapper::new();
    mapper4.set_input_connection(plane4.get_output_port());

    let mut actor4 = Actor::new();
    actor4.set_mapper(&mut mapper4);
    let mut prop4 = actor4.get_property();
    prop4.set_color(0.9, 0.7, 0.1); // Gold
    renderer.add_actor(&mut actor4);
    let res_x = plane4.get_x_resolution();
    let res_y = plane4.get_y_resolution();
    println!("4. High-res mesh - {}x{} resolution, GOLD", res_x, res_y);

    // 5. Plane with custom normal - angled (PURPLE)
    let mut plane5 = PlaneSource::default();
    plane5.set_center(0.0, 0.0, 3.0);
    plane5.set_normal(1.0, 1.0, 1.0); // Diagonal normal
    plane5.set_x_resolution(5);
    plane5.set_y_resolution(5);

    let mut mapper5 = PolyDataMapper::new();
    mapper5.set_input_connection(plane5.get_output_port());

    let mut actor5 = Actor::new();
    actor5.set_mapper(&mut mapper5);
    let mut prop5 = actor5.get_property();
    prop5.set_color(0.7, 0.3, 0.9); // Purple
    renderer.add_actor(&mut actor5);
    println!("5. Rotated plane - normal(1,1,1), center at (0,0,3), PURPLE");

    // 6. Pushed plane - offset along normal (CYAN)
    let mut plane6 = PlaneSource::default();
    plane6.set_center(3.0, 0.0, 3.0);
    plane6.set_normal(0.0, 0.0, 1.0); // Z normal
    plane6.push(1.5); // Push 1.5 units along normal
    plane6.set_x_resolution(4);
    plane6.set_y_resolution(4);

    let mut mapper6 = PolyDataMapper::new();
    mapper6.set_input_connection(plane6.get_output_port());

    let mut actor6 = Actor::new();
    actor6.set_mapper(&mut mapper6);
    let mut prop6 = actor6.get_property();
    prop6.set_color(0.1, 0.9, 0.9); // Cyan
    renderer.add_actor(&mut actor6);
    let (ox, oy, oz) = plane6.get_origin();
    println!("6. Pushed plane - center(3,0,3), pushed 1.5 along Z normal, CYAN");
    println!("   Origin after push: ({:.2}, {:.2}, {:.2})", ox, oy, oz);

    let mut render_window = RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(1200, 800);
    render_window.set_window_name("PlaneSource Demo - 6 Different Planes");

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    let mut style = InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    renderer.reset_camera();
    render_window.render();

    println!("\nRendering 6 different plane configurations.");
    println!("Use mouse to interact: Left-drag=rotate, Right-drag/Scroll=zoom");

    interactor.start();
}
