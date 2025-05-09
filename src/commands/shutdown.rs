use crate::lxc;
use anyhow::Result;

pub fn run(name: &str) -> Result<()> {
    println!("Shutting down container: {}", name);
    lxc::lxc_shutdown(name)?;
    println!("Container {} shut down successfully", name);
    Ok(())
}