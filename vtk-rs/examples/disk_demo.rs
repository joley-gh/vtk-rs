use vtk_rs::*;

fn main() {
    println!("\n=== VTK DiskSource Demo ===\n");
    println!("Demonstrating 6 different disk configurations:");
    println!("1. Full disk (inner radius = 0)");
    println!("2. Washer/ring (inner > 0)");
    println!("3. Low resolution disk (6 segments)");
    println!("4. High resolution disk (smooth circle)");
    println!("5. Thick washer (high radial resolution)");
    println!("6. Rotated disk (custom normal)\n");

    let mut renderer = Renderer::new();
    renderer.set_background(0.1, 0.1, 0.1);

    // 1. Full disk - solid circle (RED)
    let mut disk1 = DiskSource::new();
    disk1.set_center(-3.0, 0.0, 0.0);
    disk1.set_inner_radius(0.0);  // No hole
    disk1.set_outer_radius(1.0);
    disk1.set_radial_resolution(2);
    disk1.set_circumferential_resolution(16);
    
    let mut mapper1 = PolyDataMapper::new();
    mapper1.set_input_connection(disk1.get_output_port());
    
    let mut actor1 = Actor::new();
    actor1.set_mapper(&mut mapper1);
    let mut prop1 = actor1.get_property();
    prop1.set_color(0.8, 0.2, 0.2); // Red
    renderer.add_actor(&mut actor1);
    
    let inner1 = disk1.get_inner_radius();
    let outer1 = disk1.get_outer_radius();
    println!("1. Full disk - inner: {:.1}, outer: {:.1}, RED", inner1, outer1);

    // 2. Washer/ring - annulus (GREEN)
    let mut disk2 = DiskSource::new();
    disk2.set_center(0.0, 0.0, 0.0);
    disk2.set_inner_radius(0.4);  // Inner hole
    disk2.set_outer_radius(1.0);
    disk2.set_radial_resolution(2);
    disk2.set_circumferential_resolution(20);
    
    let mut mapper2 = PolyDataMapper::new();
    mapper2.set_input_connection(disk2.get_output_port());
    
    let mut actor2 = Actor::new();
    actor2.set_mapper(&mut mapper2);
    let mut prop2 = actor2.get_property();
    prop2.set_color(0.2, 0.8, 0.2); // Green
    renderer.add_actor(&mut actor2);
    
    let inner2 = disk2.get_inner_radius();
    let outer2 = disk2.get_outer_radius();
    println!("2. Washer - inner: {:.1}, outer: {:.1}, GREEN", inner2, outer2);

    // 3. Low resolution disk - hexagon-like (BLUE)
    let mut disk3 = DiskSource::new();
    disk3.set_center(3.0, 0.0, 0.0);
    disk3.set_inner_radius(0.0);
    disk3.set_outer_radius(1.0);
    disk3.set_radial_resolution(1);
    disk3.set_circumferential_resolution(6);  // Very few segments
    
    let mut mapper3 = PolyDataMapper::new();
    mapper3.set_input_connection(disk3.get_output_port());
    
    let mut actor3 = Actor::new();
    actor3.set_mapper(&mut mapper3);
    let mut prop3 = actor3.get_property();
    prop3.set_color(0.2, 0.5, 0.9); // Blue
    renderer.add_actor(&mut actor3);
    
    let circ3 = disk3.get_circumferential_resolution();
    println!("3. Low-res disk - {} segments (hexagon-like), BLUE", circ3);

    // 4. High resolution disk - smooth (GOLD)
    let mut disk4 = DiskSource::new();
    disk4.set_center(-3.0, 0.0, 3.0);
    disk4.set_inner_radius(0.2);
    disk4.set_outer_radius(1.0);
    disk4.set_radial_resolution(3);
    disk4.set_circumferential_resolution(50);  // Very smooth
    
    let mut mapper4 = PolyDataMapper::new();
    mapper4.set_input_connection(disk4.get_output_port());
    
    let mut actor4 = Actor::new();
    actor4.set_mapper(&mut mapper4);
    let mut prop4 = actor4.get_property();
    prop4.set_color(0.9, 0.7, 0.1); // Gold
    renderer.add_actor(&mut actor4);
    
    let circ4 = disk4.get_circumferential_resolution();
    let radial4 = disk4.get_radial_resolution();
    println!("4. High-res disk - {} circumf, {} radial, GOLD", circ4, radial4);

    // 5. Thick washer - many radial subdivisions (PURPLE)
    let mut disk5 = DiskSource::new();
    disk5.set_center(0.0, 0.0, 3.0);
    disk5.set_inner_radius(0.3);
    disk5.set_outer_radius(1.0);
    disk5.set_radial_resolution(8);  // Many radial rings
    disk5.set_circumferential_resolution(24);
    
    let mut mapper5 = PolyDataMapper::new();
    mapper5.set_input_connection(disk5.get_output_port());
    
    let mut actor5 = Actor::new();
    actor5.set_mapper(&mut mapper5);
    let mut prop5 = actor5.get_property();
    prop5.set_color(0.7, 0.3, 0.9); // Purple
    renderer.add_actor(&mut actor5);
    
    let radial5 = disk5.get_radial_resolution();
    println!("5. Thick washer - {} radial rings, PURPLE", radial5);

    // 6. Rotated disk - vertical orientation (CYAN)
    let mut disk6 = DiskSource::new();
    disk6.set_center(3.0, 0.0, 3.0);
    disk6.set_inner_radius(0.25);
    disk6.set_outer_radius(0.9);
    disk6.set_normal(0.0, 1.0, 0.0);  // Point along Y axis (vertical)
    disk6.set_radial_resolution(3);
    disk6.set_circumferential_resolution(20);
    
    let mut mapper6 = PolyDataMapper::new();
    mapper6.set_input_connection(disk6.get_output_port());
    
    let mut actor6 = Actor::new();
    actor6.set_mapper(&mut mapper6);
    let mut prop6 = actor6.get_property();
    prop6.set_color(0.1, 0.9, 0.9); // Cyan
    renderer.add_actor(&mut actor6);
    
    let (nx, ny, nz) = disk6.get_normal();
    println!("6. Rotated disk - normal({:.1}, {:.1}, {:.1}), CYAN", nx, ny, nz);

    let mut render_window = RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(1200, 800);
    render_window.set_window_name("DiskSource Demo - Circles, Washers & Rings");

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);
    
    let mut style = InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    renderer.reset_camera();
    render_window.render();

    println!("\nRendering 6 different disk configurations.");
    println!("Use mouse to interact: Left-drag=rotate, Right-drag/Scroll=zoom");
    
    interactor.start();
}
