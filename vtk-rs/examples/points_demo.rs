use vtk_rs as vtk;

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("Points Demo - Dynamic Point Collection");
    println!("═══════════════════════════════════════════════════════\n");

    // Create a Points collection
    let mut points = vtk::Points::new();

    println!("Creating points in a helix pattern...\n");

    // Add points in a helix pattern
    let num_points = 50;
    let radius = 5.0;
    let height = 10.0;

    for i in 0..num_points {
        let t = (i as f64) / (num_points as f64);
        let angle = t * 4.0 * std::f64::consts::PI;
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        let z = height * t;

        let id = points.insert_next_point(x, y, z);
        if i % 10 == 0 {
            println!("  Point {}: ({:.2}, {:.2}, {:.2})", id, x, y, z);
        }
    }

    println!("\nTotal points created: {}", points.get_number_of_points());

    // Demonstrate point modification
    println!("\nModifying point 25...");
    let (x, y, z) = points.get_point(25);
    println!("  Original: ({:.2}, {:.2}, {:.2})", x, y, z);
    points.set_point(25, x, y, z + 2.0);
    let (x, y, z) = points.get_point(25);
    println!("  Modified: ({:.2}, {:.2}, {:.2})", x, y, z);

    // Demonstrate iteration
    println!("\nIterating over first 5 points:");
    for (id, x, y, z) in points.iter().take(5) {
        println!("  Point {}: ({:.2}, {:.2}, {:.2})", id, x, y, z);
    }

    // Create a PolyData to visualize the points
    // For now, just create the points - we'll add visualization in future examples
    println!("\n═══════════════════════════════════════════════════════");
    println!("Points module is working correctly!");
    println!("Next: Create PolyData and use these points for visualization");
    println!("═══════════════════════════════════════════════════════");
}
