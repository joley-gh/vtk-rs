use vtk_rs::*;

fn main() {
    println!("\n═══════════════════════════════════════════════════════");
    println!("Interactive PointPicker Demo");
    println!("═══════════════════════════════════════════════════════\n");
    println!("Click on the sphere to pick individual points (vertices).");
    println!("Picked points will be highlighted with small red spheres.\n");

    let mut renderer = Renderer::new();
    renderer.set_background(0.1, 0.1, 0.15);

    // Create plane with visible resolution
    let mut plane = PlaneSource::new();
    plane.set_x_resolution(20);
    plane.set_y_resolution(20);

    let mut plane_mapper = PolyDataMapper::new();
    plane_mapper.set_input_connection(plane.get_output_port());

    let mut plane_actor = Actor::new();
    plane_actor.set_mapper(&mut plane_mapper);
    let mut plane_prop = plane_actor.get_property();
    plane_prop.set_color(0.3, 0.6, 0.9);
    plane_prop.set_representation(RepresentationType::Wireframe);
    renderer.add_actor(&mut plane_actor);

    // Add sphere for point picking too
    let mut sphere = SphereSource::new();
    sphere.set_radius(3.0);
    sphere.set_center([0.0, 0.0, 3.0]);
    sphere.set_theta_resolution(12);
    sphere.set_phi_resolution(10);

    let mut sphere_mapper = PolyDataMapper::new();
    let sphere_output = SphereSource::get_output_port(&mut sphere);
    sphere_mapper.set_input_connection(sphere_output);

    let mut sphere_actor = Actor::new();
    sphere_actor.set_mapper(&mut sphere_mapper);
    let mut sphere_prop = sphere_actor.get_property();
    sphere_prop.set_color(0.8, 0.3, 0.3);
    renderer.add_actor(&mut sphere_actor);

    let mut render_window = RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(1200, 800);
    render_window.set_window_name("PointPicker Demo");

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    let mut style = InteractorStyleCustom::new();

    let renderer_ptr = &mut renderer as *mut Renderer as usize;
    let render_window_ptr = &mut render_window as *mut RenderWindow as usize;

    println!("Scene ready - click on vertices!\n");

    style.set_left_button_press_callback(move |x, y| {
        unsafe {
            println!("\n═══ Click at ({}, {}) ═══", x, y);

            let renderer = &mut *(renderer_ptr as *mut Renderer);
            let mut picker = PointPicker::new();

            if picker.pick(x as f64, y as f64, 0.0, renderer) {
                let point_id = picker.get_point_id();
                let (px, py, pz) = picker.get_pick_position();

                println!("✅ Point picked!");
                println!("  ID: {} at ({:.3}, {:.3}, {:.3})", point_id, px, py, pz);

                let highlight = Box::leak(Box::new(SphereSource::new()));
                highlight.set_radius(0.3);
                highlight.set_center([px, py, pz]);

                let mapper = Box::leak(Box::new(PolyDataMapper::new()));
                mapper.set_input_connection(highlight.get_output_port());

                let actor = Box::leak(Box::new(Actor::new()));
                actor.set_mapper(mapper);

                let mut prop = actor.get_property();
                prop.set_color(1.0, 0.2, 0.2);

                renderer.add_actor(actor);

                let render_window = &mut *(render_window_ptr as *mut RenderWindow);
                render_window.render();
            } else {
                println!("✗ No point picked");
            }
        }
    });

    interactor.set_interactor_style_custom(&mut style);
    renderer.reset_camera();
    interactor.initialize();
    render_window.render();

    println!("Left-click: Pick points | Right-drag: Rotate | Scroll: Zoom\n");

    interactor.start();
}
