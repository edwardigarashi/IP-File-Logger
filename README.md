# IP File Logger

A Rust program that logs the current global IP address and hostname of the network to a CSV file. It only appends a new record if the IP address or hostname has changed since the last recorded entry.

## Features

- **Records Global IP Address**: Retrieves the current global IP address using the [ipify](https://www.ipify.org) API.
- **Logs Hostname**: Logs the local hostname of the machine.
- **Conditional Logging**: Skips logging if the current IP address and hostname match the latest entry in the CSV file.
- **Customizable Output**: The CSV file path is provided as a command-line argument.

## Usage

### Running the Program

To run the program, compile it and provide the path to the CSV file as an argument:

```sh
cargo run --release <path_to_csv>
```
### For example:
```sh
cargo run --release /path/to/ip_log.csv
```
## Cross-Compilation for Windows
#### To compile a Windows executable from a Linux system (like Ubuntu), use the following command:
```sh
cargo build --release --target x86_64-pc-windows-gnu
```
#### The resulting executable can be found in the target/x86_64-pc-windows-gnu/release directory.

### Testing
#### Run the tests using the following command:
```sh
cargo test
```
### Dependencies
- **reqwest**: For HTTP requests to fetch the global IP address.
- **csv**: For writing to and reading from CSV files.
- **chrono**: For working with date and time.

### Example Output
#### The CSV file will contain records in the following format:
```bash
date,hostname,ip_address
2024-08-05T12:34:56Z,localhost,127.0.0.1
```

### Contributing
#### Contributions are welcome! Please open an issue or submit a pull request.

### License
#### This project is licensed under the MIT License. See the LICENSE file for details.
Author

### Notes

- **Customize the placeholders**: Replace placeholders like `<path_to_csv>`, `Your Name`, and any URLs with your actual information.
- **Expand Sections**: You can expand sections such as "Contributing" and "License" based on your project's needs.
- **Add Additional Information**: Include any other details specific to your project, such as known issues, future features, etc.

This `README.md` serves as a basic template and guide for users and developers interacting with your project.

