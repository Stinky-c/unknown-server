use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // trigger recompilation when a new migration is added
    println!("cargo:rerun-if-changed=migrations/");

    // trigger recompilation when an asset or template is added
    println!("cargo:rerun-if-changed=templates/");
    println!("cargo:rerun-if-changed=assets/");

    // In release mode, copy assets from unknown-web/dist to assets/
    if env::var("PROFILE").unwrap_or_default() == "release" {
        copy_web_assets();
        println!("cargo:rerun-if-changed=../unknown-web/dist/");
    }

    // Load jinja templates
}

fn copy_web_assets() {
    let web_dist_path = Path::new("../unknown-web/dist");
    let assets_path = Path::new("dist");

    if !web_dist_path.exists() {
        println!(
            "cargo:warning=unknown-web/dist directory not found. Run 'mise run unknown-web:build' first."
        );
        return;
    }

    // Clear existing assets (except keep any manually placed files)
    if assets_path.exists() {
        if let Err(e) = fs::remove_dir_all(&assets_path) {
            println!("cargo:warning=Failed to clean assets directory: {}", e);
        }
    }

    // Copy web assets
    if let Err(e) = copy_dir_recursive(&web_dist_path, &assets_path) {
        println!("cargo:warning=Failed to copy web assets: {}", e);
    } else {
        println!("cargo:warning=Successfully copied web assets from dist/ to assets/");
    }
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            if let Some(parent) = dst_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
