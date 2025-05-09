use anyhow::{Context, Result};
   use std::process::{Command, Output};

   pub trait LxcCommander {
       fn create(&self, name: &str) -> Result<()>;
       fn start(&self, name: &str) -> Result<()>;
       fn stop(&self, name: &str) -> Result<()>;
       fn delete(&self, name: &str) -> Result<()>;
       fn list(&self) -> Result<String>;
       fn shutdown(&self, name: &str) -> Result<()>;
   }

   pub struct RealLxcCommander;

   impl RealLxcCommander {
       fn execute_command(&self, program: &str, args: &[&str]) -> Result<Output> {
           let output = Command::new(program)
               .args(args)
               .output()
               .context(format!("Failed to execute {}", program))?;

           if output.status.success() {
               Ok(output)
           } else {
               let error = String::from_utf8_lossy(&output.stderr);
               Err(anyhow::anyhow!("{} failed: {}", program, error))
           }
       }
   }

   impl LxcCommander for RealLxcCommander {
       fn create(&self, name: &str) -> Result<()> {
           self.execute_command("lxc-create", &["-n", name, "-t", "ubuntu"])?;
           Ok(())
       }

       fn start(&self, name: &str) -> Result<()> {
           self.execute_command("lxc-start", &["-n", name])?;
           Ok(())
       }

       fn stop(&self, name: &str) -> Result<()> {
           self.execute_command("lxc-stop", &["-n", name])?;
           Ok(())
       }

       fn delete(&self, name: &str) -> Result<()> {
           self.execute_command("lxc-destroy", &["-n", name])?;
           Ok(())
       }

       fn list(&self) -> Result<String> {
           let output = self.execute_command("lxc-ls", &["-f"])?;
           let list = String::from_utf8_lossy(&output.stdout);
           Ok(list.to_string())
       }

       fn shutdown(&self, name: &str) -> Result<()> {
           self.execute_command("lxc-stop", &["-n", name])?;
           Ok(())
       }
   }

   pub fn lxc_create(name: &str) -> Result<()> {
       let commander = RealLxcCommander;
       commander.create(name)
   }

   pub fn lxc_start(name: &str) -> Result<()> {
       let commander = RealLxcCommander;
       commander.start(name)
   }

   pub fn lxc_stop(name: &str) -> Result<()> {
       let commander = RealLxcCommander;
       commander.stop(name)
   }

   pub fn lxc_delete(name: &str) -> Result<()> {
       let commander = RealLxcCommander;
       commander.delete(name)
   }

   pub fn lxc_list() -> Result<String> {
       let commander = RealLxcCommander;
       commander.list()
   }

   pub fn lxc_shutdown(name: &str) -> Result<()> {
       let commander = RealLxcCommander;
       commander.shutdown(name)
   }