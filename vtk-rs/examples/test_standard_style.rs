use vtk_rs as vtk;

fn main() {
    println!("Testing basic VTK rendering without custom style...\n");

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

    // Use standard trackball style (this works)
    let mut style = vtk::InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut style);

    println!("Standard style works fine. Starting render...");
    
    interactor.initialize();
    render_window.render();
    
    println!("Window opened successfully.\n");
    
    interactor.start();
}
