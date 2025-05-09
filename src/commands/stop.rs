use crate::lxc;
use anyhow::Result;

pub fn run(name: &str) -> Result<()> {
    println!("Stopping container: {}", name);
    lxc::lxc_stop(name)?;
    println!("Container {} stopped successfully", name);
    Ok(())
}