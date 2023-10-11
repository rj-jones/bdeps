extern crate cargo_toml;

pub use cargo_toml::Dependency;
use cargo_toml::Manifest;
use std::{collections::BTreeMap, path::Path};

pub fn parse_toml() -> BTreeMap<String, Dependency> {
    let path = Path::new("Cargo.toml");
    let manifest = Manifest::from_path(&path).unwrap();
    let dependencies = &manifest.dependencies;

    // for (name, dep) in dependencies {
    //     match dep {
    //         Dependency::Simple(version) => println!("{}:{}", name, version),
    //         _ => (),
    //     }
    // }

    dependencies.clone()
}
