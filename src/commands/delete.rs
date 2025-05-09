use anyhow::Result;
use crate::lxc::lxc_delete;

pub fn run(name: &str) -> Result<()> {
    lxc_delete(name)?;
    println!("Container {} deleted", name);
    Ok(())
}