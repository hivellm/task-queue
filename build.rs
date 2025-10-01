use std::process::Command;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=dashboard/src");
    println!("cargo:rerun-if-changed=dashboard/index.html");
    println!("cargo:rerun-if-changed=dashboard/vite.config.js");
    println!("cargo:rerun-if-changed=dashboard/package.json");

    // Check if we're in a development environment
    if std::env::var("PROFILE").unwrap_or_default() == "debug" {
        println!("cargo:warning=Development build - skipping dashboard build");
        return;
    }

    // Build the dashboard
    if Path::new("dashboard/package.json").exists() {
        println!("cargo:warning=Building dashboard...");
        
        // Install dependencies
        let install_output = Command::new("npm")
            .arg("install")
            .current_dir("dashboard")
            .output();

        match install_output {
            Ok(output) => {
                if !output.status.success() {
                    println!("cargo:warning=Failed to install dashboard dependencies: {}", 
                        String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => {
                println!("cargo:warning=Failed to run npm install: {}", e);
            }
        }

        // Build the dashboard
        let build_output = Command::new("npm")
            .arg("run")
            .arg("build")
            .current_dir("dashboard")
            .output();

        match build_output {
            Ok(output) => {
                if !output.status.success() {
                    println!("cargo:warning=Failed to build dashboard: {}", 
                        String::from_utf8_lossy(&output.stderr));
                } else {
                    println!("cargo:warning=Dashboard built successfully");
                }
            }
            Err(e) => {
                println!("cargo:warning=Failed to run npm build: {}", e);
            }
        }
    } else {
        println!("cargo:warning=Dashboard package.json not found, skipping build");
    }
}
