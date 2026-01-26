use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub include_private: bool,
    pub create_stubs: bool,
    pub verbose: bool,
    pub home_override: Option<String>,
    pub title_override: Option<String>,
    pub favorites_override: Option<Vec<String>>,
    pub site_name_override: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_dir: PathBuf::from("."),
            output_dir: PathBuf::from("quartz-content"),
            include_private: false,
            create_stubs: true,
            verbose: false,
            home_override: None,
            title_override: None,
            favorites_override: None,
            site_name_override: None,
        }
    }
}
