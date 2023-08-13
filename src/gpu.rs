pub mod gpu {
    use devices::Devices;
    pub fn gpu() -> String {
        match Devices::get() {
            Ok(devices) => {
                for device in devices {
                    if device.class() == "VGA compatible controller" {
                        return device.product().to_string()
                    }
                }
            },
            Err(_) => {}
        }
        return "no gpu :pointlaugh:".to_string()
    }
}