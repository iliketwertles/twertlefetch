pub mod basic {
    use crate::pack::pack;
    use colored::*;
    use std::env;
    use std::fs;
    use std::io::Result;
    use uname_rs::Uname;

    pub fn dist() -> String {
        let os_release = std::fs::read_to_string("/etc/os-release").expect("couldn't read /etc/os-release");
        for line in os_release.lines() {
            if line.starts_with("NAME=") {
                let distro = line.trim_start_matches("NAME=").replace('\"', "");
                return distro;
            }
        }
        String::from("Unknown")
    }

    pub fn desk() -> String {
        let ret = match env::var("XDG_CURRENT_DESKTOP") {
            Ok(val) => val,
            Err(_) => match env::var("DESKTOP_SESSION") {
                Ok(val) => val,
                Err(_) => "None or unknown".to_string(),
            },
        };
        ret
    }

    pub fn kern() -> Result<String> {
            let uts = Uname::new()?;
            Ok(uts.release)
        }

    pub fn birth() -> u64 {
        let root_meta = fs::metadata("/").unwrap();
        let root_birth = root_meta.created().unwrap().elapsed().unwrap();
        root_birth.as_secs() / (24 * 60 * 60)
    }

    pub fn normal_ascii() {
        let ascii: [&str; 7] = [
            "     .--.",
            &format!("    |o_o |    Distro: {}", dist().cyan()),
            &format!("    |:_/ |    Kernel: {}", kern().unwrap().yellow()),
            &format!(r"   //   \ \   Desktop: {}", desk().red()),
            &format!("  (|     | )  Packages: {}", pack().blue()),
            r" /'\_   _/`\ ",
            &format!(
                r" \___)=(___/  You installed {} days ago",
                birth().to_string()
            ),
        ];
        for line in ascii {
            println!("{}", line);
        }
    }
}
