//! Runtime configuration for the iOS client.

use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub auth_host: String,
    pub auth_port: u16,
    pub username: String,
    pub password: String,
    pub auto_login: bool,
    pub realm_index: usize,
    pub character_index: usize,
    pub data_path: Option<PathBuf>,
    pub locale: String,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            auth_host: "127.0.0.1".into(),
            auth_port: 3724,
            username: String::new(),
            password: String::new(),
            auto_login: false,
            realm_index: 0,
            character_index: 0,
            data_path: None,
            locale: "enUS".into(),
        }
    }
}

impl ClientConfig {
    pub fn from_env() -> Self {
        let mut cfg = Self::default();
        if let Ok(h) = std::env::var("WOW_AUTH_HOST") {
            cfg.auth_host = h;
        }
        if let Ok(p) = std::env::var("WOW_AUTH_PORT") {
            if let Ok(v) = p.parse() {
                cfg.auth_port = v;
            }
        }
        if let Ok(u) = std::env::var("WOW_USERNAME") {
            cfg.username = u;
        }
        if let Ok(p) = std::env::var("WOW_PASSWORD") {
            cfg.password = p;
        }
        if std::env::var("WOW_AUTO_LOGIN").ok().as_deref() == Some("1") {
            cfg.auto_login = true;
        }
        if let Ok(p) = std::env::var("WOW_DATA_PATH") {
            cfg.data_path = Some(PathBuf::from(p));
        } else if let Some(p) = wow_platform::data_path::default_data_path() {
            cfg.data_path = Some(p);
        }
        if let Ok(l) = std::env::var("WOW_LOCALE") {
            cfg.locale = l;
        }
        cfg
    }

    pub fn set_data_path<P: AsRef<Path>>(&mut self, path: P) {
        self.data_path = Some(path.as_ref().to_path_buf());
    }
}
