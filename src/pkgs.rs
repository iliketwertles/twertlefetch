pub mod pack {
    extern crate nix;
    use std::path::Path;
    use std::fs;
    use std::process::Command;

    pub fn pack() -> String {
        if Path::new("/var/db/pkg").is_dir() {
            fs::read_dir("/var/db/pkg")
            .map(|iter| {
                iter.flatten()
                    .flat_map(|entry| fs::read_dir(entry.path()))
                    .flatten()
                    .flatten()
                    .count()
                    .to_string()
            })
            .unwrap_or_else(|_| String::from("Unable to read package directory"))
        }else if Path::new("/var/lib/pacman/local").is_dir() {
            let local = "/var/lib/pacman/local";
            let dir_entries = fs::read_dir(local).unwrap();
            let mut count = 0;
            for entry in dir_entries {
                let entry_path = entry.unwrap().path();
                if entry_path.is_dir() && entry_path != Path::new(local) {
                    count += 1;
                }
            }
            count.to_string()
        }else if Path::new("/etc/apk").is_dir() {
            let output = Command::new("apk")
                .args(&["info", "-v"])
                .output()
                .expect("Failed to execute apk command");
    
            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.split('\n').collect();
    
            let total_packages = lines.len().to_string();
            return total_packages;
        }else if Path::new("/nix/store").is_dir() {
            let home = std::env::var("HOME").expect("$HOME is not set!");
            let profile = format!("{home}/.nix-profile");

            let output = Command::new("nix-store")
                .args(&["-qR", "/run/current-system/sw", profile.as_str()])
                .output()
                .expect("Failed to execute nix-store command");

            let stdout = String::from_utf8_lossy(&output.stdout);
            //println!("{stdout:?}");
            let lines: Vec<&str> = stdout.split('\n').collect();

            let total_packages = lines.len().to_string();
            return total_packages;
        }else if Path::new("/etc/apt").is_dir() {
            let output = Command::new("dpkg-query")
                .arg("-f")
                .arg("'.\n'")
                .arg("-W")
                .output()
                .expect("cant run dpkg-query");
            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.split('\n').collect();
            
            //                                         |
            // I do not know why its 1 package off but V
            let total_packages = lines.len() - 1;
            return total_packages.to_string();
        }else { return "not supported yet :/".to_string() } // REPLACE TO ADD MORE PKG MANAGER SUPPORT!
    }
}