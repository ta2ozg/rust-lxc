use anyhow::Result;
use crate::lxc::lxc_stop;

pub fn run(name: &str) -> Result<()> {
    lxc_stop(name)?;
    println!("Container {} stopped", name);
    Ok(())
}