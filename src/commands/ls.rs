use anyhow::Result;
   use crate::lxc;

   pub fn run() -> Result<()> {
       let list = lxc::lxc_list()?;
       println!("{}", list);
       Ok(())
   }