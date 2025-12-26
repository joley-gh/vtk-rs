// Test CellPicker in isolation
use vtk_rs::CellPicker;

fn main() {
    println!("Creating CellPicker...");
    let mut picker = CellPicker::new();
    println!("✓ CellPicker created successfully");

    println!("Setting tolerance...");
    picker.set_tolerance(0.01);
    println!("✓ Tolerance set successfully");

    println!("All tests passed!");
}
