use crate::hardware::HardwareInfo;
use crate::logos;
use crate::system::SystemInfo;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const ACCENT: &str = "\x1b[1;36m";

struct LogoLine {
    display: String,
    width: usize,
}

pub fn render(sys: &SystemInfo, hw: &HardwareInfo, no_color: bool) -> String {
    render_with_distro(sys, hw, no_color, &sys.distro_id)
}

pub fn render_with_distro(sys: &SystemInfo, hw: &HardwareInfo, no_color: bool, distro_id: &str) -> String {
    let logo = logo_for(distro_id, no_color);
    let info_lines = build_info_lines(sys, hw, no_color);
    compose(&logo, &info_lines)
}

pub fn render_logo_only(distro_id: &str, no_color: bool) -> String {
    logo_for(distro_id, no_color)
        .into_iter()
        .map(|l| l.display)
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn logo_names() -> Vec<&'static str> {
    logos::list()
}

pub fn resolve_logo_name(name: &str) -> Option<String> {
    let normalized = normalize_name(name);
    if logos::get(&normalized).is_some() {
        return Some(normalized);
    }
    let alias = match normalized.as_str() {
        "archlinux" | "archcraft" | "garuda" | "cachyos" | "arco" | "arcolinux" => "arch",
        "endeavour" => "endeavouros",
        "mint" => "linuxmint",
        "popos" | "pop_os" => "pop",
        "opensuse-leap" | "opensuse-tumbleweed" | "opensuse_slowroll"
        | "opensuse-tumbleweed-slowroll" | "suse" | "sles" | "opensuse/suse" => "opensuse",
        "rhel" | "redhat" | "centos" | "rocky" | "almalinux" | "alma" | "oracle"
        | "rhel/centos/rocky/alma" => "rhel",
        "freebsd/bsd" => "freebsd",
        "bsd" => "bsd",
        "nix" => "nixos",
        "generic" => "unknown",
        "tux" => "linux",
        _ => return None,
    };
    if logos::get(alias).is_some() {
        Some(alias.to_string())
    } else {
        None
    }
}

fn normalize_name(name: &str) -> String {
    name.trim()
        .to_lowercase()
        .replace(' ', "_")
        .replace('/', "_")
}

fn logo_for(id: &str, no_color: bool) -> Vec<LogoLine> {
    let candidates = logo_candidates(id);
    for cand in candidates {
        if let Some(raw) = logos::get(&cand) {
            let palette = crate::logo_colors::colors(&cand);
            let has_placeholders = logo_has_placeholders(raw);
            return raw
                .lines()
                .enumerate()
                .map(|(i, l)| colorize_logo_line(l, no_color, palette, i, has_placeholders))
                .collect();
        }
    }
    if let Some(raw) = logos::get("linux") {
        return raw
            .lines()
            .enumerate()
            .map(|(i, l)| {
                colorize_logo_line(
                    l,
                    no_color,
                    crate::logo_colors::colors("linux"),
                    i,
                    logo_has_placeholders(raw),
                )
            })
            .collect();
    }
    if let Some(raw) = logos::get("unknown") {
        return raw
            .lines()
            .enumerate()
            .map(|(i, l)| {
                colorize_logo_line(
                    l,
                    no_color,
                    crate::logo_colors::colors("unknown"),
                    i,
                    logo_has_placeholders(raw),
                )
            })
            .collect();
    }
    vec![LogoLine {
        display: "(logo unavailable)".to_string(),
        width: "(logo unavailable)".chars().count(),
    }]
}

fn logo_has_placeholders(raw: &str) -> bool {
    let mut chars = raw.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '$' {
            if let Some(&next) = chars.peek() {
                if next.is_ascii_digit() {
                    return true;
                }
            }
        }
    }
    false
}

fn logo_candidates(id: &str) -> Vec<String> {
    let n = normalize_name(id);
    let mut c = Vec::new();
    if !n.is_empty() {
        c.push(n.clone());
    }
    let mapped = match n.as_str() {
        "archlinux" => Some("arch"),
        "endeavour" => Some("endeavouros"),
        "linuxmint" => Some("linuxmint"),
        "pop" => Some("pop"),
        "opensuse-tumbleweed" | "opensuse-leap" | "opensuse_slowroll" => Some("opensuse"),
        "raspbian" => Some("raspbian"),
        "kali" => Some("kali"),
        "manjaro" => Some("manjaro"),
        "ubuntu" => Some("ubuntu"),
        "debian" => Some("debian"),
        "fedora" => Some("fedora"),
        "alpine" => Some("alpine"),
        "nixos" => Some("nixos"),
        "gentoo" => Some("gentoo"),
        "void" => Some("void"),
        "centos" => Some("centos"),
        "rhel" | "redhat" => Some("rhel"),
        "rocky" => Some("rocky"),
        "almalinux" => Some("almalinux"),
        "freebsd" => Some("freebsd"),
        "android" => Some("android"),
        _ => None,
    };
    if let Some(m) = mapped {
        c.push(m.to_string());
    }
    if n.contains("arch") { c.push("arch".to_string()); }
    if n.contains("ubuntu") { c.push("ubuntu".to_string()); }
    if n.contains("debian") { c.push("debian".to_string()); }
    if n.contains("fedora") { c.push("fedora".to_string()); }
    if n.contains("suse") { c.push("opensuse".to_string()); }
    if n.contains("mint") { c.push("linuxmint".to_string()); }
    if n.contains("manjaro") { c.push("manjaro".to_string()); }
    if n.contains("nix") { c.push("nixos".to_string()); }
    if n.contains("gentoo") { c.push("gentoo".to_string()); }
    if n.contains("alpine") { c.push("alpine".to_string()); }
    if n.contains("void") { c.push("void".to_string()); }
    if n.contains("kali") { c.push("kali".to_string()); }
    c.push("linux".to_string());
    c.push("unknown".to_string());
    let mut out = Vec::new();
    for x in c {
        if !out.iter().any(|y: &String| y == &x) {
            out.push(x);
        }
    }
    out
}

fn colorize_logo_line(
    raw: &str,
    no_color: bool,
    palette: Option<&'static [&'static str]>,
    line_idx: usize,
    has_placeholders: bool,
) -> LogoLine {
    let mut display = String::with_capacity(raw.len() * 2 + 16);
    let mut width = 0usize;
    let mut chars = raw.chars().peekable();
    let mut active = false;
    let mut current: Option<&'static str> = None;

    if !no_color && has_placeholders {
        if let Some(p) = palette.and_then(|p| p.first().copied()) {
            display.push_str(p);
            current = Some(p);
            active = true;
        }
    }

    while let Some(ch) = chars.next() {
        if ch == '$' {
            if let Some(&next) = chars.peek() {
                if next.is_ascii_digit() {
                    chars.next();
                    if !no_color && has_placeholders {
                        if let Some(code) = placeholder_color(next, palette) {
                            if current != Some(code) {
                                display.push_str(code);
                                current = Some(code);
                            }
                            active = true;
                        } else if active {
                            display.push_str(RESET);
                            current = None;
                            active = false;
                        }
                    }
                    continue;
                }
            }
        }

        if !no_color && !has_placeholders {
            if let Some(p) = palette {
                if !p.is_empty() {
                    let code = p[(line_idx + width) % p.len()];
                    if current != Some(code) {
                        display.push_str(code);
                        current = Some(code);
                        active = true;
                    }
                }
            }
        }

        display.push(ch);
        width += 1;
    }

    if active {
        display.push_str(RESET);
    }

    LogoLine { display, width }
}

fn placeholder_color(d: char, palette: Option<&'static [&'static str]>) -> Option<&'static str> {
    if let Some(p) = palette {
        let idx = d.to_digit(10)? as usize;
        if idx == 0 {
            return p.first().copied();
        }
        if let Some(c) = p.get(idx - 1) {
            return Some(*c);
        }
        return p.first().copied();
    }
    palette_code(d)
}

fn palette_code(d: char) -> Option<&'static str> {
    match d {
        '1' => Some("\x1b[36m"),
        '2' => Some("\x1b[34m"),
        '3' => Some("\x1b[33m"),
        '4' => Some("\x1b[32m"),
        '5' => Some("\x1b[35m"),
        '6' => Some("\x1b[31m"),
        '7' => Some("\x1b[37m"),
        '8' => Some("\x1b[90m"),
        '9' => Some("\x1b[97m"),
        _ => None,
    }
}

fn build_info_lines(sys: &SystemInfo, hw: &HardwareInfo, no_color: bool) -> Vec<String> {
    let mut lines = Vec::new();
    let title = format!("{}@{}", sys.username, sys.hostname);
    let separator = "-".repeat(title.chars().count().max(1));

    lines.push(style(&title, ACCENT, no_color));
    lines.push(style(&separator, BOLD, no_color));
    lines.push(kv("OS", &sys.os, no_color));
    lines.push(kv("Kernel", &sys.kernel, no_color));
    lines.push(kv("Uptime", &sys.uptime, no_color));
    lines.push(kv("Shell", &sys.shell, no_color));
    lines.push(kv("DE", &sys.de, no_color));
    lines.push(kv("WM", &sys.wm, no_color));
    lines.push(kv("Terminal", &sys.terminal, no_color));
    lines.push(kv("CPU", &hw.cpu, no_color));
    if let Some(gpu) = &hw.gpu {
        if !gpu.trim().is_empty() && gpu != "Unknown" {
            lines.push(kv("GPU", gpu, no_color));
        }
    }
    lines.push(kv("Memory", &hw.memory, no_color));
    lines.push(kv("Disk", &hw.disk, no_color));
    lines.push(kv("Arch", &sys.architecture, no_color));

    lines
}

fn kv(key: &str, value: &str, no_color: bool) -> String {
    let label = format!("{key:<9}");
    format!("{} {}", style(&label, ACCENT, no_color), value)
}

fn style(text: &str, code: &str, no_color: bool) -> String {
    if no_color {
        text.to_string()
    } else {
        format!("{code}{text}{RESET}")
    }
}

fn compose(logo: &[LogoLine], info: &[String]) -> String {
    let logo_width = logo.iter().map(|l| l.width).max().unwrap_or(0);
    let gap = "  ";
    let mut out = String::new();
    let rows = logo.len().max(info.len());

    for i in 0..rows {
        let (logo_part, lwidth) = logo
            .get(i)
            .map(|l| (l.display.as_str(), l.width))
            .unwrap_or(("", 0));
        let info_part = info.get(i).map(|s| s.as_str()).unwrap_or("");
        let padding = logo_width.saturating_sub(lwidth);

        if !logo_part.is_empty() || !info_part.is_empty() {
            out.push_str(logo_part);
            if !info_part.is_empty() {
                out.push_str(&" ".repeat(padding));
                out.push_str(gap);
                out.push_str(info_part);
            }
        }
        if i + 1 < rows {
            out.push('\n');
        }
    }

    out
}
