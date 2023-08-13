pub mod gpu {
    use devices::Devices;
    use which::which;
    pub fn gpu() -> String {
        // checking for devices deps
        match which("lspci") {
            Ok(_) => {
                match which("lsusb") {
                    Ok(_) => {},
                    Err(_) => return "[install lsusb to fetch gpu]".to_string()
                }
            },
            Err(_) => return "[install lspci to fetch gpu]".to_string()
        }
        match Devices::get() {
            Ok(devices) => {
                for device in devices {
                    if device.class() == "VGA compatible controller" {
                        return device.product().to_string()
                    }
                }
            },
            Err(_) => {return "uhh".to_string()}
        }
        return "no gpu :pointlaugh:".to_string()
    }
}
