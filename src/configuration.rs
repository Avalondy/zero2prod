#[derive(serde::Deserialize)]
pub struct Settings {
    database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
struct DatabaseSettings {
    username: String,
    password: String,
    port: u16,
    host: String,
    database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        // Add configuration values from a file named `configuration.yaml`.
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    // Try to convert the configuration values into Settings type
    settings.try_deserialize()
}
