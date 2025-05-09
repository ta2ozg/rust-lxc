use anyhow::Result;
use crate::lxc::lxc_create;

pub fn run(name: &str, template: &str) -> Result<()> {
    lxc_create(name, template)?;
    println!("Container {} created with template {}", name, template);
    Ok(())
}