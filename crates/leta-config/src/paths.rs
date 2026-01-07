use std::path::PathBuf;

pub fn get_config_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config").join("leta")
}

pub fn get_config_path() -> PathBuf {
    get_config_dir().join("config.toml")
}

pub fn get_cache_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".cache").join("leta")
}

pub fn get_log_dir() -> PathBuf {
    get_cache_dir().join("log")
}

pub fn get_socket_path() -> PathBuf {
    get_cache_dir().join("daemon.sock")
}

pub fn get_pid_path() -> PathBuf {
    get_cache_dir().join("daemon.pid")
}

pub fn detect_workspace_root(path: &std::path::Path) -> Option<PathBuf> {
    let markers = [
        ".git",
        "pyproject.toml",
        "setup.py",
        "package.json",
        "Cargo.toml",
        "go.mod",
        "pom.xml",
        "build.gradle",
        "Gemfile",
        "composer.json",
        "mix.exs",
        "dune-project",
    ];

    let mut current = path.to_path_buf();
    loop {
        for marker in &markers {
            if current.join(marker).exists() {
                return Some(current);
            }
        }
        if !current.pop() {
            break;
        }
    }
    None
}
