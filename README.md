# NetScan

NetScan is a fast network scanner tool built with the Rust programming language. It uses multiple threads to check many ports at the same time, making it very quick at finding open services on a network.

## Features

- Multi-threaded scanning: It can check many ports at once using a pool of threads.
- Supports different targets: You can scan a single IP address, a hostname, or a CIDR range.
- Custom port selection: You can choose which ports to scan using single numbers or ranges.
- Timeout control: You can set how long the scanner waits for a response from each port.
- JSON output: It can provide results in JSON format for use with other tools.
- Statistics summary: It provides a summary of total, open, closed, and filtered ports after each scan.

## Installation

To build and run this tool, you need to have Rust and Cargo installed on your system.

1. Clone the repository.
2. Navigate to the project directory.
3. Build the project using Cargo:

```bash
cargo build --release
```

The compiled binary will be located in the `target/release` folder.

## Usage

The primary command for the tool is `scan`.

### Basic Scan

To scan the default ports (1-1024) of a single IP address:

```bash
./target/release/netscan scan 192.168.1.1
```

### Scan a Specific Port Range

To scan ports 80, 443, and everything from 1000 to 2000:

```bash
./target/release/netscan scan 192.168.1.1 --ports 80,443,1000-2000
```

### Set a Custom Timeout

To set a connection timeout of 500 milliseconds:

```bash
./target/release/netscan scan 192.168.1.1 --timeout 500
```

### Output in JSON Format

To get the results in JSON format:

```bash
./target/release/netscan scan 192.168.1.1 --json
```

### Verbose Mode

To see detailed debug information during the scan:

```bash
./target/release/netscan scan 192.168.1.1 --verbose
```

## Output Explanation

The tool provides two types of output:

- **Human Readable**: Shows each host and its open ports, followed by a summary of the scan.
- **JSON**: A structured format including the report for each IP and the final statistics.

Port states explained:
- **Open**: The tool successfully connected to the port.
- **Closed**: The connection was refused.
- **Filtered**: The connection timed out (often due to a firewall).

## Project Structure

- `src/main.rs`: The main entry point of the application.
- `src/cli.rs`: Handles command line arguments.
- `src/scanner.rs`: Contains the core logic for connecting to ports.
- `src/target.rs`: Handles parsing of IP addresses and ranges.
- `src/ports.rs`: Handles parsing of port numbers and ranges.
- `src/report.rs`: Formats the results for display.
- `src/stats.rs`: Calculates statistics about the scan.
