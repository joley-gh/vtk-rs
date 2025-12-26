use vtk_rs::*;

fn main() {
    println!("\n=== VTK ArrowSource Demo ===\n");
    println!("Demonstrating 6 different arrow configurations:");
    println!("1. Default arrow (center, red)");
    println!("2. Thin arrow (top-left, green, pointing up)");
    println!("3. Fat arrow (top-right, blue, 45° rotation)");
    println!("4. Long tip arrow (bottom-left, gold, pointing down)");
    println!("5. Short tip arrow (bottom-right, purple, scaled & rotated)");
    println!("6. Inverted arrow (far left, cyan, tilted in 3D)\n");

    let mut renderer = Renderer::new();
    renderer.set_background(0.1, 0.1, 0.1);

    // 1. Default arrow - standard proportions (RED, center)
    let mut arrow1 = ArrowSource::new();
    arrow1.set_arrow_origin_to_default();
    arrow1.set_tip_resolution(16);
    arrow1.set_shaft_resolution(16);

    let mut mapper1 = PolyDataMapper::new();
    mapper1.set_input_connection(arrow1.get_output_port());

    let mut actor1 = Actor::new();
    actor1.set_mapper(&mut mapper1);
    actor1.set_position(0.0, 0.0, 0.0); // Center
    let mut prop1 = actor1.get_property();
    prop1.set_color(0.8, 0.2, 0.2); // Red
    renderer.add_actor(&mut actor1);

    let origin1 = arrow1.get_arrow_origin_as_string();
    println!("1. Default arrow - origin: {}, RED (center)", origin1);

    // 2. Thin arrow - small shaft (GREEN)
    let mut arrow2 = ArrowSource::new();
    arrow2.set_arrow_origin_to_default();
    arrow2.set_shaft_radius(0.01); // Very thin
    arrow2.set_tip_radius(0.05);
    arrow2.set_tip_length(0.35);
    arrow2.set_tip_resolution(12);
    arrow2.set_shaft_resolution(12);

    let mut mapper2 = PolyDataMapper::new();
    mapper2.set_input_connection(arrow2.get_output_port());

    let mut actor2 = Actor::new();
    actor2.set_mapper(&mut mapper2);
    actor2.set_position(-1.5, 1.5, 0.0); // Top-left
    actor2.rotate_z(90.0); // Point upward
    let mut prop2 = actor2.get_property();
    prop2.set_color(0.2, 0.8, 0.2); // Green
    renderer.add_actor(&mut actor2);

    let shaft_r2 = arrow2.get_shaft_radius();
    let tip_r2 = arrow2.get_tip_radius();
    println!("2. Thin arrow - shaft: {:.3}, tip: {:.3}, GREEN", shaft_r2, tip_r2);

    // 3. Fat arrow - large shaft (BLUE)
    let mut arrow3 = ArrowSource::new();
    arrow3.set_arrow_origin_to_default();
    arrow3.set_shaft_radius(0.08); // Thick
    arrow3.set_tip_radius(0.15);
    arrow3.set_tip_length(0.35);
    arrow3.set_tip_resolution(24);
    arrow3.set_shaft_resolution(24);

    let mut mapper3 = PolyDataMapper::new();
    mapper3.set_input_connection(arrow3.get_output_port());

    let mut actor3 = Actor::new();
    actor3.set_mapper(&mut mapper3);
    actor3.set_position(1.5, 1.5, 0.0); // Top-right
    actor3.rotate_z(45.0); // Diagonal
    let mut prop3 = actor3.get_property();
    prop3.set_color(0.2, 0.5, 0.9); // Blue
    renderer.add_actor(&mut actor3);

    let shaft_r3 = arrow3.get_shaft_radius();
    let shaft_res3 = arrow3.get_shaft_resolution();
    println!("3. Fat arrow - shaft: {:.2}, resolution: {}, BLUE", shaft_r3, shaft_res3);

    // 4. Long tip arrow - extended tip (GOLD)
    let mut arrow4 = ArrowSource::new();
    arrow4.set_arrow_origin_to_default();
    arrow4.set_tip_length(0.6); // Long tip (default is 0.35)
    arrow4.set_tip_radius(0.1);
    arrow4.set_shaft_radius(0.03);
    arrow4.set_tip_resolution(16);
    arrow4.set_shaft_resolution(16);

    let mut mapper4 = PolyDataMapper::new();
    mapper4.set_input_connection(arrow4.get_output_port());

    let mut actor4 = Actor::new();
    actor4.set_mapper(&mut mapper4);
    actor4.set_position(-1.5, -1.5, 0.0); // Bottom-left
    actor4.rotate_z(-90.0); // Point downward
    let mut prop4 = actor4.get_property();
    prop4.set_color(0.9, 0.7, 0.1); // Gold
    renderer.add_actor(&mut actor4);

    let tip_len4 = arrow4.get_tip_length();
    println!("4. Long tip arrow - tip length: {:.2}, GOLD", tip_len4);

    // 5. Short tip arrow - compact tip (PURPLE)
    let mut arrow5 = ArrowSource::new();
    arrow5.set_arrow_origin_to_default();
    arrow5.set_tip_length(0.15); // Short tip
    arrow5.set_tip_radius(0.1);
    arrow5.set_shaft_radius(0.04);
    arrow5.set_tip_resolution(16);
    arrow5.set_shaft_resolution(16);

    let mut mapper5 = PolyDataMapper::new();
    mapper5.set_input_connection(arrow5.get_output_port());

    let mut actor5 = Actor::new();
    actor5.set_mapper(&mut mapper5);
    actor5.set_position(1.5, -1.5, 0.0); // Bottom-right
    actor5.set_scale(1.2, 1.2, 1.2); // Scale up slightly
    actor5.rotate_z(180.0); // Point backward
    let mut prop5 = actor5.get_property();
    prop5.set_color(0.7, 0.3, 0.9); // Purple
    renderer.add_actor(&mut actor5);

    let tip_len5 = arrow5.get_tip_length();
    let tip_res5 = arrow5.get_tip_resolution();
    println!("5. Short tip arrow - tip length: {:.2}, resolution: {}, PURPLE", tip_len5, tip_res5);

    // 6. Inverted arrow - pointing backward (CYAN)
    let mut arrow6 = ArrowSource::new();
    arrow6.set_arrow_origin_to_center();
    arrow6.set_invert(true); // Point in opposite direction
    arrow6.set_tip_resolution(16);
    arrow6.set_shaft_resolution(16);

    let mut mapper6 = PolyDataMapper::new();
    mapper6.set_input_connection(arrow6.get_output_port());

    let mut actor6 = Actor::new();
    actor6.set_mapper(&mut mapper6);
    actor6.set_position(-3.0, 0.0, 0.0); // Far left
    actor6.rotate_y(45.0); // Tilt in 3D
    actor6.rotate_z(30.0); // Additional rotation
    let mut prop6 = actor6.get_property();
    prop6.set_color(0.1, 0.9, 0.9); // Cyan
    renderer.add_actor(&mut actor6);

    let is_inverted = arrow6.get_invert();
    let origin6 = arrow6.get_arrow_origin_as_string();
    println!("6. Inverted arrow - inverted: {}, origin: {}, CYAN", is_inverted, origin6);

    let mut render_window = RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(1200, 800);
    render_window.set_window_name("ArrowSource Demo - Various Arrow Styles");

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    let mut style = InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    renderer.reset_camera();
    render_window.render();

    println!("\nRendering 6 different arrow configurations.");
    println!("Use mouse to interact: Left-drag=rotate, Right-drag/Scroll=zoom");

    interactor.start();
}
