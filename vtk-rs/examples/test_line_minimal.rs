use vtk_rs as vtk;

fn main() {
    println!("Creating LineSource...");
    let mut line = vtk::LineSource::new();
    println!("LineSource created successfully!");
    drop(line);
    println!("LineSource dropped successfully!");
}
