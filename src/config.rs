use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub template: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine current directory.");
    let configuration_file_path = base_path.join(".pyza").join("config.yaml");
    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_file_path).required(true))
        .build()
        .expect("Failed to load configuration file.");
    settings.try_deserialize()
}
