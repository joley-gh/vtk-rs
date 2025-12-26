// Interactive AreaPicker Demo
// Demonstrates rectangular area selection to pick multiple objects
//
// Usage:
// - Left-click and drag to create a selection rectangle
// - Console output shows the selection coordinates in real-time
// - Visual rubber band rectangle appears during selection
// - All objects within the rectangle will be picked
// - Right-drag to rotate | Scroll to zoom

use vtk_rs::*;

struct PickerState {
    area_picker: AreaPicker,
    renderer_ptr: usize,
    render_window_ptr: usize,
    style_ptr: usize,
    start_x: i32,
    start_y: i32,
    // Cache the base frame pixels to avoid re-rendering on every mouse move
    base_frame: Option<Vec<u8>>,
}

impl PickerState {
    fn new(
        area_picker: AreaPicker,
        renderer_ptr: usize,
        render_window_ptr: usize,
        style_ptr: usize
    ) -> Self {
        Self {
            area_picker,
            renderer_ptr,
            render_window_ptr,
            style_ptr,
            start_x: 0,
            start_y: 0,
            base_frame: None,
        }
    }
}

fn main() {
    println!("\n═══════════════════════════════════════════════════════");
    println!("Interactive AreaPicker Demo");
    println!("═══════════════════════════════════════════════════════\n");
    println!("Drag to create a selection rectangle to pick multiple objects.");
    println!("Watch the console for real-time selection coordinates.\n");

    let mut renderer = Renderer::new();
    renderer.set_background(0.1, 0.2, 0.3);

    // Create a single sphere for testing
    let mut sphere1 = SphereSource::new();
    sphere1.set_center([-3.0, 0.0, 0.0]);
    sphere1.set_radius(1.0);
    sphere1.set_theta_resolution(16);
    sphere1.set_phi_resolution(12);

    let mut mapper1 = PolyDataMapper::new();
    let output1 = SphereSource::get_output_port(&mut sphere1);
    mapper1.set_input_connection(output1);

    let mut actor1 = Actor::new();
    actor1.set_mapper(&mut mapper1);
    let mut prop1 = actor1.get_property();
    prop1.set_color(1.0, 0.4, 0.4);
    renderer.add_actor(&mut actor1);

    // Add a second sphere
    let mut sphere2 = SphereSource::new();
    sphere2.set_center([3.0, 0.0, 0.0]);
    sphere2.set_radius(1.0);
    sphere2.set_theta_resolution(16);
    sphere2.set_phi_resolution(12);

    let mut mapper2 = PolyDataMapper::new();
    let output2 = SphereSource::get_output_port(&mut sphere2);
    mapper2.set_input_connection(output2);

    let mut actor2 = Actor::new();
    actor2.set_mapper(&mut mapper2);
    let mut prop2 = actor2.get_property();
    prop2.set_color(0.4, 0.6, 1.0);
    renderer.add_actor(&mut actor2);

    // Create render window and interactor
    let mut render_window = RenderWindow::new();
    render_window.add_renderer(&mut renderer);
    render_window.set_size(800, 600);
    render_window.set_window_name("AreaPicker - Drag to Select");

    let mut interactor = RenderWindowInteractor::new();
    interactor.set_render_window(&mut render_window);

    // Create area picker
    let area_picker = AreaPicker::new();

    // Store pointers as usize for thread safety
    let renderer_ptr = &mut renderer as *mut Renderer as usize;
    let render_window_ptr = &mut render_window as *mut RenderWindow as usize;

    // Create picker state (style_ptr will be set after style is created)
    let picker_state = Box::new(PickerState::new(area_picker, renderer_ptr, render_window_ptr, 0));
    let picker_state_ptr = Box::into_raw(picker_state) as usize;

    // Mouse press callback - record start position and cache base frame
    let on_left_button_press = move |x: i32, y: i32| {
        unsafe {
            let state = &mut *(picker_state_ptr as *mut PickerState);
            let window = &mut *(state.render_window_ptr as *mut RenderWindow);

            state.start_x = x;
            state.start_y = y;

            // Render once and cache the base frame to avoid flickering
            window.render();
            state.base_frame = Some(window.get_pixel_data());

            println!("\n═══ Starting selection at ({}, {}) ═══", x, y);
        }
    };

    // Mouse move callback - draw rubber band during drag (optimized with frame caching)
    let on_mouse_move = move |x: i32, y: i32| {
        unsafe {
            let state = &mut *(picker_state_ptr as *mut PickerState);
            let style = &*(state.style_ptr as *mut InteractorStyleCustom);
            let window = &mut *(state.render_window_ptr as *mut RenderWindow);

            // Only draw if we're actually dragging (selection in progress)
            if style.is_moving() {
                if let Some(ref base_frame) = state.base_frame {
                    // Use cached frame for smooth drawing without re-rendering
                    interactor_style_custom::draw_rubber_band_rectangle_cached(
                        window,
                        base_frame,
                        state.start_x,
                        state.start_y,
                        x,
                        y,
                        (255, 255, 0), // Yellow
                        2
                    );
                }
            }
        }
    };

    // Mouse release callback - perform area pick
    let on_left_button_release = move |x: i32, y: i32| {
        unsafe {
            let state = &mut *(picker_state_ptr as *mut PickerState);

            // Get window size for coordinate conversion
            let window = &mut *(state.render_window_ptr as *mut RenderWindow);
            let size = window.get_size();
            let window_height = size.1;

            // Convert to VTK coordinates (bottom-left origin)
            let y0 = window_height - state.start_y;
            let y1 = window_height - y;
            let x0 = state.start_x;
            let x1 = x;

            // Clear the inline coordinate display
            println!();
            println!(
                "═══ Selection complete: ({}, {}) to ({}, {}) [VTK coords] ═══",
                x0,
                y0,
                x1,
                y1
            );

            // Convert renderer pointer back
            let renderer_ref = &mut *(state.renderer_ptr as *mut Renderer);

            // Perform area pick
            let picked = state.area_picker.area_pick(
                x0 as f64,
                y0 as f64,
                x1 as f64,
                y1 as f64,
                renderer_ref
            );

            if picked {
                println!("✅ Area pick successful!");
                println!("   Objects within selection rectangle were found.");
            } else {
                println!("❌ No objects picked in selection area");
            }

            // Clear the cached frame and re-render the scene
            state.base_frame = None;
            window.render();
        }
    };

    // Set up custom interactor style with visual rubber band
    let mut style = InteractorStyleCustom::new();
    style.set_selection_mode(true); // Enable rubber band + disable camera rotation

    // Store style pointer in picker state
    unsafe {
        let state = &mut *(picker_state_ptr as *mut PickerState);
        state.style_ptr = &mut style as *mut InteractorStyleCustom as usize;
    }

    style.set_left_button_press_callback(on_left_button_press);
    style.set_mouse_move_callback(on_mouse_move);
    style.set_left_button_release_callback(on_left_button_release);
    interactor.set_interactor_style_custom(&mut style);

    println!("Scene ready - drag to select objects!");
    println!("Selection coordinates will be displayed in the console.\n");
    println!("\nLeft-drag: Select area | Right-drag: Rotate | Scroll: Zoom\n");

    // Start interaction
    render_window.render();
    interactor.start();

    // Cleanup
    unsafe {
        let _ = Box::from_raw(picker_state_ptr as *mut PickerState);
    }
}
