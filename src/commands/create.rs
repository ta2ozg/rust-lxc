use anyhow::Result;
   use crate::lxc;

   pub fn run(name: &str) -> Result<()> {
       lxc::lxc_create(name)?;
       println!("Container {} created", name);
       Ok(())
   }