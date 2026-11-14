use std::fs;
use std::process::Command;

pub struct HardwareInfo {
    pub cpu: String,
    pub gpu: Option<String>,
    pub memory: String,
    pub disk: String,
}

pub fn collect() -> HardwareInfo {
    HardwareInfo {
        cpu: cpu(),
        gpu: gpu(),
        memory: memory(),
        disk: disk(),
    }
}

pub fn cpu() -> String {
    let Some(content) = read_to_string("/proc/cpuinfo") else {
        return "Unknown".to_string();
    };

    let mut model_name = None;
    let mut hardware = None;
    let mut processor = None;

    for line in content.lines() {
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();
            if value.is_empty() {
                continue;
            }
            match key {
                "model name" if model_name.is_none() => model_name = Some(value.to_string()),
                "Hardware" if hardware.is_none() => hardware = Some(value.to_string()),
                "Processor" if processor.is_none() => processor = Some(value.to_string()),
                _ => {}
            }
        }
    }

    model_name
        .or(hardware)
        .or(processor)
        .unwrap_or_else(|| "Unknown".to_string())
}

pub fn memory() -> String {
    let Some(content) = read_to_string("/proc/meminfo") else {
        return "Unknown".to_string();
    };

    let mut mem_total_kb: Option<u64> = None;
    let mut mem_available_kb: Option<u64> = None;
    let mut mem_free_kb: Option<u64> = None;
    let mut buffers_kb: Option<u64> = None;
    let mut cached_kb: Option<u64> = None;
    let mut sreclaimable_kb: Option<u64> = None;
    let mut shmem_kb: Option<u64> = None;

    for line in content.lines() {
        if let Some((key, value)) = line.split_once(':') {
            let num = parse_kb_value(value);
            match key.trim() {
                "MemTotal" => mem_total_kb = num,
                "MemAvailable" => mem_available_kb = num,
                "MemFree" => mem_free_kb = num,
                "Buffers" => buffers_kb = num,
                "Cached" => cached_kb = num,
                "SReclaimable" => sreclaimable_kb = num,
                "Shmem" => shmem_kb = num,
                _ => {}
            }
        }
    }

    let Some(total) = mem_total_kb else {
        return "Unknown".to_string();
    };

    let available = mem_available_kb.or_else(|| {
        let free = mem_free_kb.unwrap_or(0);
        let buffers = buffers_kb.unwrap_or(0);
        let cached = cached_kb.unwrap_or(0);
        let sreclaim = sreclaimable_kb.unwrap_or(0);
        let shmem = shmem_kb.unwrap_or(0);
        let approx = free + buffers + cached + sreclaim;
        Some(approx.saturating_sub(shmem))
    });

    let used = match available {
        Some(avail) if avail <= total => total - avail,
        _ => 0,
    };

    format!("{} / {}", format_gib(used), format_gib(total))
}

pub fn gpu() -> Option<String> {
    let entries = fs::read_dir("/sys/class/drm").ok()?;
    let mut gpus: Vec<String> = Vec::new();

    let mut cards: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    cards.sort_by_key(|e| e.file_name());

    for entry in cards {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !is_card_device(&name) {
            continue;
        }
        let device_path = entry.path().join("device");
        if !device_path.exists() {
            continue;
        }
        if let Some(desc) = describe_gpu(&device_path) {
            if !gpus.iter().any(|g| g == &desc) {
                gpus.push(desc);
            }
        }
    }

    if gpus.is_empty() {
        None
    } else {
        Some(gpus.join(", "))
    }
}

pub fn disk() -> String {
    let output = Command::new("df")
        .args(["-B1", "-P", "/"])
        .output();
    let Ok(output) = output else {
        return "Unknown".to_string();
    };
    if !output.status.success() {
        return "Unknown".to_string();
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let mut lines = text.lines();
    let _header = lines.next();
    let Some(line) = lines.next() else {
        return "Unknown".to_string();
    };
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 5 {
        return "Unknown".to_string();
    }
    let total = parts[1].parse::<u64>().ok();
    let used = parts[2].parse::<u64>().ok();
    let percent = parts[4].trim_end_matches('%');
    match (used, total) {
        (Some(u), Some(t)) if t > 0 => {
            format!("{} / {} ({}%)", format_bytes(u), format_bytes(t), percent)
        }
        _ => "Unknown".to_string(),
    }
}

fn read_to_string(path: &str) -> Option<String> {
    fs::read_to_string(path).ok()
}

fn parse_kb_value(value: &str) -> Option<u64> {
    value
        .split_whitespace()
        .next()
        .and_then(|n| n.parse::<u64>().ok())
}

fn format_gib(kib: u64) -> String {
    let gib = kib as f64 / 1024.0 / 1024.0;
    format!("{gib:.1} GiB")
}

fn format_bytes(bytes: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = 1024.0 * 1024.0;
    const GIB: f64 = 1024.0 * 1024.0 * 1024.0;
    const TIB: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0;
    let b = bytes as f64;
    if b >= TIB {
        format!("{:.1} TiB", b / TIB)
    } else if b >= GIB {
        format!("{:.1} GiB", b / GIB)
    } else if b >= MIB {
        format!("{:.1} MiB", b / MIB)
    } else if b >= KIB {
        format!("{:.1} KiB", b / KIB)
    } else {
        format!("{bytes} B")
    }
}

fn is_card_device(name: &str) -> bool {
    if !name.starts_with("card") {
        return false;
    }
    let suffix = &name[4..];
    !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit())
}

fn describe_gpu(device_path: &std::path::Path) -> Option<String> {
    if let Some(label) = read_trimmed(&device_path.join("label")) {
        if !label.is_empty() {
            return Some(label);
        }
    }
    if let Some(product) = read_trimmed(&device_path.join("product_name")) {
        if !product.is_empty() {
            return Some(product);
        }
    }

    let vendor = read_trimmed(&device_path.join("vendor")).unwrap_or_default();
    let device = read_trimmed(&device_path.join("device")).unwrap_or_default();
    let vendor_name = pci_vendor_name(&vendor);
    let device_id = normalize_pci_id(&device);

    match (vendor_name, device_id) {
        (Some(v), Some(d)) => Some(format!("{v} {d}")),
        (None, Some(d)) if !vendor.is_empty() => {
            Some(format!("{} {d}", normalize_pci_id(&vendor).unwrap_or(vendor)))
        }
        _ => {
            let driver = read_trimmed(&device_path.join("uevent")).and_then(|u| {
                u.lines().find_map(|line| {
                    line.strip_prefix("DRIVER=")
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                })
            });
            driver
        }
    }
}

fn read_trimmed(path: &std::path::Path) -> Option<String> {
    fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn normalize_pci_id(id: &str) -> Option<String> {
    if id.is_empty() {
        return None;
    }
    let trimmed = id.trim().to_lowercase();
    let hex = trimmed.strip_prefix("0x").unwrap_or(&trimmed);
    if hex.is_empty() {
        None
    } else {
        Some(format!("0x{hex}"))
    }
}

fn pci_vendor_name(vendor: &str) -> Option<&'static str> {
    match normalize_pci_id(vendor).as_deref() {
        Some("0x8086") => Some("Intel"),
        Some("0x10de") => Some("NVIDIA"),
        Some("0x1002") | Some("0x1022") => Some("AMD"),
        Some("0x13b5") => Some("ARM"),
        Some("0x5143") => Some("Qualcomm"),
        _ => None,
    }
}

