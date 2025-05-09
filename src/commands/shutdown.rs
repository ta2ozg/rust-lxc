use anyhow::Result;
use crate::lxc::lxc_shutdown;

pub fn run(name: &str) -> Result<()> {
    lxc_shutdown(name)?;
    println!("Container {} shutdown", name);
    Ok(())
}