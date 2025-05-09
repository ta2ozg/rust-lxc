use rust_lxc::lxc::LxcCommander;
use anyhow::{Result, anyhow};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// Fake mockup data
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
    fn create(&self, name: &str, _template: &str) -> Result<()> {
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
            Some(ContainerState::Running) => {
                Err(anyhow!("Container {} is running, stop first", name))
            }
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
const TEST_TEMPLATE: &str = "ubuntu"; // default template

// Tests
#[test]
fn test_create_container() -> Result<()> {
    let commander = MockLxcCommander::new();

    // Create a container
    let result = commander.create(TEST_CONTAINER_NAME, TEST_TEMPLATE);
    assert!(result.is_ok(), "Failed to create container: {:?}", result);

    // Check new container in list
    let list = commander.list()?;
    assert!(
        list.contains(TEST_CONTAINER_NAME),
        "Container {} not found in list",
        TEST_CONTAINER_NAME
    );

    // Try re-creating container
    let result = commander.create(TEST_CONTAINER_NAME, TEST_TEMPLATE);
    assert!(result.is_err(), "Creating existing container should fail");

    Ok(())
}

#[test]
fn test_start_container() -> Result<()> {
    let commander = MockLxcCommander::new();

    // Create
    commander.create(TEST_CONTAINER_NAME, TEST_TEMPLATE)?;

    // Start
    let result = commander.start(TEST_CONTAINER_NAME);
    assert!(result.is_ok(), "Failed to start container: {:?}", result);

    // List
    let list = commander.list()?;
    assert!(
        list.contains("Running"),
        "Container {} is not running",
        TEST_CONTAINER_NAME
    );

    // Re-start container
    let result = commander.start(TEST_CONTAINER_NAME);
    assert!(result.is_err(), "Starting already running container should fail");

    Ok(())
}

#[test]
fn test_stop_container() -> Result<()> {
    let commander = MockLxcCommander::new();

    // Create and start
    commander.create(TEST_CONTAINER_NAME, TEST_TEMPLATE)?;
    commander.start(TEST_CONTAINER_NAME)?;

    // Stop container
    let result = commander.stop(TEST_CONTAINER_NAME);
    assert!(result.is_ok(), "Failed to stop container: {:?}", result);

    // Check
    let list = commander.list()?;
    assert!(
        list.contains("Stopped"),
        "Container {} is still running",
        TEST_CONTAINER_NAME
    );

    // Re-stop container
    let result = commander.stop(TEST_CONTAINER_NAME);
    assert!(result.is_err(), "Stopping already stopped container should fail");

    Ok(())
}

#[test]
fn test_delete_container() -> Result<()> {
    let commander = MockLxcCommander::new();

    // Create container
    commander.create(TEST_CONTAINER_NAME, TEST_TEMPLATE)?;

    // Delete container
    let result = commander.delete(TEST_CONTAINER_NAME);
    assert!(result.is_ok(), "Failed to delete container: {:?}", result);

    // Check
    let list = commander.list()?;
    assert!(
        !list.contains(TEST_CONTAINER_NAME),
        "Container {} still exists",
        TEST_CONTAINER_NAME
    );

    // Re-delete container
    commander.create(TEST_CONTAINER_NAME, TEST_TEMPLATE)?;
    commander.start(TEST_CONTAINER_NAME)?;
    let result = commander.delete(TEST_CONTAINER_NAME);
    assert!(result.is_err(), "Deleting running container should fail");

    Ok(())
}

#[test]
fn test_ls_containers() -> Result<()> {
    let commander = MockLxcCommander::new();

    // Create multiple containers
    commander.create(TEST_CONTAINER_NAME, TEST_TEMPLATE)?;
    commander.create("another-container", TEST_TEMPLATE)?;

    // Check
    let list = commander.list()?;
    assert!(
        list.contains(TEST_CONTAINER_NAME),
        "Container {} not found in list",
        TEST_CONTAINER_NAME
    );
    assert!(
        list.contains("another-container"),
        "Container another-container not found in list"
    );

    // Delete one and check
    commander.delete(TEST_CONTAINER_NAME)?;
    let list = commander.list()?;
    assert!(
        !list.contains(TEST_CONTAINER_NAME),
        "Deleted container {} still in list",
        TEST_CONTAINER_NAME
    );
    assert!(
        list.contains("another-container"),
        "Container another-container missing from list"
    );

    Ok(())
}

#[test]
fn test_shutdown_container() -> Result<()> {
    let commander = MockLxcCommander::new();

    // Create and start
    commander.create(TEST_CONTAINER_NAME, TEST_TEMPLATE)?;
    commander.start(TEST_CONTAINER_NAME)?;

    // Shutdown
    let result = commander.shutdown(TEST_CONTAINER_NAME);
    assert!(result.is_ok(), "Failed to shutdown container: {:?}", result);

    // Check
    let list = commander.list()?;
    assert!(
        list.contains("Stopped"),
        "Container {} is still running",
        TEST_CONTAINER_NAME
    );

    Ok(())
}