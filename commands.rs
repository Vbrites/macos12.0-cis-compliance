// Structure to store commands
pub struct BashCommand {
    name: &'static str,                     // Command name
    description: &'static str,              // Command description
    executable: &'static str,               // Path to the executable
    args: Vec<String>,                      // Fixed arguments
    dynamic_args: Option<Box<dyn Fn() -> Vec<String>>>, // Logic for dynamic arguments
}

impl BashCommand {
    pub fn execute(&self) {
        let mut args = self.args.clone();

        // Adds dynamic arguments if available
        if let Some(dynamic_fn) = &self.dynamic_args {
            args.extend(dynamic_fn());
        }

        println!("============================================");
        println!("Executing Command: '{}'", self.name);
        println!("Description: {}", self.description);
        println!("Executable: {}", self.executable);
        println!("Arguments: {:?}", args);
        println!("============================================");

        // Check if the command needs input ("yes") for stdin
        let needs_input = self.name == "c17_disable_remote_login";

        let output = if needs_input {
            // Send "yes" to stdin for this specific command
            let mut child = std::process::Command::new(&self.executable)
                .args(&args)
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .expect("Failed to start the command");

            // Write "yes" to stdin
            if let Some(stdin) = child.stdin.as_mut() {
                use std::io::Write;
                stdin.write_all(b"yes\n").expect("Failed to write to stdin");
            }

            child.wait_with_output().expect("Failed to read output")
        } else {
            // Regular execution for commands without stdin requirements
            std::process::Command::new(&self.executable)
                .args(&args)
                .output()
                .expect("Failed to execute the command")
        };

        // Process the output
        if output.status.success() {
            println!("--------------------------------------------");
            println!("✅ Command '{}' executed successfully.", self.name);
            println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
            println!("--------------------------------------------");
        } else {
            eprintln!("--------------------------------------------");
            eprintln!("❌ Error while executing '{}'.", self.name);
            eprintln!("Error Output:\n{}", String::from_utf8_lossy(&output.stderr));
            eprintln!("--------------------------------------------");
        }
    }
}


pub fn c1_enable_os_autoupdate() -> BashCommand {
    BashCommand {
        name: "c1_enable_os_autoupdate", // Command name
        description: "1.6 Ensure Install of macOS Updates Is Enabled", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "/usr/bin/defaults".to_string(), // First part of the command
            "write".to_string(), // Sub-command
            "/Library/Preferences/com.apple.SoftwareUpdate".to_string(), // Preference to modify
            "AutomaticallyInstallMacOSUpdates".to_string(), // Key to modify
            "-bool".to_string(), // Boolean flag
            "true".to_string(), // Value to set
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c2_ensure_apple_software_is_current() -> BashCommand {
    BashCommand {
        name: "c2_ensure_apple_software_is_current", // Command name
        description: "1.1 Ensure All Apple-provided Software Is Current.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "/usr/sbin/softwareupdate".to_string(), // First part of the command
            "-l".to_string(), // List available updates
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c3_enable_auto_update() -> BashCommand {
    BashCommand {
        name: "c3_enable_auto_update", // Command name
        description: "1.2 Ensure Auto Update Is Enabled.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "/usr/bin/defaults".to_string(), // First part of the command
            "write".to_string(), // Sub-command
            "/Library/Preferences/com.apple.SoftwareUpdate".to_string(), // Preference to modify
            "AutomaticCheckEnabled".to_string(), // Key to modify
            "-bool".to_string(), // Boolean flag
            "true".to_string(), // Value to set
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c4_enable_download_new_updates() -> BashCommand {
    BashCommand {
        name: "c4_enable_download_new_updates", // Command name
        description: "1.3 Ensure Download New Updates When Available Is Enabled.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "/usr/bin/defaults".to_string(), // First part of the command
            "write".to_string(), // Sub-command
            "/Library/Preferences/com.apple.SoftwareUpdate".to_string(), // Preference to modify
            "AutomaticDownload".to_string(), // Key to modify
            "-bool".to_string(), // Boolean flag
            "true".to_string(), // Value to set
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c5_enable_app_update_installation() -> BashCommand {
    BashCommand {
        name: "c5_enable_app_update_installation", // Command name
        description: "1.4 Ensure Installation of App Update Is Enabled.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "/usr/bin/defaults".to_string(), // First part of the command
            "write".to_string(), // Sub-command
            "/Library/Preferences/com.apple.commerce".to_string(), // Preference to modify
            "AutoUpdate".to_string(), // Key to modify
            "-bool".to_string(), // Boolean flag
            "true".to_string(), // Value to set
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c6_1_enable_system_data_files() -> BashCommand {
    BashCommand {
        name: "c6_1_enable_system_data_files", // Command name
        description: "1.5.1 Ensure System Data Files Are Downloaded Automatically.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "/usr/bin/defaults".to_string(), // First command
            "write".to_string(), // Sub-command
            "/Library/Preferences/com.apple.SoftwareUpdate".to_string(), // Preference to modify
            "ConfigDataInstall".to_string(), // Key to modify
            "-bool".to_string(), // Boolean flag
            "true".to_string(), // Value to set
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c6_2_enable_security_updates() -> BashCommand {
    BashCommand {
        name: "c6_2_enable_security_updates", // Command name
        description: "1.5.2 Ensure Security Updates Are Downloaded Automatically.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "/usr/bin/defaults".to_string(), // First command
            "write".to_string(), // Sub-command
            "/Library/Preferences/com.apple.SoftwareUpdate".to_string(), // Preference to modify
            "CriticalUpdateInstall".to_string(), // Key to modify
            "-bool".to_string(), // Boolean flag
            "true".to_string(), // Value to set
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c7_1_disable_bluetooth() -> BashCommand {
    BashCommand {
        name: "c7_1_disable_bluetooth", // Command name
        description: "1.5.1 Ensure Bluetooth is disabled if no devices are paired.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "/usr/bin/defaults".to_string(), // First part of the command
            "write".to_string(), // Sub-command
            "/Library/Preferences/com.apple.Bluetooth".to_string(), // Preference to modify
            "ControllerPowerState".to_string(), // Key to modify
            "-int".to_string(), // Integer flag
            "0".to_string(), // Value to set
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c7_2_restart_bluetooth_daemon() -> BashCommand {
    BashCommand {
        name: "c7_2_restart_bluetooth_daemon", // Command name
        description: "1.5.2 Restart the Bluetooth daemon.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "killall".to_string(), // Command to terminate the process
            "-HUP".to_string(), // Hang-up signal
            "bluetoothd".to_string(), // Target daemon
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c8_show_bluetooth_status() -> Vec<BashCommand> {
    // Subfunction 1: Fetch the list of users
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        let name = std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect();

        println!("Users= {:?}", name);
        return name
    }

    // Subfunction 2: Create and execute a BashCommand for a user
    fn create_bluetooth_status_command(username: &str) -> BashCommand {
        BashCommand {
            name: &"c8_show_bluetooth_status_for_everyuser", // Concatenate strings directly
            description: "1.5 Ensure Show Bluetooth Status in Menu Bar is Enabled.", // Command description
            executable: "sudo", // Path to the executable
            args: vec![
                "-u".to_string(), // Switch user
                username.to_string(), // Username
                "defaults".to_string(),
                "-currentHost".to_string(),
                "write".to_string(),
                "com.apple.controlcenter.plist".to_string(),
                "Bluetooth".to_string(),
                "-int".to_string(),
                "18".to_string(),
            ],
            dynamic_args: None, // No dynamic arguments
        }
    }

    // Generate commands for all users
    fetch_users()
        .iter()
        .map(|user| create_bluetooth_status_command(user))
        .collect()
}

pub fn c9_1_set_timezone() -> BashCommand {
    BashCommand {
        name: "c9_1_set_timezone", // Command name
        description: "2.2.1 Ensure Set Timezone to America/Sao_Paulo.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "/usr/sbin/systemsetup".to_string(),
            "-settimezone".to_string(),
            "America/Sao_Paulo".to_string(),
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c9_2_set_network_time_server() -> BashCommand {
    BashCommand {
        name: "c9_2_set_network_time_server", // Command name
        description: "2.2.1 Ensure Set Network Time Server to time.apple.com.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "/usr/sbin/systemsetup".to_string(),
            "-setnetworktimeserver".to_string(),
            "time.apple.com".to_string(),
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c9_3_enable_network_time() -> BashCommand {
    BashCommand {
        name: "c9_3_enable_network_time", // Command name
        description: "2.2.1 Ensure Using Network Time is Enabled.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "/usr/sbin/systemsetup".to_string(),
            "-setusingnetworktime".to_string(),
            "on".to_string(),
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c10_sync_time() -> BashCommand {
    BashCommand {
        name: "c10_sync_time", // Command name
        description: "2.2.2 Ensure Time Is Set Within Appropriate Limits.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "sntp".to_string(),
            "-sS".to_string(),
            "time.apple.com".to_string(),
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c11_set_screensaver_inactivity_interval() -> Vec<BashCommand> {
    // Subfunction: Fetch the list of users
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    // Subfunction: Create a BashCommand for a specific user
    fn create_screensaver_command(username: &str) -> BashCommand {
        BashCommand {
            name: "c11_set_screensaver_inactivity_for_everyuser",
            description: "2.3.1 Ensure an Inactivity Interval of 20 Minutes Or Less for the Screen Saver Is Enabled.",
            executable: "sudo",
            args: vec![
                "-u".to_string(),
                username.to_string(),
                "/usr/bin/defaults".to_string(),
                "-currentHost".to_string(),
                "write".to_string(),
                "com.apple.screensaver".to_string(),
                "idleTime".to_string(),
                "-int".to_string(),
                "600".to_string(), // 10 minutes in seconds
            ],
            dynamic_args: None,
        }
    }

    // Fetch the list of users and generate commands
    fetch_users()
        .iter()
        .map(|user| create_screensaver_command(user))
        .collect()
}

pub fn c12_ensure_secure_screensaver_corners() -> Vec<BashCommand> {
    // Subfunction: Fetch the list of users
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    // Subfunctions: Create BashCommand for each corner
    fn configure_top_left_corner(username: &str) -> BashCommand {
        BashCommand {
            name: "c12_secure_top_left_corner_for_everyuser",
            description: "2.3.2 Ensure Screen Saver Top-Left Corner Is Secure.",
            executable: "sudo",
            args: vec![
                "-u".to_string(),
                username.to_string(),
                "/usr/bin/defaults".to_string(),
                "write".to_string(),
                "com.apple.dock".to_string(),
                "wvous-tl-corner".to_string(),
                "-int".to_string(),
                "0".to_string(),
            ],
            dynamic_args: None,
        }
    }

    fn configure_bottom_left_corner(username: &str) -> BashCommand {
        BashCommand {
            name: "c12_secure_top_left_corner_for_everyuser",
            description: "2.3.2 Ensure Screen Saver Bottom-Left Corner Is Secure.",
            executable: "sudo",
            args: vec![
                "-u".to_string(),
                username.to_string(),
                "/usr/bin/defaults".to_string(),
                "write".to_string(),
                "com.apple.dock".to_string(),
                "wvous-bl-corner".to_string(),
                "-int".to_string(),
                "0".to_string(),
            ],
            dynamic_args: None,
        }
    }

    fn configure_top_right_corner(username: &str) -> BashCommand {
        BashCommand {
            name: "c12_secure_top_left_corner_for_everyuser",
            description: "2.3.2 Ensure Screen Saver Top-Right Corner Is Secure.",
            executable: "sudo",
            args: vec![
                "-u".to_string(),
                username.to_string(),
                "/usr/bin/defaults".to_string(),
                "write".to_string(),
                "com.apple.dock".to_string(),
                "wvous-tr-corner".to_string(),
                "-int".to_string(),
                "0".to_string(),
            ],
            dynamic_args: None,
        }
    }

    fn configure_bottom_right_corner(username: &str) -> BashCommand {
        BashCommand {
            name: "c12_secure_top_left_corner_for_everyuser",
            description: "2.3.2 Ensure Screen Saver Bottom-Right Corner Is Secure.",
            executable: "sudo",
            args: vec![
                "-u".to_string(),
                username.to_string(),
                "/usr/bin/defaults".to_string(),
                "write".to_string(),
                "com.apple.dock".to_string(),
                "wvous-br-corner".to_string(),
                "-int".to_string(),
                "0".to_string(),
            ],
            dynamic_args: None,
        }
    }

    // Fetch the list of users and generate commands for all corners
    fetch_users()
        .iter()
        .flat_map(|user| {
            vec![
                configure_top_left_corner(user),
                configure_bottom_left_corner(user),
                configure_top_right_corner(user),
                configure_bottom_right_corner(user),
            ]
        })
        .collect()
}

pub fn c13_disable_remote_apple_events() -> BashCommand {
    BashCommand {
        name: "c13_disable_remote_apple_events", // Command name
        description: "2.4.1 Ensure Remote Apple Events is Disabled.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "systemsetup".to_string(), // Command
            "-setremoteappleevents".to_string(),
            "off".to_string(),
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c14_disable_internet_sharing() -> BashCommand {
    BashCommand {
        name: "c14_disable_internet_sharing", // Command name
        description: "2.4.2 Ensure Internet Sharing is Disabled.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "defaults".to_string(),
            "write".to_string(),
            "/Library/Preferences/SystemConfiguration/com.apple.nat".to_string(),
            "NAT".to_string(),
            "-dict".to_string(),
            "Enabled".to_string(),
            "-int".to_string(),
            "0".to_string(),
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c15_disable_screen_sharing() -> BashCommand {
    BashCommand {
        name: "c15_disable_screen_sharing", // Command name
        description: "2.4.3 Ensure Screen Sharing is Disabled.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "launchctl".to_string(),
            "disable".to_string(),
            "system/com.apple.screensharing".to_string(),
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c16_disable_printer_sharing() -> BashCommand {
    BashCommand {
        name: "c16_disable_printer_sharing", // Command name
        description: "2.4.4 Ensure Printer Sharing is Disabled.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "cupsctl".to_string(),
            "--no-share-printers".to_string(),
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c17_disable_remote_login() -> BashCommand {
    BashCommand {
        name: "c17_disable_remote_login", // Command name
        description: "2.4.5 Ensure Remote Login is Disabled.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "/usr/sbin/systemsetup".to_string(),
            "-setremotelogin".to_string(),
            "off".to_string(),
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c18_disable_dvd_cd_sharing() -> BashCommand {
    BashCommand {
        name: "c18_disable_dvd_cd_sharing", // Command name
        description: "2.4.6 Ensure DVD or CD Sharing is Disabled.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "launchctl".to_string(),
            "disable".to_string(),
            "system/com.apple.ODSAgent".to_string(),
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c19_disable_file_sharing() -> BashCommand {
    BashCommand {
        name: "c19_disable_file_sharing", // Command name
        description: "2.4.8 Ensure File Sharing is Disabled.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "launchctl".to_string(),
            "disable".to_string(),
            "system/com.apple.smbd".to_string(),
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c20_disable_remote_management() -> BashCommand {
    BashCommand {
        name: "c20_disable_remote_management", // Command name
        description: "2.4.9 Ensure Remote Management is Disabled.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "/System/Library/CoreServices/RemoteManagement/ARDAgent.app/Contents/Resources/kickstart".to_string(),
            "-deactivate".to_string(),
            "-stop".to_string(),
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c21_disable_content_caching() -> BashCommand {
    BashCommand {
        name: "c21_disable_content_caching", // Command name
        description: "2.4.10 Ensure Content Caching is Disabled.", // Command description
        executable: "sudo", // Path to the executable
        args: vec![
            "AssetCacheManagerUtil".to_string(),
            "deactivate".to_string(),
        ],
        dynamic_args: None, // No dynamic arguments
    }
}

pub fn c22_disable_airdrop() -> Vec<BashCommand> {
    // Fetch the list of users
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    // Create BashCommand for each user
    fn create_airdrop_command(username: &str) -> BashCommand {
        BashCommand {
            name: "c22_disable_airdrop_for_everyuser",
            description: "2.4.11 Ensure AirDrop is Disabled.",
            executable: "sudo",
            args: vec![
                "-u".to_string(),
                username.to_string(),
                "defaults".to_string(),
                "write".to_string(),
                "com.apple.NetworkBrowser".to_string(),
                "DisableAirDrop".to_string(),
                "-bool".to_string(),
                "true".to_string(),
            ],
            dynamic_args: None,
        }
    }

    // Generate commands for all users
    fetch_users()
        .iter()
        .map(|user| create_airdrop_command(user))
        .collect()
}

pub fn c23_disable_media_sharing() -> Vec<BashCommand> {
    // Fetch the list of users
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    // Create BashCommand for each user
    fn create_media_sharing_command(username: &str) -> BashCommand {
        BashCommand {
            name: "c23_disable_media_sharing_for_everyuser",
            description: "2.4.12 Ensure Media Sharing is Disabled.",
            executable: "sudo",
            args: vec![
                "-u".to_string(),
                username.to_string(),
                "defaults".to_string(),
                "write".to_string(),
                "com.apple.amp.mediasharingd".to_string(),
                "home-sharing-enabled".to_string(),
                "-int".to_string(),
                "0".to_string(),
            ],
            dynamic_args: None,
        }
    }

    // Generate commands for all users
    fetch_users()
        .iter()
        .map(|user| create_media_sharing_command(user))
        .collect()
}

pub fn c24_disable_airplay_receiver() -> Vec<BashCommand> {
    // Fetch the list of users
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    // Create BashCommand for each user
    fn create_airplay_receiver_command(username: &str) -> BashCommand {
        BashCommand {
            name: "c24_disable_airplay_receiver_for_everyuser",
            description: "2.4.13 Ensure AirPlay Receiver is Disabled.",
            executable: "sudo",
            args: vec![
                "-u".to_string(),
                username.to_string(),
                "defaults".to_string(),
                "-currentHost".to_string(),
                "write".to_string(),
                "com.apple.controlcenter.plist".to_string(),
                "AirplayRecieverEnabled".to_string(),
                "-bool".to_string(),
                "false".to_string(),
            ],
            dynamic_args: None,
        }
    }

    // Generate commands for all users
    fetch_users()
        .iter()
        .map(|user| create_airplay_receiver_command(user))
        .collect()
}

pub fn c25_enable_firewall() -> BashCommand {
    BashCommand {
        name: "c25_enable_firewall",
        description: "2.5.2.1 Ensure Firewall is Enabled.",
        executable: "sudo",
        args: vec![
            "/usr/bin/defaults".to_string(),
            "write".to_string(),
            "/Library/Preferences/com.apple.alfglobalstate".to_string(),
            "-int".to_string(),
            "1".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c26_enable_firewall_stealth_mode() -> BashCommand {
    BashCommand {
        name: "c26_enable_firewall_stealth_mode",
        description: "2.5.2.2 Ensure Firewall Stealth Mode is Enabled.",
        executable: "sudo",
        args: vec![
            "/usr/libexec/ApplicationFirewall/socketfilterfw".to_string(),
            "--setstealthmode".to_string(),
            "on".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c27_1_enable_location_services() -> BashCommand {
    BashCommand {
        name: "c27_1_enable_location_services",
        description: "2.5.3 Ensure Location Services is Enabled (Part 1).",
        executable: "sudo",
        args: vec![
            "/usr/bin/defaults".to_string(),
            "write".to_string(),
            "/var/db/locationd/Library/Preferences/ByHost/com.apple.locationd".to_string(),
            "LocationServicesEnabled".to_string(),
            "-bool".to_string(),
            "true".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c27_2_restart_location_services() -> BashCommand {
    BashCommand {
        name: "c27_2_restart_location_services",
        description: "2.5.3 Ensure Location Services is Enabled (Part 2).",
        executable: "sudo",
        args: vec![
            "/bin/launchctl".to_string(),
            "kickstart".to_string(),
            "-k".to_string(),
            "system/com.apple.locationd".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c28_1_disable_diagnostic_data() -> BashCommand {
    BashCommand {
        name: "c28_1_disable_diagnostic_data",
        description: "2.5.5 Ensure Sending Diagnostic and Usage Data to Apple is Disabled (Global Settings).",
        executable: "sudo",
        args: vec![
            "/usr/bin/defaults".to_string(),
            "write".to_string(),
            "/Library/Application Support/CrashReporter/DiagnosticMessagesHistory.plist".to_string(),
            "AutoSubmit".to_string(),
            "-bool".to_string(),
            "false".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c28_2_disable_diagnostic_data_per_user() -> Vec<BashCommand> {
    // Fetch users
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    // Create commands for each user
    fn create_diagnostic_data_command(username: &str) -> BashCommand {
        BashCommand {
            name: "c28_2_disable_diagnostic_data_for_everyuser",
            description: "2.5.5 Ensure Sending Diagnostic and Usage Data to Apple is Disabled (Per User).",
            executable: "sudo",
            args: vec![
                "-u".to_string(),
                username.to_string(),
                "/usr/bin/defaults".to_string(),
                "write".to_string(),
                format!("/Users/{}/Library/Preferences/com.apple.assistant.support", username),
                "'Siri Data Sharing Opt-In Status'".to_string(),
                "-int".to_string(),
                "2".to_string(),
            ],
            dynamic_args: None,
        }
    }

    fetch_users()
        .iter()
        .map(|user| create_diagnostic_data_command(user))
        .collect()
}

pub fn c29_enable_limit_ad_tracking() -> Vec<BashCommand> {
    // Fetch users
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    // Create commands for each user
    fn create_ad_tracking_command(username: &str) -> BashCommand {
        BashCommand {
            name: "c29_enable_limit_ad_tracking_for_everyuser",
            description: "2.5.6 Ensure Limit Ad Tracking is Enabled.",
            executable: "sudo",
            args: vec![
                "-u".to_string(),
                username.to_string(),
                "/usr/bin/defaults".to_string(),
                "write".to_string(),
                format!("/Users/{}/Library/Preferences/com.apple.Adlib.plist", username),
                "allowApplePersonalizedAdvertising".to_string(),
                "-bool".to_string(),
                "false".to_string(),
            ],
            dynamic_args: None,
        }
    }

    fetch_users()
        .iter()
        .map(|user| create_ad_tracking_command(user))
        .collect()
}

pub fn c30_enable_gatekeeper() -> BashCommand {
    BashCommand {
        name: "c30_enable_gatekeeper",
        description: "2.5.7 Ensure Gatekeeper is Enabled.",
        executable: "sudo",
        args: vec![
            "/usr/sbin/spctl".to_string(),
            "--master-enable".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c31_enable_custom_login_message() -> BashCommand {
    BashCommand {
        name: "c31_enable_custom_login_message",
        description: "2.5.8 Ensure a Custom Message for the Login Screen is Enabled.",
        executable: "sudo",
        args: vec![
            "/usr/bin/defaults".to_string(),
            "write".to_string(),
            "/Library/Preferences/com.apple.loginwindow".to_string(),
            "LoginwindowText".to_string(),
            "Access for authorized personnel only. \\nThis system is being monitored.".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c32_ensure_admin_password_for_system_preferences() -> BashCommand {
    BashCommand {
        name: "c32_ensure_admin_password_for_system_preferences",
        description: "2.5.9 Ensure an Administrator Password is Required to Access System-Wide Preferences.",
        executable: "sh",
        args: vec![
            "-c".to_string(),
            r#"/usr/bin/sudo /usr/bin/security authorizationdb read system.preferences > /$HOME/Desktop/system.preferences.plist 2>&1 && /usr/bin/sudo plutil -replace shared -bool false "$HOME/Desktop/system.preferences.plist" && /usr/bin/sudo /usr/bin/security authorizationdb write system.preferences < /$HOME/Desktop/system.preferences.plist 2>&1"#.to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c33_require_password_to_wake() -> BashCommand {
    BashCommand {
        name: "c33_require_password_to_wake",
        description: "2.5.9 Ensure a Password is Required to Wake the Computer From Sleep or Screen Saver is Enabled.",
        executable: "sudo",
        args: vec![
            "/usr/sbin/sysadminctl".to_string(),
            "-screenLock".to_string(),
            "5".to_string(),
            "seconds".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c34_configure_sleep_settings_based_on_cpu() -> BashCommand {
    BashCommand {
        name: "c34_configure_sleep_settings_based_on_cpu",
        description: "2.8.1 Ensure the OS is Not Active When Resuming from Sleep and Display Sleep, Configured Based on CPU Type.",
        executable: "sh",
        args: vec![
            "-c".to_string(),
            r#"
cpu_model=$(/usr/sbin/sysctl -n machdep.cpu.brand_string)
if [[ $cpu_model == *Apple* ]]; then
    /usr/bin/sudo /usr/bin/pmset -a sleep 10
    /usr/bin/sudo /usr/bin/pmset -a displaysleep 15
    /usr/bin/sudo /usr/bin/pmset -a hibernatemode 25
elif [[ $cpu_model == *Intel* ]]; then
    /usr/bin/sudo /usr/bin/pmset -a standbydelaylow 900
    /usr/bin/sudo /usr/bin/pmset -a standbydelayhigh 900
    /usr/bin/sudo /usr/bin/pmset -a highstandbythreshold 600
    /usr/bin/sudo /usr/bin/pmset -a destroyfvkeyonstandby 1
    /usr/bin/sudo /usr/bin/pmset -a hibernatemode 25
else
    echo 'CPU Model not identified'
fi
            "#.to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c35_disable_wake_for_network_access() -> BashCommand {
    BashCommand {
        name: "c35_disable_wake_for_network_access",
        description: "2.8.2 Ensure Wake for Network Access is Disabled.",
        executable: "sudo",
        args: vec![
            "/usr/bin/pmset".to_string(),
            "-a".to_string(),
            "womp".to_string(),
            "0".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c36_disable_powernap_based_on_cpu() -> BashCommand {
    // Identifica o modelo de CPU
    let output = std::process::Command::new("/usr/sbin/sysctl")
        .arg("-n")
        .arg("machdep.cpu.brand_string")
        .output()
        .expect("Failed to retrieve CPU model");

    let cpu_model = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Lógica de decisão com base no modelo de CPU
    if cpu_model.contains("Apple") {
        return BashCommand {
            name: "c36_disable_powernap_for_intel",
            description: "2.8.3 Ensure Power Nap is Disabled for Intel Macs.",
            executable: "echo",
            args: vec!["No action required for Apple CPUs.".to_string()],
            dynamic_args: None,
        }
    } else if cpu_model.contains("Intel") {
        return BashCommand {
            name: "c36_disable_powernap_for_intel",
            description: "2.8.3 Ensure Power Nap is Disabled for Intel Macs.",
            executable: "sudo",
            args: vec![
                "/usr/bin/pmset".to_string(),
                "-a".to_string(),
                "powernap".to_string(),
                "0".to_string(),
            ],
            dynamic_args: None,
        }
    } else {
        return BashCommand {
            name: "c36_disable_powernap_for_intel",
            description: "2.8.3 Ensure Power Nap is Disabled for Intel Macs.",
            executable: "echo",
            args: vec!["No action required for Apple CPUs.".to_string()],
            dynamic_args: None,
        }
    }
}

pub fn c37_enable_security_auditing() -> BashCommand {
    BashCommand {
        name: "c37_enable_security_auditing",
        description: "3.1 Ensure Security Auditing is Enabled.",
        executable: "sudo",
        args: vec![
            "/bin/launchctl".to_string(),
            "load".to_string(),
            "-w".to_string(),
            "/System/Library/LaunchDaemons/com.apple.auditd.plist".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c38_configure_auditing_flags() -> BashCommand {
    let file_path = "/etc/security/audit_control";

    // Abrir o arquivo e ler o conteúdo
    let content = std::fs::read_to_string(file_path).expect("Failed to read the file.");

    // Procurar a linha que começa com "flags:"
    let target_line = content
        .lines()
        .find(|line| line.starts_with("flags:"))
        .map(|line| line.to_string());

    let target_line = match target_line {
        Some(line) => line,
        None => {
            return BashCommand {
                name: "c38_configure_auditing_flags",
                description: "3.2 Ensure Security Auditing Flags for User-Attributable Events Are Configured.",
                executable: "echo",
                args: vec!["Target line starting with 'flags:' not found in the file.".to_string()],
                dynamic_args: None,
            }; // Nada a fazer se as flags já estão configuradas
        }
    };

    // Nova configuração de flags
    let required_flags = "flags:-fm,ad,-ex,aa,-fr,lo,-fw";

    // Verificar se as flags já estão configuradas corretamente
    if target_line == required_flags {
        return BashCommand {
            name: "c38_configure_auditing_flags",
            description: "3.2 Ensure Security Auditing Flags for User-Attributable Events Are Configured.",
            executable: "echo",
            args: vec!["The auditing flags are already configured correctly.".to_string()],
            dynamic_args: None,
        }; // Nada a fazer se as flags já estão configuradas
    }

    // Retornar o comando BashCommand
    BashCommand {
        name: "c38_configure_auditing_flags",
        description: "3.2 Ensure Security Auditing Flags for User-Attributable Events Are Configured.",
        executable: "sudo",
        args: vec![
            "sed".to_string(),
            "-i".to_string(),
            "''".to_string(),
            format!("s/^flags:.*$/{}/", required_flags),
            file_path.to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c39_configure_install_log_retention() -> BashCommand {
    let file_path = "/etc/asl/com.apple.install";

    // Abrir o arquivo e ler o conteúdo
    let content = std::fs::read_to_string(file_path).expect("Failed to read the file.");

    // Procurar a linha que contém "* file /var/log/install.log"
    let target_line = content
        .lines()
        .find(|line| line.contains("* file /var/log/install.log"))
        .map(|line| line.to_string());

    let target_line = match target_line {
        Some(line) => line,
        None => {
            return BashCommand {
                name: "c39_configure_install_log_retention",
                description: "3.3 Ensure install.log is Retained for 365 or More Days and No Maximum Size.",
                executable: "echo",
                args: vec!["Target line not found in the file.".to_string()],
                dynamic_args: None,
            }; // Nada a fazer se as flags já estão configuradas
        }
    };

    // Verificar as flags existentes
    let required_flags = vec!["rotate=seq", "compress", "file_max=50M", "size_only", "ttl=365"];
    let mut missing_flags = vec![];

    for flag in &required_flags {
        if !target_line.contains(flag) {
            missing_flags.push(flag.to_string());
        }
    }

    if missing_flags.is_empty() {
        println!("All required flags are already present.");
        return BashCommand {
            name: "c39_configure_install_log_retention",
            description: "3.3 Ensure install.log is Retained for 365 or More Days and No Maximum Size.",
            executable: "echo",
            args: vec!["All required flags are already present.".to_string()],
            dynamic_args: None,
        }; // Nada a fazer se as flags já estão configuradas
    }

    // Adicionar flags ausentes à linha
    let mut updated_line = target_line.clone();
    for flag in &missing_flags {
        updated_line = format!("{} {}", updated_line, flag);
    }

    // Retornar o comando BashCommand
    BashCommand {
        name: "c39_configure_install_log_retention",
        description: "3.3 Ensure install.log is Retained for 365 or More Days and No Maximum Size.",
        executable: "sudo",
        args: vec![
            "sed".to_string(),
            "-i".to_string(),
            "''".to_string(),
            format!(
                "'s|^.*file /var/log/install.log.*$|{}|'",
                updated_line
            ),
            file_path.to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c40_configure_auditing_retention() -> BashCommand {
    let file_path = "/etc/security/audit_control";

    // Abrir o arquivo e ler o conteúdo
    let content = std::fs::read_to_string(file_path).expect("Failed to read the file.");

    // Procurar a linha que começa com "expire-after:"
    let target_line = content
        .lines()
        .find(|line| line.starts_with("expire-after:"))
        .map(|line| line.to_string());

    let required_setting = "expire-after:60d";

    if let Some(line) = target_line {
        if line == required_setting {
            return BashCommand {
                name: "c40_configure_auditing_retention",
                description: "3.2 Ensure Security Auditing Retention is Enabled.",
                executable: "echo",
                args: vec!["Auditing retention already configured.".to_string()],
                dynamic_args: None,
            };
        }
    }

    BashCommand {
        name: "c40_configure_auditing_retention",
        description: "3.2 Ensure Security Auditing Retention is Enabled.",
        executable: "sudo",
        args: vec![
            "sed".to_string(),
            "-i".to_string(),
            "''".to_string(),
            "s|^expire-after:.*$|expire-after:60d|".to_string(),
            file_path.to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c41_1_chown_audit_control() -> BashCommand {
    BashCommand {
        name: "c41_1_chown_audit_control",
        description: "Ensure audit_control is owned by root:wheel.",
        executable: "sudo",
        args: vec![
            "/usr/sbin/chown".to_string(),
            "-R".to_string(),
            "root:wheel".to_string(),
            "/etc/security/audit_control".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c41_2_chmod_audit_control() -> BashCommand {
    BashCommand {
        name: "c41_2_chmod_audit_control",
        description: "Ensure audit_control permissions are set to prevent access by others.",
        executable: "sudo",
        args: vec![
            "/bin/chmod".to_string(),
            "-R".to_string(),
            "o-rw".to_string(),
            "/etc/security/audit_control".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c41_3_chown_var_audit() -> BashCommand {
    BashCommand {
        name: "c41_3_chown_var_audit",
        description: "Ensure /var/audit is owned by root:wheel.",
        executable: "sudo",
        args: vec![
            "/usr/sbin/chown".to_string(),
            "-R".to_string(),
            "root:wheel".to_string(),
            "/var/audit/".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c41_4_chmod_var_audit() -> BashCommand {
    BashCommand {
        name: "c41_4_chmod_var_audit",
        description: "Ensure /var/audit permissions are set to prevent access by others.",
        executable: "sudo",
        args: vec![
            "/bin/chmod".to_string(),
            "-R".to_string(),
            "o-rw".to_string(),
            "/var/audit/".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c42_1_enable_logging_mode() -> BashCommand {
    BashCommand {
        name: "c42_1_enable_logging_mode",
        description: "Ensure Logging Mode is Enabled.",
        executable: "sudo",
        args: vec![
            "/usr/libexec/ApplicationFirewall/socketfilterfw".to_string(),
            "--setloggingmode".to_string(),
            "on".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c42_2_configure_logging_detail() -> BashCommand {
    BashCommand {
        name: "c42_2_configure_logging_detail",
        description: "Ensure Logging is Configured to Detail Mode.",
        executable: "sudo",
        args: vec![
            "/usr/libexec/ApplicationFirewall/socketfilterfw".to_string(),
            "--setloggingopt".to_string(),
            "detail".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c43_disable_bonjour_advertising() -> BashCommand {
    BashCommand {
        name: "c43_disable_bonjour_advertising",
        description: "4.1 Ensure Bonjour Advertising Services is Disabled.",
        executable: "sudo",
        args: vec![
            "/usr/bin/defaults".to_string(),
            "write".to_string(),
            "/Library/Preferences/com.apple.mDNSResponder.plist".to_string(),
            "NoMulticastAdvertisements".to_string(),
            "-bool".to_string(),
            "true".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c44_disable_http_server() -> BashCommand {
    BashCommand {
        name: "c44_disable_http_server",
        description: "4.2 Ensure HTTP Server is Disabled.",
        executable: "sudo",
        args: vec![
            "/bin/launchctl".to_string(),
            "unload".to_string(),
            "-w".to_string(),
            "/System/Library/LaunchDaemons/org.apache.httpd.plist".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c45_disable_nfs_server() -> BashCommand {
    BashCommand {
        name: "c45_disable_nfs_server",
        description: "4.3 Ensure NFS Server is Disabled.",
        executable: "sudo",
        args: vec![
            "/bin/launchctl".to_string(),
            "disable".to_string(),
            "system/com.apple.nfsd".to_string(),
            "&&".to_string(),
            "/bin/rm".to_string(),
            "/etc/exports".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c46_secure_home_folders() -> Vec<BashCommand> {
    // Buscar os usuários
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    // Criar comandos para cada usuário
    fetch_users()
        .iter()
        .map(|user| BashCommand {
            name: "c46_secure_home_folder_for_everyuser",
            description: "5.1.1 Ensure Home Folders Are Secure.",
            executable: "sudo",
            args: vec![
                "/bin/chmod".to_string(),
                "-R".to_string(),
                "og-rwx".to_string(),
                format!("/Users/{}", user),
            ],
            dynamic_args: None,
        })
        .collect()
}

pub fn c47_enable_amfi() -> BashCommand {
    BashCommand {
        name: "c47_enable_amfi",
        description: "5.1.3 Ensure Apple Mobile File Integrity (AMFI) is Enabled.",
        executable: "sudo",
        args: vec![
            "/usr/sbin/nvram".to_string(),
            "boot-args=".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c47_1_enable_library_validation() -> BashCommand {
    BashCommand {
        name: "c47_1_enable_library_validation",
        description: "Ensure Library Validation is Enabled.",
        executable: "sudo",
        args: vec![
            "/usr/bin/defaults".to_string(),
            "write".to_string(),
            "/Library/Preferences/com.apple.security.libraryvalidation.plist".to_string(),
            "DisableLibraryValidation".to_string(),
            "-bool".to_string(),
            "false".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c48_ensure_permissions_for_system_apps() -> Vec<BashCommand> {
    // Subfunção para buscar aplicativos no diretório /Applications
    fn fetch_apps() -> Vec<String> {
        use std::process::Command;

        let output = Command::new("find")
            .args(["/Applications", "-iname", "*.app", "-type", "d", "-perm", "-2"])
            .output()
            .expect("Failed to find applications with world-writable permissions.");

        let apps = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        if apps.is_empty() {
            println!("No world-writable applications found.");
        }

        apps
    }

    // Criar comandos para cada aplicativo
    fetch_apps()
        .iter()
        .map(|app| BashCommand {
            name: "c48_fix_permissions_for_every_systemdwide_apps",
            description: "Ensure Appropriate Permissions Are Enabled for System Wide Applications.",
            executable: "sudo",
            args: vec![
                "/bin/chmod".to_string(),
                "-R".to_string(),
                "o-w".to_string(),
                app.to_string(),
            ],
            dynamic_args: None,
        })
        .collect()
}

pub fn c49_fix_world_writable_system_files() -> Vec<BashCommand> {
    // Buscar diretórios com permissões inadequadas no System Folder
    fn find_world_writable_system_files() -> Vec<String> {
        use std::process::Command;

        let output = Command::new("find")
            .args([
                "/System/Volumes/Data/System",
                "-type",
                "d",
                "-perm",
                "-2",
                "-exec",
                "grep",
                "-v",
                "Drop Box",
                "{}",
                ";",
            ])
            .output()
            .expect("Failed to find world-writable files in the System folder.");

        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| line.to_string())
            .collect()
    }

    // Criar comandos para corrigir permissões
    find_world_writable_system_files()
        .iter()
        .map(|path| BashCommand {
            name: "c49_fix_permissions_for_every_world_writable_system_files",
            description: "5.1.6 Ensure No World Writable Files Exist in the System Folder.",
            executable: "sudo",
            args: vec![
                "/bin/chmod".to_string(),
                "-R".to_string(),
                "o-w".to_string(),
                path.clone(),
            ],
            dynamic_args: None,
        })
        .collect()
}

pub fn c50_fix_world_writable_library_files() -> Vec<BashCommand> {
    // Buscar diretórios com permissões inadequadas no Library Folder
    fn find_world_writable_library_files() -> Vec<String> {
        use std::process::Command;

        let output = Command::new("find")
            .args([
                "/System/Volumes/Data/Library",
                "-type",
                "d",
                "-perm",
                "-2",
                "-exec",
                "grep",
                "-v",
                "Caches",
                "-v",
                "/Preferences/Audio/Data",
                "{}",
                ";",
            ])
            .output()
            .expect("Failed to find world-writable files in the Library folder.");

        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| line.to_string())
            .collect()
    }

    // Criar comandos para corrigir permissões
    find_world_writable_library_files()
        .iter()
        .map(|path| BashCommand {
            name: "c50_fix_permissions_every_world_writable_library_files",
            description: "5.1.7 Ensure No World Writable Files Exist in the Library Folder.",
            executable: "sudo",
            args: vec![
                "/bin/chmod".to_string(),
                "-R".to_string(),
                "o-w".to_string(),
                path.clone(),
            ],
            dynamic_args: None,
        })
        .collect()
}

pub fn c51_configure_password_account_lockout() -> BashCommand {
    BashCommand {
        name: "c51_configure_password_account_lockout",
        description: "5.2.1 Ensure Password Account Lockout Threshold is Configured.",
        executable: "sudo",
        args: vec![
            "/usr/bin/pwpolicy".to_string(),
            "-n".to_string(),
            "/Local/Default".to_string(),
            "-setglobalpolicy".to_string(),
            "maxFailedLoginAttempts=5".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c52_configure_password_min_length() -> BashCommand {
    BashCommand {
        name: "c52_configure_password_min_length",
        description: "5.2.1 Ensure Password Minimum Length is Configured.",
        executable: "sudo",
        args: vec![
            "/usr/bin/pwpolicy".to_string(),
            "-n".to_string(),
            "/Local/Default".to_string(),
            "-setglobalpolicy".to_string(),
            "minChars=12".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c53_configure_password_requires_alpha() -> BashCommand {
    BashCommand {
        name: "c53_configure_password_requires_alpha",
        description: "5.2.3 Ensure Complex Password Must Contain Alphabetic Characters is Configured.",
        executable: "sudo",
        args: vec![
            "/usr/bin/pwpolicy".to_string(),
            "-n".to_string(),
            "/Local/Default".to_string(),
            "-setglobalpolicy".to_string(),
            "requiresAlpha=1".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c54_configure_password_requires_numeric() -> BashCommand {
    BashCommand {
        name: "c54_configure_password_requires_numeric",
        description: "5.2.4 Ensure Complex Password Must Contain Numeric Character is Configured.",
        executable: "sudo",
        args: vec![
            "/usr/bin/pwpolicy".to_string(),
            "-n".to_string(),
            "/Local/Default".to_string(),
            "-setglobalpolicy".to_string(),
            "requiresNumeric=2".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c55_configure_password_requires_symbol() -> BashCommand {
    BashCommand {
        name: "c55_configure_password_requires_symbol",
        description: "5.2.5 Ensure Complex Password Must Contain Special Character is Configured.",
        executable: "sudo",
        args: vec![
            "/usr/bin/pwpolicy".to_string(),
            "-n".to_string(),
            "/Local/Default".to_string(),
            "-setglobalpolicy".to_string(),
            "requiresSymbol=1".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c56_configure_password_requires_mixed_case() -> BashCommand {
    BashCommand {
        name: "c56_configure_password_requires_mixed_case",
        description: "5.2.6 Ensure Complex Password Must Contain Uppercase and Lowercase Characters is Configured.",
        executable: "sudo",
        args: vec![
            "/usr/bin/pwpolicy".to_string(),
            "-n".to_string(),
            "/Local/Default".to_string(),
            "-setglobalpolicy".to_string(),
            "requiresMixedCase=1".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c57_configure_password_age() -> BashCommand {
    BashCommand {
        name: "c57_configure_password_age",
        description: "5.2.7 Ensure Password Age is Configured.",
        executable: "sudo",
        args: vec![
            "/usr/bin/pwpolicy".to_string(),
            "-n".to_string(),
            "/Local/Default".to_string(),
            "-setglobalpolicy".to_string(),
            "maxMinutesUntilChangePassword=259200".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c58_configure_password_history() -> BashCommand {
    BashCommand {
        name: "c58_configure_password_history",
        description: "5.2.8 Ensure Password History is Configured.",
        executable: "sudo",
        args: vec![
            "/usr/bin/pwpolicy".to_string(),
            "-n".to_string(),
            "/Local/Default".to_string(),
            "-setglobalpolicy".to_string(),
            "usingHistory=15".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c59_configure_sudo_settings() -> BashCommand {
    BashCommand {
        name: "c59_configure_sudo_settings",
        description: "Ensure Sudo Timeout Period and Separate Timestamp are Configured.",
        executable: "sh",
        args: vec![
            "-c".to_string(),
            "{ echo 'Defaults timestamp_timeout=0'; echo 'Defaults timestamp_type=tty'; } | sudo tee /etc/sudoers.d/10_cissudoconfiguration > /dev/null".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c60_update_progress() -> BashCommand {
    BashCommand {
        name: "c60_update_progress",
        description: "Update Progress for Sudo Timeout and Separate Timestamp Configuration.",
        executable: "echo",
        args: vec!["Progress updated.".to_string()],
        dynamic_args: None,
    }
}

pub fn c61_disable_root_account() -> BashCommand {
    BashCommand {
        name: "c61_disable_root_account",
        description: "5.5 Ensure the 'root' Account is Disabled.",
        executable: "sudo",
        args: vec![
            "/usr/sbin/dsenableroot".to_string(),
            "-d".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c62_disable_automatic_login() -> BashCommand {
    BashCommand {
        name: "c62_disable_automatic_login",
        description: "5.6 Ensure Automatic Login is Disabled.",
        executable: "sudo",
        args: vec![
            "/usr/bin/defaults".to_string(),
            "delete".to_string(),
            "/Library/Preferences/com.apple.loginwindow".to_string(),
            "autoLoginUser".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c63_prevent_admin_login_to_locked_session() -> BashCommand {
    BashCommand {
        name: "c63_prevent_admin_login_to_locked_session",
        description: "5.7 Ensure an Administrator Account Cannot Log in to Another User's Active and Locked Session.",
        executable: "sudo",
        args: vec![
            "/usr/bin/security".to_string(),
            "authorizationdb".to_string(),
            "write".to_string(),
            "system.login.screensaver".to_string(),
            "use-login-window-ui".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c63_1_disable_fast_user_switching() -> BashCommand {
    BashCommand {
        name: "c63_1_disable_fast_user_switching",
        description: "Ensure Fast User Switching is Disabled.",
        executable: "sudo",
        args: vec![
            "/usr/bin/defaults".to_string(),
            "write".to_string(),
            "/Library/Preferences/.GlobalPreferences".to_string(),
            "MultipleSessionEnabled".to_string(),
            "-bool".to_string(),
            "false".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c64_remove_password_hints() -> Vec<BashCommand> {
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    fetch_users()
        .iter()
        .map(|user| BashCommand {
            name: "c64_remove_password_hint_for_everyuser",
            description: "Ensure User Accounts Do Not Have a Password Hint.",
            executable: "sudo",
            args: vec![
                "/usr/bin/dscl".to_string(),
                ".".to_string(),
                "-delete".to_string(),
                format!("/Users/{}", user),
                "hint".to_string(),
            ],
            dynamic_args: None,
        })
        .collect()
}

pub fn c65_enable_secure_keyboard_entry() -> Vec<BashCommand> {
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    fetch_users()
        .iter()
        .map(|user| BashCommand {
            name: "c65_enable_secure_keyboard_entry_for_everyuser",
            description: "Ensure Secure Keyboard Entry in Terminal.app is Enabled.",
            executable: "sudo",
            args: vec![
                "-u".to_string(),
                user.to_string(),
                "/usr/bin/defaults".to_string(),
                "write".to_string(),
                "-app".to_string(),
                "Terminal".to_string(),
                "SecureKeyboardEntry".to_string(),
                "-bool".to_string(),
                "true".to_string(),
            ],
            dynamic_args: None,
        })
        .collect()
}

pub fn c66_enable_login_name_password_display() -> BashCommand {
    BashCommand {
        name: "c66_enable_login_name_password_display",
        description: "Ensure Login Windows Displays as Name and Password is Enabled.",
        executable: "sudo",
        args: vec![
            "/usr/bin/defaults".to_string(),
            "write".to_string(),
            "/Library/Preferences/com.apple.loginwindow".to_string(),
            "SHOWFULLNAME".to_string(),
            "-bool".to_string(),
            "true".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c67_disable_password_hints() -> BashCommand {
    BashCommand {
        name: "c67_disable_password_hints",
        description: "Ensure Show Password Hints is Disabled.",
        executable: "sudo",
        args: vec![
            "/usr/bin/defaults".to_string(),
            "write".to_string(),
            "/Library/Preferences/com.apple.loginwindow".to_string(),
            "RetriesUntilHint".to_string(),
            "-int".to_string(),
            "0".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c68_disable_guest_account() -> BashCommand {
    BashCommand {
        name: "c68_disable_guest_account",
        description: "Ensure Guest Account is Disabled.",
        executable: "sudo",
        args: vec![
            "/usr/bin/defaults".to_string(),
            "write".to_string(),
            "/Library/Preferences/com.apple.loginwindow".to_string(),
            "GuestEnabled".to_string(),
            "-bool".to_string(),
            "false".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c69_disable_guest_shared_folders_access() -> BashCommand {
    BashCommand {
        name: "c69_disable_guest_shared_folders_access",
        description: "Ensure Guest Access to Shared Folders is Disabled.",
        executable: "sudo",
        args: vec![
            "/usr/sbin/sysadminctl".to_string(),
            "-smbGuestAccess".to_string(),
            "off".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c70_remove_guest_home_folder() -> BashCommand {
    BashCommand {
        name: "c70_remove_guest_home_folder",
        description: "6.1.5 Ensure the Guest home Folder Does Not Exist.",
        executable: "sudo",
        args: vec![
            "/bin/rm".to_string(),
            "-R".to_string(),
            "/Users/Guest".to_string(),
        ],
        dynamic_args: None,
    }
}

pub fn c71_enable_show_all_filename_extensions() -> Vec<BashCommand> {
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    let mut commands = vec![BashCommand {
        name: "c71_enable_show_all_extensions_root",
        description: "6.2 Ensure Show All Filename Extensions Setting is Enabled for root.",
        executable: "sudo",
        args: vec![
            "defaults".to_string(),
            "write".to_string(),
            "/var/root/Library/Preferences/.GlobalPreferences.plist".to_string(),
            "AppleShowAllExtensions".to_string(),
            "-bool".to_string(),
            "true".to_string(),
        ],
        dynamic_args: None,
    }];

    let user_commands = fetch_users()
        .iter()
        .map(|user| BashCommand {
            name: "c71_enable_show_all_extensions_for_everyuser",
            description: "6.2 Ensure Show All Filename Extensions Setting is Enabled.",
            executable: "sudo",
            args: vec![
                "-u".to_string(),
                user.to_string(),
                "/usr/bin/defaults".to_string(),
                "write".to_string(),
                format!("/Users/{}/Library/Preferences/.GlobalPreferences.plist", user),
                "AppleShowAllExtensions".to_string(),
                "-bool".to_string(),
                "true".to_string(),
            ],
            dynamic_args: None,
        })
        .collect::<Vec<BashCommand>>();

    commands.extend(user_commands);
    commands.push(BashCommand {
        name: "c71_kill_finder",
        description: "Restart Finder to apply Show All Extensions.",
        executable: "sudo",
        args: vec!["killall".to_string(), "Finder".to_string()],
        dynamic_args: None,
    });

    commands
}

pub fn c72_disable_auto_open_safe_files() -> Vec<BashCommand> {
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    fetch_users()
        .iter()
        .map(|user| BashCommand {
            name: "c72_disable_auto_open_safe_files_for_everyuser",
            description: "7.2.1 Ensure Automatic Opening of Safe Files in Safari is Disabled.",
            executable: "sudo",
            args: vec![
                "-u".to_string(),
                user.to_string(),
                "/usr/bin/defaults".to_string(),
                "read".to_string(),
                format!("/Users/{}/Library/Containers/com.apple.Safari/Data/Library/Preferences/com.apple.Safari", user),
                "AutoOpenSafeDownloads".to_string(),
            ],
            dynamic_args: None,
        })
        .collect()
}

pub fn c73_enable_warn_about_fraudulent_websites() -> Vec<BashCommand> {
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    fetch_users()
        .iter()
        .map(|user| BashCommand {
            name: "c73_warn_about_fraudulent_websites_for_everyuser",
            description: "7.2.4 Ensure Warn When Visiting A Fraudulent Website in Safari Is Enabled.",
            executable: "sudo",
            args: vec![
                "-u".to_string(),
                user.to_string(),
                "/usr/bin/defaults".to_string(),
                "read".to_string(),
                format!("/Users/{}/Library/Containers/com.apple.Safari/Data/Library/Preferences/com.apple.Safari", user),
                "WarnAboutFraudulentWebsites".to_string(),
            ],
            dynamic_args: None,
        })
        .collect()
}

pub fn c74_enable_cross_site_tracking_prevention() -> Vec<BashCommand> {
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    fetch_users()
        .iter()
        .flat_map(|user| {
            vec![
                BashCommand {
                    name: "c74_block_storage_policy_for_everyuser",
                    description: "7.2.5 Ensure Prevent Cross-site Tracking in Safari is Enabled.",
                    executable: "sudo",
                    args: vec![
                        "-u".to_string(),
                        user.to_string(),
                        "/usr/bin/defaults".to_string(),
                        "read".to_string(),
                        format!("/Users/{}/Library/Containers/com.apple.Safari/Data/Library/Preferences/com.apple.Safari", user),
                        "BlockStoragePolicy".to_string(),
                    ],
                    dynamic_args: None,
                },
                BashCommand {
                    name: "c74_storage_blocking_policy_for_everyuser",
                    description: "7.2.5 Ensure Prevent Cross-site Tracking in Safari is Enabled.",
                    executable: "sudo",
                    args: vec![
                        "-u".to_string(),
                        user.to_string(),
                        "/usr/bin/defaults".to_string(),
                        "read".to_string(),
                        format!("/Users/{}/Library/Containers/com.apple.Safari/Data/Library/Preferences/com.apple.Safari", user),
                        "WebKitPreferences.storageBlockingPolicy".to_string(),
                    ],
                    dynamic_args: None,
                },
                BashCommand {
                    name: "c74_webkit_storage_blocking_policy_for_everyuser",
                    description: "7.2.5 Ensure Prevent Cross-site Tracking in Safari is Enabled.",
                    executable: "sudo",
                    args: vec![
                        "-u".to_string(),
                        user.to_string(),
                        "/usr/bin/defaults".to_string(),
                        "read".to_string(),
                        format!("/Users/{}/Library/Containers/com.apple.Safari/Data/Library/Preferences/com.apple.Safari", user),
                        "WebKitStorageBlockingPolicy".to_string(),
                    ],
                    dynamic_args: None,
                },
            ]
        })
        .collect()
}

pub fn c75_disable_automatic_opening_of_safe_files() -> Vec<BashCommand> {
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    fetch_users()
        .iter()
        .map(|user| BashCommand {
            name: "c75_disable_auto_open_safe_files_for_everyuser",
            description: "7.2.6 Ensure Automatic Opening of Safe Files in Safari is Disabled.",
            executable: "sudo",
            args: vec![
                "-u".to_string(),
                user.to_string(),
                "/usr/bin/defaults".to_string(),
                "write".to_string(),
                format!("/Users/{}/Library/Containers/com.apple.Safari/Data/Library/Preferences/com.apple.Safari", user),
                "WBSPrivacyProxyAvailabilityTraffic".to_string(),
                "-int".to_string(),
                "3300".to_string(),
            ],
            dynamic_args: None,
        })
        .collect()
}

pub fn c76_disable_private_click_measurement() -> Vec<BashCommand> {
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    fetch_users()
        .iter()
        .map(|user| BashCommand {
            name: "c76_disable_private_click_measurement_for_everyuser",
            description: "7.2.7 Ensure Private Click Measurement in Safari is Disabled.",
            executable: "sudo",
            args: vec![
                "-u".to_string(),
                user.to_string(),
                "/usr/bin/defaults".to_string(),
                "write".to_string(),
                format!("/Users/{}/Library/Containers/com.apple.Safari/Data/Library/Preferences/com.apple.Safari", user),
                "WebKitPreferences.privateClickMeasurementEnabled".to_string(),
                "-bool".to_string(),
                "false".to_string(),
            ],
            dynamic_args: None,
        })
        .collect()
}

pub fn c77_enable_show_full_website_address() -> Vec<BashCommand> {
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    fetch_users()
        .iter()
        .map(|user| BashCommand {
            name: "c77_enable_full_website_address_for_everyuser",
            description: "7.2.8 Ensure Show Full Website Address in Safari is Enabled.",
            executable: "sudo",
            args: vec![
                "-u".to_string(),
                user.to_string(),
                "/usr/bin/defaults".to_string(),
                "read".to_string(),
                format!("/Users/{}/Library/Containers/com.apple.Safari/Data/Library/Preferences/com.apple.Safari", user),
                "ShowFullURLInSmartSearchField".to_string(),
            ],
            dynamic_args: None,
        })
        .collect()
}

pub fn c78_disable_bluetooth_sharing() -> Vec<BashCommand> {
    fn fetch_users() -> Vec<String> {
        let users_dir = "/Users";
        std::fs::read_dir(users_dir)
            .expect("Failed to read /Users directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if name != "Shared" && name != ".localized" {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    let mut commands = vec![
        // Comando para root
        BashCommand {
            name: "c78_disable_bluetooth_sharing_root",
            description: "2.4.7 Ensure Bluetooth Sharing Is Disabled for root.",
            executable: "sudo",
            args: vec![
                "defaults".to_string(),
                "write".to_string(),
                "/var/root/Library/Preferences/com.apple.Bluetooth".to_string(),
                "PrefKeyServicesEnabled".to_string(),
                "-bool".to_string(),
                "false".to_string(),
            ],
            dynamic_args: None,
        },
        BashCommand {
            name: "c78_disable_bluetooth_sharing_root_byhost",
            description: "2.4.7 Ensure Bluetooth Sharing Is Disabled for root (ByHost).",
            executable: "sudo",
            args: vec![
                "defaults".to_string(),
                "write".to_string(),
                "/var/root/Library/Preferences/ByHost/com.apple.Bluetooth".to_string(),
                "PrefKeyServicesEnabled".to_string(),
                "-bool".to_string(),
                "false".to_string(),
            ],
            dynamic_args: None,
        },
    ];

    // Comandos para cada usuário
    let user_commands = fetch_users()
        .iter()
        .map(|user| BashCommand {
            name: "c78_disable_bluetooth_sharing_for_everyuser",
            description: "2.4.7 Ensure Bluetooth Sharing Is Disabled.",
            executable: "sudo",
            args: vec![
                "-u".to_string(),
                user.to_string(),
                "/usr/bin/defaults".to_string(),
                "-currentHost".to_string(),
                "write".to_string(),
                "com.apple.Bluetooth".to_string(),
                "PrefKeyServicesEnabled".to_string(),
                "-bool".to_string(),
                "false".to_string(),
            ],
            dynamic_args: None,
        })
        .collect::<Vec<BashCommand>>();

    commands.extend(user_commands);
    commands
}

/// This function sets a login window banner for macOS systems.
/// The banner text is in English and references a generic organization name.
pub fn c79_set_login_window_banner() -> BashCommand {
    BashCommand {
        name: "c79_set_login_window_banner",
        description: "5.8 Ensure a Login Window Banner Exists.",
        executable: "sh",
        args: vec![
            "-c".to_string(),
            r#"sudo tee /Library/Security/PolicyBanner.txt > /dev/null <<EOL
=================================================================
                                                        LOGIN NOTICE
=================================================================

This system is the property of [ORGANIZATION].

Unauthorized use of this system is prohibited. By logging in, you
acknowledge that you have the proper authorization to access this
system and agree to comply with all applicable policies and regulations
of the [ORGANIZATION]. Any misuse or unauthorized use of this system is
strictly forbidden.

All system activities are monitored and recorded. Any violation of
the policies may result in disciplinary or legal actions, as appropriate.

                                                        [ORGANIZATION]
EOL"#
            .to_string(),
        ],
        dynamic_args: None,
    }
}


pub fn c80_restart_wazuh_agent() -> BashCommand {
    let file_path = "/Library/Ossec/bin/wazuh-control";

    if std::path::Path::new(file_path).exists() {
        // Reiniciar o agente Wazuh
        BashCommand {
            name: "c80_restart_wazuh_agent",
            description: "Restart Wazuh Agent if the control file is found.",
            executable: "sudo",
            args: vec![file_path.to_string(), "restart".to_string()],
            dynamic_args: None,
        }
    } else {
        // Comando alternativo se o arquivo não for encontrado
        BashCommand {
            name: "c80_wazuh_agent_not_found",
            description: "Notify that Wazuh Agent control file was not found.",
            executable: "echo",
            args: vec!["Wazuh Agent control file not found.".to_string()],
            dynamic_args: None,
        }
    }
}
