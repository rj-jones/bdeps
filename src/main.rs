mod clap;
mod command;
mod error;
mod toml;

use clap::Parser;
use std::process::Command;

fn main() -> Result<(), String> {
    let args = clap::Opts::parse();

    if args.cargo_update {
        let _ = command::execute_command(&mut Command::new("cargo").arg("update"))
            .map_err(|e| e.to_string());
    }

    let dependencies = toml::get_dependencies();

    for dep in dependencies {
        println!("Building {}", dep);
        let _ = command::execute_command(
            &mut Command::new("cargo")
                .arg("build")
                .arg("--package")
                .arg(dep.as_str()),
        )
        .map_err(|e| e.to_string());
    }

    Ok(())
}
