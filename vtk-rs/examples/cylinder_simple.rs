use vtk_rs::*;

fn main() {
    println!("Creating CylinderSource...");
    let mut cylinder = CylinderSource::new();
    println!("✓ CylinderSource created");

    println!("Setting radius...");
    cylinder.set_radius(1.0);
    println!("✓ Radius set");

    println!("Getting radius...");
    let radius = cylinder.get_radius();
    println!("✓ Radius: {}", radius);

    println!("Setting height...");
    cylinder.set_height(2.0);
    println!("✓ Height set");

    println!("Getting height...");
    let height = cylinder.get_height();
    println!("✓ Height: {}", height);

    println!("Setting center...");
    cylinder.set_center(1.0, 2.0, 3.0);
    println!("✓ Center set");

    println!("Getting center...");
    let center = cylinder.get_center();
    println!("✓ Center: ({}, {}, {})", center.0, center.1, center.2);

    println!("\nAll CylinderSource basic operations work!");
}
