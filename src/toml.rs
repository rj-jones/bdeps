extern crate cargo_toml;

use cargo_lock::Lockfile;
pub use cargo_toml::Dependency;
use cargo_toml::Manifest;
use std::path::Path;

pub fn get_dependencies() -> Vec<String> {
    let lockfile = Lockfile::load("Cargo.lock").unwrap();
    let package_names = get_dependency_names_from_toml();
    let mut dependencies: Vec<String> = Vec::with_capacity(128);

    for p in &lockfile.packages {
        if package_names.contains(&p.name.to_string()) {
            dependencies.push(format!("{}@{}", p.name, p.version));
        }
    }

    dependencies
}

fn get_dependency_names_from_toml() -> Vec<String> {
    let path = Path::new("Cargo.toml");
    let manifest = Manifest::from_path(&path).unwrap();
    let dependencies = &manifest.dependencies;
    dependencies.iter().map(|(name, _)| name.clone()).collect()
}
