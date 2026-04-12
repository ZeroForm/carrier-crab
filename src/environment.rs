use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    #[serde(default)]
    pub vars: HashMap<String, String>,
}

impl Environment {
    pub fn load_from_file<P: AsRef<Path>>(target_yaml: P, env_name: &str) -> std::io::Result<Self> {
        let parent_dir = target_yaml.as_ref().parent().unwrap_or(Path::new(""));
        let env_file_path = parent_dir.join("environments").join(format!("{}.yml", env_name));
        
        if env_file_path.exists() {
            let content = fs::read_to_string(&env_file_path)?;
            let env: Environment = serde_yaml::from_str(&content)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
            Ok(env)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Environment file not found: {}", env_file_path.display())
            ))
        }
    }
}
