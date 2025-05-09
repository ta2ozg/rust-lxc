
# rust-lxc

`rust-lxc` is a binding and managing LXC written in Rust. This project provides an API to manage LXC containers using `liblxc`, leveraging FFI to call `liblxc` functions from Rust.

## Why Rust Bindings for liblxc?

LXC is a powerful container technology that allows us to create lightweight virtual machines on Linux. However, there wasn't a direct Rust API to interact with LXC. This project provides bindings for `liblxc` in Rust, allowing us to manage containers and integrate LXC functionality within Rust projects.

By combining Rust's memory safety, concurrency features and `liblxc`, we can efficiently and securely manage LXC containers.

## Features

With `rust-lxc`, you can perform the following operations:

- **Create a container** (`create`)
- **Start a container** (`start`)
- **Stop a container** (`stop`)
- **Delete a container** (`delete`)
- **List all containers** (`ls`)
- **Shutdown a container** (`shutdown`)

## Dependencies

- Rust 2024 Edition
- `clap` (for the CLI)
- `anyhow` (for error handling)
- `liblxc` (C-based library for LXC containers)

## Installation

### 1. Clone the project

```bash
git clone https://github.com/yourusername/rust-lxc.git
cd rust-lxc
```

### 2. Install dependencies

To install the dependencies, run:

```bash
cargo build
```

### 3. Use the commands

After building the project, you can manage containers with the following commands:

- **Create a container:**

```bash
cargo run -- create --name mycontainer
```

- **Start a container:**

```bash
cargo run -- start --name mycontainer
```

- **Stop a container:**

```bash
cargo run -- stop --name mycontainer
```

- **Delete a container:**

```bash
cargo run -- delete --name mycontainer
```

- **List containers:**

```bash
cargo run -- ls
```

- **Shutdown a container:**

```bash
cargo run -- shutdown --name mycontainer
```

## Understanding the Code

### File Structure

- **`lib.rs`**: This file contains the core logic for interacting with `liblxc`. FFI is used to interface with the `liblxc` library and container operations are defined here.
- **`cli.rs`**: This file defines the CLI using `clap`. Commands and arguments are parsed here.
- **`lxc.rs`**: All operations related to LXC containers are defined in this file. Creating, starting, stopping, and deleting containers happen here.
- **`main.rs`**: This is the entry point of the program. It processes the CLI commands and calls the appropriate functions.

### FFI (Foreign Function Interface)

FFI is used to bridge Rust with the C-based `liblxc` library. This allows Rust to directly call `liblxc` functions and manage LXC containers.

### Commands

The following commands are available in this project:

- `create`: Creates a new LXC container.
- `delete`: Deletes an existing container.
- `start`: Starts a container.
- `stop`: Stops a container.
- `ls`: Lists all available containers.
- `shutdown`: Shuts down a container.

### Tests

Integration tests are included in the `tests` directory to verify the functionality of container creation, starting, stopping, and deletion.

### Running Tests

To run the tests:

```bash
cargo test
```

## Contributing

If you would like to contribute to this project, feel free to submit pull requests or open issues for bugs or feature suggestions.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more information.
