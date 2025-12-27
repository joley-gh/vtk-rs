use vtk_rs as vtk;

fn main() {
    println!("Creating UnstructuredGrid with mixed cell types...");

    // Create points for a small mesh
    let mut points = vtk::Points::new();

    // Add vertices for a tetrahedron (pyramid with triangular base)
    points.insert_next_point(0.0, 0.0, 0.0);
    points.insert_next_point(1.0, 0.0, 0.0);
    points.insert_next_point(0.5, 1.0, 0.0);
    points.insert_next_point(0.5, 0.5, 1.0);

    // Add vertices for a hexahedron (cube)
    let cube_offset = 1.5;
    for z in 0..2 {
        for y in 0..2 {
            for x in 0..2 {
                points.insert_next_point((x as f64) + cube_offset, y as f64, z as f64);
            }
        }
    }

    // Create unstructured grid
    let mut grid = vtk::UnstructuredGrid::new();
    grid.set_points(&mut points);
    grid.allocate(2); // 1 tetrahedron + 1 hexahedron

    // Insert tetrahedron
    grid.insert_next_cell(vtk::VtkCellType::Tetra, &[0, 1, 2, 3]);

    // Insert hexahedron (cube)
    // VTK hexahedron ordering: bottom face (CCW), then top face (CCW)
    grid.insert_next_cell(vtk::VtkCellType::Hexahedron, &[4, 5, 7, 6, 8, 9, 11, 10]);

    println!("Grid created:");
    println!("  Points: {}", grid.get_number_of_points());
    println!("  Cells: {}", grid.get_number_of_cells());
    let bounds = grid.get_bounds();
    println!(
        "  Bounds: X[{:.1}, {:.1}] Y[{:.1}, {:.1}] Z[{:.1}, {:.1}]",
        bounds[0],
        bounds[1],
        bounds[2],
        bounds[3],
        bounds[4],
        bounds[5]
    );

    // Create mapper using DataSetMapper (works with any vtkDataSet)
    let mut mapper = vtk::DataSetMapper::new();
    mapper.set_input_data(&mut grid);

    // Create actor
    let mut actor = vtk::Actor::new();
    actor.set_data_set_mapper(&mut mapper);

    // Setup colors
    actor.get_property().set_color(0.9, 0.2, 0.2);
    actor.get_property().set_edge_visibility(true); // Show edges
    actor.get_property().set_line_width(2.0);

    // Create renderer and window
    let mut renderer = vtk::Renderer::new();
    renderer.add_actor(&mut actor);
    renderer.set_background(0.27, 0.51, 0.71); // Light blue background

    let mut render_window = vtk::RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(800, 600);
    render_window.set_window_name("UnstructuredGrid Visualization");

    // Add interactor
    let mut interactor = vtk::RenderWindowInteractor::new();
    let mut interactor_style = vtk::InteractorStyleTrackballCamera::new();
    interactor.set_interactor_style(&mut interactor_style);
    interactor.set_render_window(&mut render_window);

    println!("\nRendering mixed cell types:");
    println!("  - Tetrahedron (left)");
    println!("  - Hexahedron/Cube (right)");
    println!("\nPress 'q' to quit, mouse to rotate");

    render_window.render();
    interactor.start();
}
