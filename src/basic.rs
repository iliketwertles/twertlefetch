pub mod basic {
    use crate::pack::pack;
    use colored::*;
    use std::env;
    use std::fs;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::io::Result;
    use uname_rs::Uname;

    pub fn dist() -> String {
        let os_release = File::open("/etc/os-release");
        let file = match os_release {
            Ok(file) => file,
            Err(e) => panic!("Unable to open file: {}", e),
        };
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                match line.find("NAME=") {
                    Some(_) => {
                        let stripped = line.trim_start_matches("NAME=").replace('\"', "");
                        return stripped;
                    }
                    None => {}
                }
            }
        }
        String::from("Unknown")
    }
    pub fn desk() -> String {
        let ret = match env::var("XDG_CURRENT_DESKTOP") {
            Ok(val) => val,
            Err(_e) => match env::var("DESKTOP_SESSION") {
                Ok(val) => val,
                Err(_e) => "None?".to_string(),
            },
        };
        ret
    }

    /*pub fn kern() -> String {
        let data = fs::read_to_string("/proc/version").unwrap();
        match data.find("version") {
            Some(_) => {
                let stripped = data
                    .trim_start_matches("Linux version ")
                    .split('(')
                    .next()
                    .unwrap()
                    .trim();
                stripped.to_string()
            }
            None => "idk bro".to_string(),
        }
    }*/

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
