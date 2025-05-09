use rust_lxc::lxc::LxcCommander;
   use anyhow::{Result, anyhow};
   use std::cell::RefCell;
   use std::collections::HashMap;
   use std::rc::Rc;

   // fake mockup data
   #[derive(Default)]
   struct MockLxcCommander {
       containers: Rc<RefCell<HashMap<String, ContainerState>>>,
   }

   #[derive(Clone, PartialEq, Debug)]
   enum ContainerState {
       Created,
       Running,
       Stopped,
       Deleted,
   }

   impl MockLxcCommander {
       fn new() -> Self {
           MockLxcCommander {
               containers: Rc::new(RefCell::new(HashMap::new())),
           }
       }
   }

   impl LxcCommander for MockLxcCommander {
       fn create(&self, name: &str) -> Result<()> {
           let mut containers = self.containers.borrow_mut();
           if containers.contains_key(name) {
               return Err(anyhow!("Container {} already exists", name));
           }
           containers.insert(name.to_string(), ContainerState::Created);
           Ok(())
       }

       fn start(&self, name: &str) -> Result<()> {
           let mut containers = self.containers.borrow_mut();
           match containers.get(name) {
               Some(ContainerState::Created) | Some(ContainerState::Stopped) => {
                   containers.insert(name.to_string(), ContainerState::Running);
                   Ok(())
               }
               Some(ContainerState::Running) => Err(anyhow!("Container {} already running", name)),
               _ => Err(anyhow!("Container {} does not exist", name)),
           }
       }

       fn stop(&self, name: &str) -> Result<()> {
           let mut containers = self.containers.borrow_mut();
           match containers.get(name) {
               Some(ContainerState::Running) => {
                   containers.insert(name.to_string(), ContainerState::Stopped);
                   Ok(())
               }
               Some(ContainerState::Stopped) => Err(anyhow!("Container {} already stopped", name)),
               _ => Err(anyhow!("Container {} does not exist or is not running", name)),
           }
       }

       fn delete(&self, name: &str) -> Result<()> {
           let mut containers = self.containers.borrow_mut();
           match containers.get(name) {
               Some(ContainerState::Created) | Some(ContainerState::Stopped) => {
                   containers.remove(name);
                   Ok(())
               }
               Some(ContainerState::Running) => Err(anyhow!("Container {} is running, stop first", name)),
               _ => Err(anyhow!("Container {} does not exist", name)),
           }
       }

       fn list(&self) -> Result<String> {
           let containers = self.containers.borrow();
           let list: Vec<String> = containers
               .iter()
               .filter(|(_, state)| **state != ContainerState::Deleted)
               .map(|(name, state)| format!("{} {:?}", name, state))
               .collect();
           Ok(list.join("\n"))
       }

       fn shutdown(&self, name: &str) -> Result<()> {
           self.stop(name)
       }
   }

   const TEST_CONTAINER_NAME: &str = "test-container";

   // Tests
   #[test]
   fn test_create_container() -> Result<()> {
       let commander = MockLxcCommander::new();
       
       // create a Container
       let result = commander.create(TEST_CONTAINER_NAME);
       assert!(result.is_ok(), "Failed to create container: {:?}", result);

       // check new Container a list
       let list = commander.list()?;
       assert!(list.contains(TEST_CONTAINER_NAME), "Container {} not found in list", TEST_CONTAINER_NAME);
       
       // try re-create Container  
       let result = commander.create(TEST_CONTAINER_NAME);
       assert!(result.is_err(), "Creating existing container should fail");
       
       Ok(())
   }

   #[test]
   fn test_start_container() -> Result<()> {
       let commander = MockLxcCommander::new();
       
       // create
       commander.create(TEST_CONTAINER_NAME)?;

       // start
       let result = commander.start(TEST_CONTAINER_NAME);
       assert!(result.is_ok(), "Failed to start container: {:?}", result);

       // list
       let list = commander.list()?;
       assert!(list.contains("Running"), "Container {} is not running", TEST_CONTAINER_NAME);

       // re-start container
       let result = commander.start(TEST_CONTAINER_NAME);
       assert!(result.is_err(), "Starting already running container should fail");

       Ok(())
   }

   #[test]
   fn test_stop_container() -> Result<()> {
       let commander = MockLxcCommander::new();
       
       // create a container and start
       commander.create(TEST_CONTAINER_NAME)?;
       commander.start(TEST_CONTAINER_NAME)?;

       // stop container
       let result = commander.stop(TEST_CONTAINER_NAME);
       assert!(result.is_ok(), "Failed to stop container: {:?}", result);

       // check
       let list = commander.list()?;
       assert!(list.contains("Stopped"), "Container {} is still running", TEST_CONTAINER_NAME);

       // re-stop container
       let result = commander.stop(TEST_CONTAINER_NAME);
       assert!(result.is_err(), "Stopping already stopped container should fail");

       Ok(())
   }

   #[test]
   fn test_delete_container() -> Result<()> {
       let commander = MockLxcCommander::new();
       
       // create container
       commander.create(TEST_CONTAINER_NAME)?;

       // delete container
       let result = commander.delete(TEST_CONTAINER_NAME);
       assert!(result.is_ok(), "Failed to delete container: {:?}", result);

       // check
       let list = commander.list()?;
       assert!(!list.contains(TEST_CONTAINER_NAME), "Container {} still exists", TEST_CONTAINER_NAME);

       // re-delete container
       commander.create(TEST_CONTAINER_NAME)?;
       commander.start(TEST_CONTAINER_NAME)?;
       let result = commander.delete(TEST_CONTAINER_NAME);
       assert!(result.is_err(), "Deleting running container should fail");

       Ok(())
   }

   #[test]
   fn test_ls_containers() -> Result<()> {
       let commander = MockLxcCommander::new();
       
       // create multi-age
       commander.create(TEST_CONTAINER_NAME)?;
       commander.create("another-container")?;

       // check
       let list = commander.list()?;
       assert!(list.contains(TEST_CONTAINER_NAME), "Container {} not found in list", TEST_CONTAINER_NAME);
       assert!(list.contains("another-container"), "Container another-container not found in list");

       // delete one and check
       commander.delete(TEST_CONTAINER_NAME)?;
       let list = commander.list()?;
       assert!(!list.contains(TEST_CONTAINER_NAME), "Deleted container {} still in list", TEST_CONTAINER_NAME);
       assert!(list.contains("another-container"), "Container another-container missing from list");

       Ok(())
   }

   #[test]
   fn test_shutdown_container() -> Result<()> {
       let commander = MockLxcCommander::new();
       
       // create and start
       commander.create(TEST_CONTAINER_NAME)?;
       commander.start(TEST_CONTAINER_NAME)?;

       // shutdown
       let result = commander.shutdown(TEST_CONTAINER_NAME);
       assert!(result.is_ok(), "Failed to shutdown container: {:?}", result);

       // check
       let list = commander.list()?;
       assert!(list.contains("Stopped"), "Container {} is still running", TEST_CONTAINER_NAME);

       Ok(())
   }