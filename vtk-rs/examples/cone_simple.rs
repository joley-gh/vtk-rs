use vtk_rs::*;

fn main() {
    println!("Creating ConeSource...");
    let mut cone = ConeSource::new();
    println!("✓ ConeSource created");

    println!("Setting radius...");
    cone.set_radius(1.0);
    println!("✓ Radius set");

    println!("Getting radius...");
    let radius = cone.get_radius();
    println!("✓ Radius: {}", radius);

    println!("Setting height...");
    cone.set_height(2.0);
    println!("✓ Height set");

    println!("Getting height...");
    let height = cone.get_height();
    println!("✓ Height: {}", height);

    println!("\nAll ConeSource basic operations work!");
}
