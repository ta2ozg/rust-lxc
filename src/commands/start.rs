use anyhow::Result;
use crate::lxc::lxc_start;

pub fn run(name: &str) -> Result<()> {
    lxc_start(name)?;
    println!("Container {} started", name);
    Ok(())
}