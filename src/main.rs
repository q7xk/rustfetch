mod cli;
mod hardware;
mod logo_colors;
mod logos;
mod system;
mod terminal;

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = env::args().skip(1);
    match cli::parse_args(args) {
        cli::ParseResult::Help => {
            print!("{}", cli::help_text());
            ExitCode::SUCCESS
        }
        cli::ParseResult::Version => {
            println!("{}", cli::version_text());
            ExitCode::SUCCESS
        }
        cli::ParseResult::Config => {
            println!("{}", cli::config_path());
            ExitCode::SUCCESS
        }
        cli::ParseResult::Err(msg) => {
            eprintln!("{msg}");
            ExitCode::from(2)
        }
        cli::ParseResult::Ok(options) => run(options),
    }
}

fn run(options: cli::Options) -> ExitCode {
    match options.command {
        cli::Command::Info => {
            let sys = system::collect();
            let hw = hardware::collect();
            if options.json {
                println!("{}", render_json(&sys, &hw));
            } else {
                println!("{}", terminal::render(&sys, &hw, options.no_color));
            }
            ExitCode::SUCCESS
        }
        cli::Command::Logo => {
            let id = system::distro_id();
            println!("{}", terminal::render_logo_only(&id, options.no_color));
            ExitCode::SUCCESS
        }
        cli::Command::LogoNamed(name) => {
            let sys = system::collect();
            let hw = hardware::collect();
            let id = terminal::resolve_logo_name(&name)
                .unwrap_or_else(|| name.trim().to_lowercase().replace(' ', "_").replace('/', "_"));
            println!("{}", terminal::render_with_distro(&sys, &hw, options.no_color, &id));
            ExitCode::SUCCESS
        }
        cli::Command::LogoList => {
            for name in terminal::logo_names() {
                println!("{name}");
            }
            ExitCode::SUCCESS
        }
        cli::Command::Cpu => {
            println!("{}", hardware::cpu());
            ExitCode::SUCCESS
        }
        cli::Command::Gpu => {
            println!("{}", hardware::gpu().unwrap_or_else(|| "Unknown".to_string()));
            ExitCode::SUCCESS
        }
        cli::Command::Memory => {
            println!("{}", hardware::memory());
            ExitCode::SUCCESS
        }
        cli::Command::Disk => {
            println!("{}", hardware::disk());
            ExitCode::SUCCESS
        }
        cli::Command::Os => {
            println!("{}", system::os_name());
            ExitCode::SUCCESS
        }
        cli::Command::Uptime => {
            println!("{}", system::uptime());
            ExitCode::SUCCESS
        }
    }
}

fn render_json(sys: &system::SystemInfo, hw: &hardware::HardwareInfo) -> String {
    let gpu = hw
        .gpu
        .as_ref()
        .map(|s| format!("\"{}\"", json_escape(s)))
        .unwrap_or_else(|| "null".to_string());
    format!(
        "{{\n  \"os\": \"{}\",\n  \"kernel\": \"{}\",\n  \"hostname\": \"{}\",\n  \"username\": \"{}\",\n  \"uptime\": \"{}\",\n  \"shell\": \"{}\",\n  \"de\": \"{}\",\n  \"wm\": \"{}\",\n  \"terminal\": \"{}\",\n  \"cpu\": \"{}\",\n  \"gpu\": {},\n  \"memory\": \"{}\",\n  \"disk\": \"{}\",\n  \"architecture\": \"{}\"\n}}",
        json_escape(&sys.os),
        json_escape(&sys.kernel),
        json_escape(&sys.hostname),
        json_escape(&sys.username),
        json_escape(&sys.uptime),
        json_escape(&sys.shell),
        json_escape(&sys.de),
        json_escape(&sys.wm),
        json_escape(&sys.terminal),
        json_escape(&hw.cpu),
        gpu,
        json_escape(&hw.memory),
        json_escape(&hw.disk),
        json_escape(&sys.architecture),
    )
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}
