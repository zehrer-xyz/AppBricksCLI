mod commands;
mod config;
mod modules;
mod runtime;

use anyhow::Result;

fn main() -> Result<()> {
    runtime::run()
}
