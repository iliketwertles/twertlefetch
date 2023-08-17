pub mod gpu {
    use devices::Devices;
    use which::which;
    pub fn gpu() -> String {
        let mut gpus : Vec<String> = Vec::new();
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
                        gpus.push(device.product().to_string());
                    }
                }
                match gpus.len() {
                    1 => return gpus[0].clone(),
                    2 => {
                        if gpus[0].eq(&gpus[1]) {
                            return String::from("2x {gpus[0]}")
                        }else {
                            return String::from("2 different gpus :O")
                        }
                    },
                    _ => {return String::from("uhh");}
                }
            },
            Err(_) => {return "uhh".to_string()}
        }
    }
}