use std::{env, net::SocketAddr};

#[derive(Clone, Debug)]
pub struct Config {
    pub app_name: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        let app_name = env::var("APP_NAME").unwrap_or_else(|_| "cardio-sentinel".into());
        let port = env::var("HTTP_PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(8080);

        Self { app_name, port }
    }

    pub fn addr(&self) -> SocketAddr {
        ([0, 0, 0, 0], self.port).into()
    }
}