use anyhow::Result;
use crate::lxc::lxc_list;

pub fn run() -> Result<()> {
    let list = lxc_list()?;
    println!("{}", list);
    Ok(())
}