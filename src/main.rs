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

    let dependencies = toml::parse_toml();

    for (name, dep) in dependencies {
        match dep {
            toml::Dependency::Simple(version) => {
                println!("{}:{}", name, version);
                let _ = command::execute_command(
                    &mut Command::new("cargo")
                        .arg("build")
                        .arg("--package")
                        .arg(format!("{}:{}", name, version).as_str()),
                )
                .map_err(|e| e.to_string());
            }
            toml::Dependency::Detailed(details) => {
                let version = details
                    .version
                    .expect(format!("no version specified for {}", name).as_str());
                println!("{}:{}", name, version);
                let _ = command::execute_command(
                    &mut Command::new("cargo")
                        .arg("build")
                        .arg("--package")
                        .arg(format!("{}:{}", name, version).as_str()),
                )
                .map_err(|e| e.to_string());
            }
            _ => (),
        }
    }

    Ok(())
}
