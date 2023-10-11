pub use clap::Parser;

#[derive(Parser)]
#[clap(name = "bdeps", version)]
pub struct Opts {
    #[clap(long, default_value = "false")]
    pub release: bool,
    #[clap(long, name = "cargo-update", default_value = "false")]
    pub cargo_update: bool,
}
