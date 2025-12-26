use vtk_rs as vtk;

fn main() {
    println!("Testing Command creation...");
    vtk::init_vtk();

    println!("Creating Command...");
    let command = vtk::Command::new();
    println!("Command created successfully!");

    drop(command);
    println!("Command dropped successfully!");
}
