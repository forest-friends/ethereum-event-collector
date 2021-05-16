use config::{Config, ConfigError, File, FileFormat, Environment};

pub fn load<'a, T: serde::de::Deserialize<'a>>(file_path: &str) -> Result<T, ConfigError> {
    let config = {
        let mut config = Config::new();
        config.merge(File::from_str(file_path, FileFormat::Yaml))?;
        config.merge(Environment::new().separator("."))?;

        if let Ok(ref path) = std::env::var("CONFIG_PATH") {
            config.merge(File::new(path, FileFormat::Yaml).required(true))?;
        }

        config
    };

    config.try_into()
}