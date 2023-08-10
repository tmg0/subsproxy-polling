# Subsproxy Pollling

This Rust program periodically fetches Xray configuration data from a Subsproxy server and writes it to a local file. It can also execute a specified command after fetching the configuration.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Configuration](#configuration)
  - [Running the Program](#running-the-program)
- [Contributing](#contributing)
- [License](#license)

## Overview

This program utilizes Rust's asynchronous programming features along with the `reqwest` and `tokio` libraries to periodically fetch Xray configuration data from a Subsproxy server. The fetched data is then written to a local file, and an optional command can be executed afterward.

## Features

- Periodically fetches Xray configuration data from a Subsproxy server.
- Writes the fetched data to a local file.
- Optional execution of a specified command.

## Getting Started

### Prerequisites

- Rust programming language: Install Rust by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository:

  ```bash
  git clone https://github.com/zekunjin/subsproxy-polling.git
  ```

2. Navigate to the project directory:

   ```bash
   cd subsproxy-xray-config-fetcher
   ```

3. Build the project:

   ```bash
   cargo build --release
   ```

### Configuration

1. Rename the config.sample.toml file to config.toml.

2. Edit the config.toml file and configure the following settings:
Set the Subsproxy server URL.

- Set the Subsproxy server URL.
- Specify the Xray configuration file path.
- Set the command to execute (optional).

### Running the Program

Run the program using the following command:

```bash
cargo run --release
```

The program will periodically fetch the Xray configuration data from the specified Subsproxy server and write it to the specified file. If a command is configured, it will be executed after fetching the data.

Press `Ctrl+C` to stop the program.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please feel free to open an issue or a pull request on the GitHub repository.

## License

Made with 💛
