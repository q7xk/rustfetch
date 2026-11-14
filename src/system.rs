use std::env;
use std::fs;
use std::path::Path;

pub struct SystemInfo {
    pub os: String,
    pub distro_id: String,
    pub kernel: String,
    pub hostname: String,
    pub username: String,
    pub uptime: String,
    pub shell: String,
    pub de: String,
    pub wm: String,
    pub terminal: String,
    pub architecture: String,
}

pub fn collect() -> SystemInfo {
    let (os_name, distro_id) = os_and_id();
    SystemInfo {
        os: os_name,
        distro_id,
        kernel: kernel(),
        hostname: hostname(),
        username: username(),
        uptime: uptime(),
        shell: shell(),
        de: desktop_environment(),
        wm: window_manager(),
        terminal: terminal_name(),
        architecture: architecture(),
    }
}

fn read_to_string(path: &str) -> Option<String> {
    fs::read_to_string(path).ok()
}

fn os_and_id() -> (String, String) {
    let arch = architecture();
    let (name, id) = read_os_release().unwrap_or_else(|| ("Linux".to_string(), "linux".to_string()));
    (format!("{name} {arch}"), id)
}

pub fn os_name() -> String {
    os_and_id().0
}

pub fn distro_id() -> String {
    os_and_id().1
}

fn read_os_release() -> Option<(String, String)> {
    for path in ["/etc/os-release", "/usr/lib/os-release"] {
        if let Some(content) = read_to_string(path) {
            if let Some(parsed) = parse_os_release(&content) {
                return Some(parsed);
            }
        }
    }
    None
}

fn parse_os_release(content: &str) -> Option<(String, String)> {
    let mut name = None;
    let mut version = None;
    let mut pretty = None;
    let mut id = None;
    let mut id_like = None;

    for line in content.lines() {
        if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
            pretty = Some(unquote(value));
        } else if let Some(value) = line.strip_prefix("NAME=") {
            name = Some(unquote(value));
        } else if let Some(value) = line.strip_prefix("VERSION_ID=") {
            version = Some(unquote(value));
        } else if let Some(value) = line.strip_prefix("ID=") {
            id = Some(unquote(value).to_lowercase());
        } else if let Some(value) = line.strip_prefix("ID_LIKE=") {
            id_like = Some(unquote(value).to_lowercase());
        }
    }

    let display = if let Some(p) = pretty.filter(|p| !p.is_empty()) {
        p
    } else {
        match (name, version) {
            (Some(n), Some(v)) if !n.is_empty() && !v.is_empty() => format!("{n} {v}"),
            (Some(n), _) if !n.is_empty() => n,
            _ => return None,
        }
    };

    let distro_id = id
        .filter(|s| !s.is_empty())
        .or_else(|| {
            id_like
                .as_ref()
                .and_then(|s| s.split_whitespace().next().map(|x| x.to_string()))
                .filter(|s| !s.is_empty())
        })
        .unwrap_or_else(|| "linux".to_string());

    Some((display, distro_id))
}

fn unquote(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.len() >= 2 {
        let bytes = trimmed.as_bytes();
        let first = bytes[0] as char;
        let last = bytes[trimmed.len() - 1] as char;
        if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
            return trimmed[1..trimmed.len() - 1].to_string();
        }
    }
    trimmed.to_string()
}

pub fn kernel() -> String {
    read_to_string("/proc/sys/kernel/osrelease")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "Unknown".to_string())
}

pub fn hostname() -> String {
    read_to_string("/proc/sys/kernel/hostname")
        .or_else(|| read_to_string("/etc/hostname"))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .or_else(|| env::var("HOSTNAME").ok().filter(|s| !s.is_empty()))
        .unwrap_or_else(|| "Unknown".to_string())
}

pub fn username() -> String {
    env::var("USER")
        .or_else(|_| env::var("LOGNAME"))
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "Unknown".to_string())
}

pub fn uptime() -> String {
    let Some(content) = read_to_string("/proc/uptime") else {
        return "Unknown".to_string();
    };
    let Some(seconds_str) = content.split_whitespace().next() else {
        return "Unknown".to_string();
    };
    let Ok(total_seconds) = seconds_str.parse::<f64>() else {
        return "Unknown".to_string();
    };
    format_uptime(total_seconds as u64)
}

fn format_uptime(total_seconds: u64) -> String {
    let days = total_seconds / 86_400;
    let hours = (total_seconds % 86_400) / 3_600;
    let minutes = (total_seconds % 3_600) / 60;

    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{days} day{}", if days == 1 { "" } else { "s" }));
    }
    if hours > 0 || days > 0 {
        parts.push(format!("{hours} hour{}", if hours == 1 { "" } else { "s" }));
    }
    if minutes > 0 || parts.is_empty() {
        parts.push(format!(
            "{minutes} min{}",
            if minutes == 1 { "" } else { "s" }
        ));
    }

    parts.join(", ")
}

pub fn shell() -> String {
    if let Some(name) = parent_process_name() {
        if let Some(pretty) = normalize_shell_name(&name) {
            return pretty;
        }
    }
    env::var("SHELL")
        .ok()
        .filter(|s| !s.is_empty())
        .and_then(|s| {
            Path::new(&s)
                .file_name()
                .and_then(|n| n.to_str())
                .map(|n| normalize_shell_name(n).unwrap_or_else(|| n.to_string()))
        })
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "Unknown".to_string())
}

fn parent_process_name() -> Option<String> {
    let stat = fs::read_to_string("/proc/self/stat").ok()?;
    let end = stat.rfind(')')?;
    let rest = stat.get(end + 2..)?;
    let ppid = rest.split_whitespace().nth(1)?;
    let comm = fs::read_to_string(format!("/proc/{ppid}/comm")).ok()?;
    let name = comm.trim().to_string();
    if name.is_empty() { None } else { Some(name) }
}

fn normalize_shell_name(name: &str) -> Option<String> {
    let base = Path::new(name)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(name)
        .trim()
        .trim_start_matches('-')
        .to_lowercase();
    if base.is_empty() {
        return None;
    }
    let pretty = if base == "bash" {
        "bash".to_string()
    } else if base == "zsh" {
        "zsh".to_string()
    } else if base == "pwsh" || base == "powershell" {
        "pwsh".to_string()
    } else if base == "fish" {
        "fish".to_string()
    } else if base == "sh" || base == "dash" || base == "ash" {
        base
    } else {
        return None;
    };
    Some(pretty)
}

pub fn desktop_environment() -> String {
    env::var("XDG_CURRENT_DESKTOP")
        .ok()
        .filter(|s| !s.is_empty())
        .or_else(|| env::var("DESKTOP_SESSION").ok().filter(|s| !s.is_empty()))
        .or_else(|| env::var("XDG_SESSION_DESKTOP").ok().filter(|s| !s.is_empty()))
        .unwrap_or_else(|| "Unknown".to_string())
}

pub fn window_manager() -> String {
    if let Ok(wm) = env::var("XDG_SESSION_TYPE") {
        if !wm.is_empty() {
            return match wm.to_lowercase().as_str() {
                "wayland" => "Wayland".to_string(),
                "x11" => "X11".to_string(),
                _ => wm,
            };
        }
    }
    if env::var("WAYLAND_DISPLAY").ok().filter(|s| !s.is_empty()).is_some() {
        return "Wayland".to_string();
    }
    if env::var("DISPLAY").ok().filter(|s| !s.is_empty()).is_some() {
        return "X11".to_string();
    }
    "Unknown".to_string()
}

pub fn terminal_name() -> String {
    env::var("TERM_PROGRAM")
        .ok()
        .filter(|s| !s.is_empty())
        .or_else(|| env::var("TERMINAL").ok().filter(|s| !s.is_empty()))
        .or_else(|| env::var("TERM").ok().filter(|s| !s.is_empty()))
        .unwrap_or_else(|| "Unknown".to_string())
}

pub fn architecture() -> String {
    env::consts::ARCH.to_string()
}
