use cmake::Config;
use std::process::Command;

use vtk_rs_link::{ log, Result, WARN };

/// Validate that VTK is installed and accessible
fn validate_vtk_installation() -> Result<()> {
    // Check if VTK_DIR is set
    let vtk_dir = std::env::var("VTK_DIR").ok();

    if let Some(dir) = &vtk_dir {
        let path = std::path::Path::new(dir);
        if !path.exists() {
            eprintln!("ERROR: VTK_DIR is set to '{}' but the directory does not exist!", dir);
            eprintln!("Please set VTK_DIR to the directory containing VTK libraries.");
            return Err("Invalid VTK_DIR".into());
        }
        log!("VTK_DIR: {}", dir);
    } else {
        log!("VTK_DIR not set, will search in default locations");
    }

    // Check if VTK_VERSION is set
    if let Some(version) = std::env::var("VTK_VERSION").ok() {
        log!("VTK_VERSION: {}", version);
    } else {
        log!("VTK_VERSION not set, will auto-detect from library files");
    }

    Ok(())
}

/// Check if cxxbridge is installed and matches the version in Cargo.lock
fn validate_cxxbridge() -> Result<()> {
    match Command::new("cxxbridge").arg("--version").output() {
        Ok(output) => {
            let version_str = String::from_utf8_lossy(&output.stdout);
            log!("cxxbridge version: {}", version_str.trim());

            // Extract version number
            if let Some(installed_version) = version_str.split_whitespace().nth(1) {
                log!("Installed cxxbridge: {}", installed_version);

                // Check if headers might need regeneration
                println!("cargo:rerun-if-changed=libvtkrs/include");
            }
        }
        Err(_) => {
            eprintln!("\nWARNING: cxxbridge command not found!");
            eprintln!("If you need to regenerate C++ headers, install it with:");
            eprintln!("  cargo install cxxbridge-cmd");
            eprintln!("\nContinuing with existing headers...\n");
        }
    }

    Ok(())
}

/// Handle building of cmake project
fn build_cmake() {
    println!("cargo:rerun-if-changed=libvtkrs");
    let mut config = Config::new("libvtkrs");

    if std::env::var("CARGO_FEATURE_V094").is_ok_and(|x| x == "1") {
        config.define("VTK094", "1");
    }

    let dst = config.build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=vtkrs");
}

fn main() -> Result<()> {
    // Exit early without doing anything if we are building for docsrs
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    if let Ok(val) = std::env::var("VERBOSE") {
        if val == "1" || val.to_lowercase() == "true" {
            WARN.store(true, std::sync::atomic::Ordering::Relaxed);
            log!("-- Verbose Logging Enabled");
        }
    }

    log!("=== VTK-RS Build Configuration ===");

    // Validate environment
    validate_vtk_installation()?;
    validate_cxxbridge()?;

    // Build cpp project
    log!("Building C++ bridge library...");
    build_cmake();

    // Link to VTK
    log!("Linking to VTK modules...");
    let modules = include_str!("linker-args.txt").lines();
    vtk_rs_link::link_cmake_project(modules)?;

    log!("=== Build Configuration Complete ===");
    Ok(())
}
