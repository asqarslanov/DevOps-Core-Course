use std::env;
use std::net::Ipv4Addr;
use std::str::FromStr;

pub struct AppConfig {
    pub host: Ipv4Addr,
    pub port: u16,
    pub debug: bool,
}

impl AppConfig {
    pub fn load() -> eyre::Result<Self> {
        let host = load_env_var_or(Ipv4Addr::UNSPECIFIED, "HOST")?;
        let port = load_env_var_or(5000, "PORT")?;
        let debug = load_env_var_or(false, "DEBUG")?;

        Ok(Self { host, port, debug })
    }
}

fn load_env_var_or<T>(default: T, name: &'static str) -> eyre::Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    match env::var(name) {
        Ok(s) => Ok(s.parse()?),
        Err(err) => match err {
            env::VarError::NotPresent => Ok(default),
            env::VarError::NotUnicode(_) => Err(err.into()),
        },
    }
}
