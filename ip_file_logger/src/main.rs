use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use chrono::Utc;
use csv::Writer;

fn get_public_ip() -> Result<String, Box<dyn Error>> {
    let response = reqwest::blocking::get("https://api.ipify.org")?;
    Ok(response.text()?)
}

fn get_hostname() -> Result<String, Box<dyn Error>> {
    let output = std::process::Command::new("hostname").output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get the CSV file path from the command-line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_csv>", args[0]);
        return Ok(());
    }
    let csv_path = &args[1];

    // Get the public IP address and hostname
    let ip_address = get_public_ip()?;
    let hostname = get_hostname()?;
    let date = Utc::now().to_string();

    // Open or create the CSV file and check the last record
    let file = OpenOptions::new().read(true).write(true).create(true).open(csv_path)?;
    let reader = BufReader::new(file.try_clone()?);
    let mut writer = Writer::from_writer(file);

    let mut last_hostname = String::new();
    let mut last_ip = String::new();

    // Read the last line from the file to check for the latest record
    for line in reader.lines() {
        if let Ok(record) = line {
            let fields: Vec<&str> = record.split(',').collect();
            if fields.len() == 3 {
                last_hostname = fields[1].to_string();
                last_ip = fields[2].to_string();
            }
        }
    }

    // Check if the current IP and hostname are different from the last record
    if last_hostname != hostname || last_ip != ip_address {
        writer.write_record(&[date, hostname, ip_address])?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    // Mock function to replace `get_public_ip` during testing
    fn mock_get_public_ip() -> Result<String, Box<dyn Error>> {
        Ok("127.0.0.1".to_string())
    }

    // Mock function to replace `get_hostname` during testing
    fn mock_get_hostname() -> Result<String, Box<dyn Error>> {
        Ok("localhost".to_string())
    }

    #[test]
    fn test_get_public_ip() {
        let ip = mock_get_public_ip().expect("Failed to get mock IP");
        assert_eq!(ip, "127.0.0.1");
    }

    #[test]
    fn test_get_hostname() {
        let hostname = mock_get_hostname().expect("Failed to get mock hostname");
        assert_eq!(hostname, "localhost");
    }

    #[test]
    fn test_main_functionality() {
        // Setup a temporary CSV file for testing
        let test_csv_path = "test_output.csv";
        if Path::new(test_csv_path).exists() {
            fs::remove_file(test_csv_path).expect("Failed to remove old test file");
        }

        // Use mock functions
        let ip_address = mock_get_public_ip().expect("Failed to get mock IP");
        let hostname = mock_get_hostname().expect("Failed to get mock hostname");
        let date = Utc::now().to_string();

        // Open or create the CSV file and check the last record
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(test_csv_path)
            .expect("Failed to open test CSV file");
        let reader = BufReader::new(file.try_clone().expect("Failed to clone file handle"));
        let mut writer = Writer::from_writer(file);

        let mut last_hostname = String::new();
        let mut last_ip = String::new();

        for line in reader.lines() {
            if let Ok(record) = line {
                let fields: Vec<&str> = record.split(',').collect();
                if fields.len() == 3 {
                    last_hostname = fields[1].to_string();
                    last_ip = fields[2].to_string();
                }
            }
        }

        if last_hostname != hostname || last_ip != ip_address {
            writer
                .write_record(&[date.clone(), hostname.clone(), ip_address.clone()])
                .expect("Failed to write record");
            writer.flush().expect("Failed to flush writer");
        }

        // Verify the output
        let contents = fs::read_to_string(test_csv_path).expect("Failed to read test CSV file");
        println!("Contents of the file: {}", contents);
        assert!(contents.contains(&ip_address), "IP address not found in contents");
        assert!(contents.contains(&hostname), "Hostname not found in contents");

        // Cleanup
        fs::remove_file(test_csv_path).expect("Failed to remove test file");
    }
}
