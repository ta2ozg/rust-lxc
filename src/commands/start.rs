use crate::lxc;
use anyhow::Result;

pub fn run(name: &str) -> Result<()> {
    println!("Starting container: {}", name);
    lxc::lxc_start(name)?;
    println!("Container {} started successfully", name);
    Ok(())
}