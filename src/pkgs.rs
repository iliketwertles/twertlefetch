pub mod pack {
    use std::path::Path;
    use std::fs;
    pub fn pack() -> String {
        if Path::new("/var/db/pkg").is_dir() {
            fs::read_dir("/var/db/pkg")
            .map(|iter| {
                iter.flatten()
                    .flat_map(|entry| fs::read_dir(entry.path()))
                    .flatten()
                    .flatten()
                    .count()
                    .to_string() // convert the count to a String
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
        }else { return "not supported yet :/".to_string() } // REPLACE TO ADD MORE PKG MANAGER SUPPORT!
    }
}