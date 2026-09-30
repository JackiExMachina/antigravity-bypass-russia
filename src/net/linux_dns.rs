use std::fs;
use std::path::{Path, PathBuf};

const MARKER: &str = "# ANTIGRAVITY-BYPASS-RUSSIA";
const RESOLVED_DIR: &str = "/etc/systemd/resolved.conf.d";
const CONF_FILE: &str = "antigravity-bypass.conf";

pub fn conf_path() -> PathBuf {
    Path::new(RESOLVED_DIR).join(CONF_FILE)
}

pub fn preflight(_domains: &[&str]) -> Result<(), String> {
    let path = conf_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if !content.contains(MARKER) {
                return Err(format!(
                    "Файл {} уже существует и не принадлежит Antigravity Bypass. Сохраните копию и удалите его перед продолжением.",
                    path.display()
                ));
            }
        }
    }
    Ok(())
}

pub fn apply(rules: &[(String, String)]) -> Result<(), String> {
    let dir = Path::new(RESOLVED_DIR);
    if let Err(e) = fs::create_dir_all(dir) {
        return Err(format!("Не удалось создать {}: {e}", dir.display()));
    }

    let mut domains = Vec::new();
    for (d, _) in rules {
        let trimmed = d.trim_start_matches('.');
        if !trimmed.is_empty() && !domains.contains(&trimmed) {
            domains.push(trimmed);
        }
    }

    let formatted_domains = domains
        .iter()
        .map(|d| format!("~{d}"))
        .collect::<Vec<_>>()
        .join(" ");

    let conf_content = format!(
        "{MARKER}\n# Автоматически сгенерировано Antigravity Bypass Linux\n[Resolve]\nDNS={}\nDomains={}\n",
        crate::net::relay::LISTEN_IP,
        formatted_domains
    );

    let path = conf_path();
    crate::system::fs_utils::robust_write_file(&path, conf_content.as_bytes())
        .map_err(|e| format!("Не удалось записать {}: {e}", path.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&path, fs::Permissions::from_mode(0o644));
    }

    // Reload systemd-resolved
    let _ = crate::system::command::output("systemctl", ["reload-or-restart", "systemd-resolved"]);

    Ok(())
}

pub fn remove() -> Result<(), String> {
    let path = conf_path();
    if path.exists() {
        fs::remove_file(&path).map_err(|e| format!("Не удалось удалить {}: {e}", path.display()))?;
        let _ = crate::system::command::output("systemctl", ["reload-or-restart", "systemd-resolved"]);
    }
    Ok(())
}

pub fn status_info() -> (usize, Option<String>, bool) {
    let path = conf_path();
    if !path.exists() {
        return (0, None, false);
    }
    let Ok(content) = fs::read_to_string(&path) else {
        return (0, None, false);
    };
    if !content.contains(MARKER) {
        return (0, None, false);
    }

    let mut count = 0;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("Domains=") {
            let domains_part = trimmed.trim_start_matches("Domains=").trim();
            count = domains_part.split_whitespace().count();
            break;
        }
    }

    let is_relay = content.contains(&format!("DNS={}", crate::net::relay::LISTEN_IP));
    (count, Some(crate::net::relay::LISTEN_IP.to_string()), is_relay)
}
