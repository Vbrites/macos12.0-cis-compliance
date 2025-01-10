# MacOS CIS Compliance CLI Tool

This project provides a CLI tool written in **Rust**, designed to configure macOS systems in compliance with the **CIS Apple macOS 12.0 Monterey Benchmark v3.1.0** security guidelines. The goal is to simplify the implementation of hardening policies, ensuring greater system security while maintaining flexibility to meet specific organizational needs.

---

## Features

- **Automated Configurations**: The program executes commands to automatically apply benchmark recommendations, such as firewall settings, password policies, software updates, and auditing adjustments.
- **Customizable Flexibility**: Certain settings, like **macOS automatic updates** and **login banner messages**, are left for manual configuration to avoid conflicts in production environments.
- **Compatibility with Newer Versions**: While the script was developed based on macOS Monterey (12.0), most commands are expected to work on newer versions. **I plan to review each command individually against the CIS benchmarks for newer versions**, ensuring continued compliance and avoiding bugs or unexpected results.

---

## Technologies and Dependencies

- **Language**: Rust  
- **Dependencies**: Only the **standard library (std)** is used, with no external dependencies, reinforcing security and simplicity.  

---

## Testing and Results

- Using **SIEM Wazuh**, the script raised compliance scores from **17% to 85%**.
- The remaining score is due to:
  - Consciously disabled settings, such as **automatic macOS updates**, to prevent disruptions in production.
  - CIS items requiring specific configuration profiles, which cannot be adjusted directly via terminal commands.

**I plan to provide these profiles soon to further enhance compliance.**

---

## Usage Instructions

1. **Clone this Repository**:
   ```bash
   git clone https://github.com/Vbrites/macos12.0-cis-compliance.git
   cd macos12.0-cis-compliance
   ```

2. **Precompiled Binary**:  
   The precompiled binary is available in the `compiled/` directory for immediate use. Run it with:
   ```bash
   sudo ./compiled/macos-cis-compliance
   ```

   **Note:** Ensure the binary has executable permissions. If not, run:
   ```bash
   chmod +x ./compiled/macos-cis-compliance
   ```

3. **Build from Source** (optional):  
   If you want to build the binary yourself, from the "macos12.0-cis-compliance" folder, run:
   ```bash
   cargo build --release
   ```

4. **Test Before Production**:  
   - Implement this script in a **test environment** before applying it to production systems.
   - Review the source code to adjust configurations that may cause disruptions in critical environments.

---

## Directory Structure

The project is organized as follows:

```
macos12.0-cis-compliance/
├── src/               # Source code in Rust
│   ├── main.rs        # Main script logic
│   ├── commands.rs    # CIS compliance commands
├── docs/              # Documentation and reference files
│   ├── CIS_Apple_macOS_12.0_Monterey_Benchmark_v3.1.0.pdf
├── compiled/          # Precompiled binary for immediate use
├── README.md          # Project documentation
├── LICENSE            # License file
```

---

## Implemented Configurations

### Automatically Applied Configurations:
- **Disabling unnecessary services**: Remote Apple Events, Screen Sharing, Printer Sharing, and more.
- **Enabling Security Auditing**: Configuring auditing flags and log retention.
- **Adjusting Password Policies**: Minimum length, special character requirements, and failed login attempt limits.

### Manually Adjustable Configurations:
- **macOS Automatic Updates**: Left disabled to avoid unexpected impacts on production systems.
- **Login Banner Messages**: Customizable according to the organization's policies.

### Profile-Dependent Configurations:
Some CIS recommendations cannot be configured via terminal commands. I am working on developing profiles to cover these items and enhance compliance.

---

## Future Plans

In addition to reviewing the script for newer macOS versions, I plan to provide:

- Configuration profiles for macOS.
- Configuration profiles for browsers like **Google Chrome**, **Edge**, and **Firefox**.

My goal is to contribute to a safer digital world by facilitating the implementation of high-security standards for everyone.

---

## Reporting Issues

If you encounter bugs or errors during execution, please open an issue in this repository with a detailed description of the problem. Your contribution will help improve this tool and ensure compliance with CIS standards.
