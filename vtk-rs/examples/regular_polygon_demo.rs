use vtk_rs::*;

fn main() {
    println!("\n=== VTK RegularPolygonSource Demo ===\n");
    println!("Demonstrating 6 different regular polygon configurations:");
    println!("1. Triangle (3 sides, red)");
    println!("2. Square (4 sides, green)");
    println!("3. Pentagon (5 sides, blue)");
    println!("4. Hexagon (6 sides, gold)");
    println!("5. Octagon (8 sides, purple)");
    println!("6. Decagon (10 sides, cyan)\n");

    let mut renderer = Renderer::new();
    renderer.set_background(0.1, 0.1, 0.1);

    // 1. Triangle (3 sides) - RED
    let mut triangle = RegularPolygonSource::new();
    triangle.set_number_of_sides(3);
    triangle.set_center(0.0, 2.0, 0.0);
    triangle.set_radius(0.6);
    triangle.set_normal(0.0, 0.0, 1.0);
    triangle.generate_polygon_on();
    
    let mut mapper1 = PolyDataMapper::new();
    mapper1.set_input_connection(triangle.get_output_port());
    
    let mut actor1 = Actor::new();
    actor1.set_mapper(&mut mapper1);
    let mut prop1 = actor1.get_property();
    prop1.set_color(0.8, 0.2, 0.2); // Red
    renderer.add_actor(&mut actor1);
    
    let sides1 = triangle.get_number_of_sides();
    let radius1 = triangle.get_radius();
    let (cx1, cy1, cz1) = triangle.get_center();
    println!("1. Triangle - sides: {}, radius: {:.2}, center: ({:.1}, {:.1}, {:.1})", 
             sides1, radius1, cx1, cy1, cz1);

    // 2. Square (4 sides) - GREEN
    let mut square = RegularPolygonSource::new();
    square.set_number_of_sides(4);
    square.set_center(2.0, 2.0, 0.0);
    square.set_radius(0.6);
    square.set_normal(0.0, 0.0, 1.0);
    square.generate_polygon_on();
    
    let mut mapper2 = PolyDataMapper::new();
    mapper2.set_input_connection(square.get_output_port());
    
    let mut actor2 = Actor::new();
    actor2.set_mapper(&mut mapper2);
    actor2.rotate_z(22.5); // Rotate square for visual variety
    let mut prop2 = actor2.get_property();
    prop2.set_color(0.2, 0.8, 0.2); // Green
    renderer.add_actor(&mut actor2);
    
    let sides2 = square.get_number_of_sides();
    let radius2 = square.get_radius();
    println!("2. Square - sides: {}, radius: {:.2}, rotated 22.5°", sides2, radius2);

    // 3. Pentagon (5 sides) - BLUE
    let mut pentagon = RegularPolygonSource::new();
    pentagon.set_number_of_sides(5);
    pentagon.set_center(-2.0, 0.0, 0.0);
    pentagon.set_radius(0.6);
    pentagon.set_normal(0.0, 0.0, 1.0);
    pentagon.generate_polygon_on();
    
    let mut mapper3 = PolyDataMapper::new();
    mapper3.set_input_connection(pentagon.get_output_port());
    
    let mut actor3 = Actor::new();
    actor3.set_mapper(&mut mapper3);
    let mut prop3 = actor3.get_property();
    prop3.set_color(0.2, 0.5, 0.9); // Blue
    renderer.add_actor(&mut actor3);
    
    let sides3 = pentagon.get_number_of_sides();
    println!("3. Pentagon - sides: {}", sides3);

    // 4. Hexagon (6 sides) - GOLD
    let mut hexagon = RegularPolygonSource::new();
    hexagon.set_number_of_sides(6);
    hexagon.set_center(0.0, 0.0, 0.0);
    hexagon.set_radius(0.6);
    hexagon.set_normal(0.0, 0.0, 1.0);
    hexagon.generate_polygon_on();
    
    let mut mapper4 = PolyDataMapper::new();
    mapper4.set_input_connection(hexagon.get_output_port());
    
    let mut actor4 = Actor::new();
    actor4.set_mapper(&mut mapper4);
    let mut prop4 = actor4.get_property();
    prop4.set_color(0.9, 0.7, 0.1); // Gold
    renderer.add_actor(&mut actor4);
    
    let sides4 = hexagon.get_number_of_sides();
    println!("4. Hexagon - sides: {}", sides4);

    // 5. Octagon (8 sides) - PURPLE
    let mut octagon = RegularPolygonSource::new();
    octagon.set_number_of_sides(8);
    octagon.set_center(2.0, 0.0, 0.0);
    octagon.set_radius(0.6);
    octagon.set_normal(0.0, 0.0, 1.0);
    octagon.generate_polygon_on();
    
    let mut mapper5 = PolyDataMapper::new();
    mapper5.set_input_connection(octagon.get_output_port());
    
    let mut actor5 = Actor::new();
    actor5.set_mapper(&mut mapper5);
    let mut prop5 = actor5.get_property();
    prop5.set_color(0.7, 0.3, 0.9); // Purple
    renderer.add_actor(&mut actor5);
    
    let sides5 = octagon.get_number_of_sides();
    println!("5. Octagon - sides: {}", sides5);

    // 6. Decagon (10 sides) - CYAN (with polyline instead of filled)
    let mut decagon = RegularPolygonSource::new();
    decagon.set_number_of_sides(10);
    decagon.set_center(0.0, -2.0, 0.0);
    decagon.set_radius(0.7);
    decagon.set_normal(0.0, 0.0, 1.0);
    decagon.generate_polygon_off();
    decagon.generate_polyline_on(); // Outline only
    
    let mut mapper6 = PolyDataMapper::new();
    mapper6.set_input_connection(decagon.get_output_port());
    
    let mut actor6 = Actor::new();
    actor6.set_mapper(&mut mapper6);
    let mut prop6 = actor6.get_property();
    prop6.set_color(0.1, 0.9, 0.9); // Cyan
    prop6.set_line_width(3.0);
    renderer.add_actor(&mut actor6);
    
    let sides6 = decagon.get_number_of_sides();
    let gen_polygon = decagon.get_generate_polygon();
    let gen_polyline = decagon.get_generate_polyline();
    println!("6. Decagon - sides: {}, polygon: {}, polyline: {}", 
             sides6, gen_polygon, gen_polyline);

    let mut render_window = RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(1200, 800);
    render_window.set_window_name("RegularPolygonSource Demo - Various Polygons");

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);
    
    let mut style = InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    renderer.reset_camera();
    render_window.render();

    println!("\nRendering 6 different regular polygons.");
    println!("Use mouse to interact: Left-drag=rotate, Right-drag/Scroll=zoom");
    
    interactor.start();
}
