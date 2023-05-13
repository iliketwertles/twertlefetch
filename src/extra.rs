pub mod extra {
    use std::io::Result;
    use std::path::Path;
    use uname_rs::Uname;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use colored::Colorize;
    use std::env;
    use crate::basic::basic::kern;
    use crate::basic::basic::dist;
    use crate::basic::basic::desk;
    use crate::pack::pack;
    use crate::basic::basic::birth;
    
    pub fn pickle_fetch() {
        fn arch() -> Result<String> {
            let uts = Uname::new()?;
            Ok(uts.machine)
        }

        fn cpu() -> String {
            let cpuinfo = File::open("/proc/cpuinfo");
            let file = match cpuinfo {
                Ok(file) => file,
                Err(e) => panic!("uhhhhh(cpuinfo) {}", e),
            };
            let reader = BufReader::new(file);
            for line in reader.lines().flatten() {
                    match line.find("model name") {
                        Some(_) => {
                            let stripped =
                                line.trim_start_matches("model name	: ").replace('\"', "");
                            return stripped;
                        }
                        None => {}
                    }
            }
            String::from("Unknown")
        }

        fn mem() -> String {
            if let Ok(file) = File::open("/proc/meminfo") {
                let reader = BufReader::new(file);
        
                let mut total_memory = 0;
                let mut available_memory = 0;
        
                for line in reader.lines() {
                    if let Ok(line) = line {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            match parts[0] {
                                "MemTotal:" => {
                                    if let Ok(value) = parts[1].parse::<u64>() {
                                        total_memory = value;
                                    }
                                }
                                "MemAvailable:" => {
                                    if let Ok(value) = parts[1].parse::<u64>() {
                                        available_memory = value;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
        
                let used_memory = total_memory - available_memory;
                let used_memory_mb = used_memory / 1024;
                let total_memory_mb = total_memory / 1024;
                let display_memory = format!("{}Mb/{}Mb", used_memory_mb.to_string(), total_memory_mb.to_string());
        
                return display_memory
                
            } else { return "idk".to_string() }
        }

        fn init() -> &'static str {
            if Path::new("/etc/systemd/system.conf").exists() {
                "Systemd"
            } else if Path::new("/etc/init.d").exists() {
                "Openrc"
            } else if Path::new("/etc/runit/1").exists() {
                "Runit"
            } else {
                "fuckin, idek dude"
            }
        }

        fn shell() -> String {
            let ret = match env::var("SHELL") {
                Ok(val) => val,
                Err(_e) => panic!("UHHHHHHHHH({})", _e),
            };
            ret
        }
        let ascii_pickle: [&str; 20] = [
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠊⠁⠈⠉⠐⢦⡀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠰⠃⠀⠾⠶⠖⣢⡀⠀⢛⡄⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⡰⢒⠒⣄⣀⣈⠚⠀⢸⠸⠀",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡎⠀⢧⡈⣀⡏⠠⠤⣳⠀⠸⠀⡆",
            "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡘⠰⡟⢿⣵⡥⠙⣆⡴⠁⠀⠇⢀⠃",
            &format!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠘⢧⣄⡛⢻⠿⡋⣿⠀⠰⠀⢸⠀     Distro: {}", dist().cyan()),
            &format!("⠀⠀⠀⠀⠀⠀⠀⠀⢀⠞⣏⠀⠀⠉⠙⠻⠿⡿⠟⠀⠃⢀⠇⠀     Kernel: {}", kern().yellow()),
            &format!("⠀⠀⠀⠀⠀⠀⠀⠀⡜⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡸⠀⣸⠀⠀     Desktop: {}", desk().red()),
            &format!("⠀⠀⠀⠀⠀⠀⠀⢠⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⢀⠃⠀⠀     Packages: {}", pack().blue()),
            &format!("⠀⠀⠀⠀⠀⠀⢀⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⠀⡸⠀⠀⠀     Arch: {}",arch().unwrap().purple()),
            &format!("⠀⠀⠀⠀⠀⠀⡾⣖⠀⠀⠀⠀⠀⠀⠀⢠⠀⠴⠃⢠⠁⠀⠀⠀     CPU: {}", cpu().green()),
            &format!("⠀⠀⠀⠀⠀⡜⠀⠀⠀⠀⠀⠀⠀⠀⠀⠆⠀⠀⢀⠎⠀⠀⠀⠀     GPU: {}", "PUT GPU HERE"),
            &format!("⠀⠀⠀⢰⠊⠀⠀⡀⠀⠀⠀⠀⠀⠀⠌⢠⠁⠀⡼⠀⠀⠀⠀⠀     Init: {}", init().red()),
            &format!("⠀⠀⢠⠋⠀⢰⡖⠁⠀⠠⠀⠀⠀⡌⢠⠃⠀⡜⠀⠀⠀⠀⠀⠀     Shell: {}", shell().magenta()),
            &format!("⠀⢀⠻⡍⠀⠀⠀⠀⡀⠁⠀⢀⠌⠠⠃⠀⡜⠀⠀⠀⠀⠀⠀⠀     Mem: {}", mem().cyan()),
            "⠀⡎⢰⠁⠀⠀⠀⡀⠀⠀⠠⠃⢔⠁⢀⠎⠁",
            "⢰⢀⠃⠀⠢⠠⠐⠀⠀⡠⠀⠈⠁⡠⠊⠀",
            "⠈⣼⠀⠀⠀⠀⠀⠀⠜⡀⠀⢠⠜⠁",
            "⠀⠘⠣⣤⠀⣀⢔⠡⣊⠠⠚⠁",
            &format!(
                "⠀⠀⠀⠀⠉⠉⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀     You installed {} days ago",
                birth().to_string()
            ),
        ];
        for line in ascii_pickle {
            println!("{}", line);
        }
    }
}
