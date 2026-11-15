use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Info,
    Cpu,
    Gpu,
    Memory,
    Disk,
    Os,
    Uptime,
    Logo,
    LogoNamed(String),
    LogoList,
}

#[derive(Debug, Clone)]
pub struct Options {
    pub command: Command,
    pub no_color: bool,
    pub json: bool,
    pub distro: Option<String>,
}

pub enum ParseResult {
    Ok(Options),
    Help,
    Version,
    Config,
    Err(String),
}

pub fn parse_args<I>(args: I) -> ParseResult
where
    I: IntoIterator<Item = String>,
{
    let mut no_color = false;
    let mut json = false;
    let mut distro: Option<String> = None;
    let mut positionals: Vec<String> = Vec::new();
    let mut iter = args.into_iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--help" | "-h" => return ParseResult::Help,
            "--version" | "-V" => return ParseResult::Version,
            "--config" => return ParseResult::Config,
            "--no-color" => no_color = true,
            "--json" => json = true,
            "--logo" => positionals.push("logo".to_string()),
            "--distro" | "-d" => match iter.next() {
                Some(value) if !value.starts_with('-') => distro = Some(value),
                _ => return ParseResult::Err("option '--distro' requires a value".to_string()),
            },
            _ if arg.starts_with("--distro=") => {
                let value = arg.trim_start_matches("--distro=").to_string();
                if value.is_empty() {
                    return ParseResult::Err("option '--distro' requires a value".to_string());
                }
                distro = Some(value);
            }
            _ if arg.starts_with('-') => {
                return ParseResult::Err(format!(
                    "unknown option: '{arg}'. Try 'rustfetch --help'."
                ))
            }
            _ => positionals.push(arg),
        }
    }

    let command = match positionals.as_slice() {
        [] => Command::Info,
        [cmd] if cmd == "info" => Command::Info,
        [cmd] if cmd == "cpu" => Command::Cpu,
        [cmd] if cmd == "gpu" => Command::Gpu,
        [cmd] if cmd == "memory" => Command::Memory,
        [cmd] if cmd == "disk" => Command::Disk,
        [cmd] if cmd == "os" => Command::Os,
        [cmd] if cmd == "uptime" => Command::Uptime,
        [cmd] if cmd == "logo" => Command::Logo,
        [cmd, sub] if cmd == "logo" && sub == "list" => Command::LogoList,
        [cmd, sub] if cmd == "logo" => Command::LogoNamed(sub.clone()),
        _ => {
            return ParseResult::Err(format!(
                "unknown command: '{}'. Try 'rustfetch --help'.",
                positionals.join(" ")
            ))
        }
    };

    if json && command != Command::Info {
        return ParseResult::Err("`--json` is only supported with full info output".to_string());
    }
    if distro.is_some() && command != Command::Info {
        return ParseResult::Err("`--distro` is only supported with full info output".to_string());
    }

    ParseResult::Ok(Options {
        command,
        no_color,
        json,
        distro,
    })
}

pub fn help_text() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!(
        "rustfetch {version}\n\
A lightweight, fast, minimal system information tool for Linux.\n\n\
USAGE:\n\
    rustfetch [COMMAND] [OPTIONS]\n\n\
COMMANDS:\n\
    (default)       Display full system information\n\
    info            Display full system information\n\
    cpu             Show CPU model\n\
    gpu             Show GPU information\n\
    memory          Show memory usage\n\
    disk            Show disk usage\n\
    os              Show operating system\n\
    uptime          Show uptime\n\
    logo            Display only the current distro logo\n\
    logo <name>     Display system info with a specific distro logo\n\
    logo list       List all available logo names

OPTIONS:
    -h, --help           Show this help message
    -V, --version        Show version information
    -d, --distro <name>  Use a specific distro logo for full info
        --config         Show configuration location
        --no-color       Disable terminal colors
        --json           Output full info as JSON
        --logo           Display only the logo
"
    )
}

pub fn version_text() -> String {
    format!("rustfetch {}", env!("CARGO_PKG_VERSION"))
}

pub fn config_path() -> String {
    if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        if !xdg.is_empty() {
            return format!("{xdg}/rustfetch/config.toml");
        }
    }
    if let Ok(home) = env::var("HOME") {
        if !home.is_empty() {
            return format!("{home}/.config/rustfetch/config.toml");
        }
    }
    "rustfetch/config.toml".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_default_info() {
        let res = parse_args(Vec::<String>::new());
        match res {
            ParseResult::Ok(opts) => {
                assert!(matches!(opts.command, Command::Info));
                assert!(!opts.no_color);
                assert!(!opts.json);
                assert!(opts.distro.is_none());
            }
            _ => panic!("expected Ok"),
        }
    }

    #[test]
    fn parse_distro_and_json() {
        let res = parse_args(vec![
            "--distro".to_string(),
            "nixos".to_string(),
            "--json".to_string(),
        ]);
        match res {
            ParseResult::Ok(opts) => {
                assert!(matches!(opts.command, Command::Info));
                assert!(opts.json);
                assert_eq!(opts.distro.as_deref(), Some("nixos"));
            }
            _ => panic!("expected Ok"),
        }
    }

    #[test]
    fn parse_logo_named() {
        let res = parse_args(vec!["logo".to_string(), "arch".to_string()]);
        match res {
            ParseResult::Ok(opts) => {
                assert!(matches!(opts.command, Command::LogoNamed(ref n) if n == "arch"))
            }
            _ => panic!("expected Ok"),
        }
    }

    #[test]
    fn reject_unknown_option() {
        let res = parse_args(vec!["--wat".to_string()]);
        assert!(matches!(res, ParseResult::Err(_)));
    }

    #[test]
    fn reject_json_with_subcommand() {
        let res = parse_args(vec!["cpu".to_string(), "--json".to_string()]);
        assert!(matches!(res, ParseResult::Err(_)));
    }
}
