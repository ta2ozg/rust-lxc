use crate::lxc;
use anyhow::Result;

pub fn run(name: &str) -> Result<()> {
    println!("Deleting container: {}", name);
    lxc::lxc_delete(name)?;
    println!("Container {} deleted successfully", name);
    Ok(())
}