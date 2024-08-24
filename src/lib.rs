use colored::*;
use serialport::SerialPortType;
use std::io::Write;
use std::time::Duration;

/// Finds and returns the COM port that matches the description "USB-SERIAL CH340".
///
/// # Returns
/// An `Option<String>` containing the port name if found, otherwise `None`.
pub fn find_port() -> Option<String> {
    let description = "USB-SERIAL CH340";
    
    let ports = match serialport::available_ports() {
        Ok(ports) => ports,
        Err(e) => {
            eprintln!("{}", "Failed to list available ports:".red());
            eprintln!("{}", e.to_string().red());
            return None;
        }
    };

    ports.iter().find_map(|p| match &p.port_type {
        SerialPortType::UsbPort(info) => {
            if info.product.as_deref().map_or(false, |desc| desc.contains(description)) {
                Some(p.port_name.clone())
            } else {
                None
            }
        }
        _ => None,
    })
}

/// Sends a movement command to the KMBox through the specified serial port.
///
/// # Parameters
/// - `port_name`: The name of the COM port to use.
/// - `x`: The x-coordinate to move to.
/// - `y`: The y-coordinate to move to.
///
/// # Returns
/// A Result indicating success or failure with a descriptive error message.
pub fn move_command(port_name: String, x: i32, y: i32) -> Result<(), String> {
    let mut port = match serialport::new(&port_name, 115_200)
        .timeout(Duration::from_millis(10))
        .open() {
        Ok(port) => port,
        Err(e) => {
            let error_message = format!("Failed to open port {}: {}", port_name, e);
            eprintln!("{}", "ERROR:".red().bold());
            eprintln!("{}", error_message.red());
            return Err(error_message);
        }
    };
    
    println!("{}", "[+] KMBox Connected on".green().bold());
    println!("{}", port_name.green());

    let command = format!("km.move({}, {})\r\n", x, y);
    if let Err(e) = port.write(command.as_bytes()) {
        let error_message = format!("Failed to write to serial port: {}", e);
        eprintln!("{}", "ERROR:".red().bold());
        eprintln!("{}", error_message.red());
        return Err(error_message);
    }

    println!("{}", "Move command sent:".green().bold());
    println!("{}", command.green());
    Ok(())
}
