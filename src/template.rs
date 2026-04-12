use crate::environment::Environment;
use regex::Regex;
use std::env;

pub fn interpolate(text: &str, environment: Option<&Environment>) -> Result<String, String> {
    let re = Regex::new(r"\{\{([\w\.]+)\}\}").unwrap();
    let mut result = text.to_string();
    let mut missing_vars = Vec::new();
    
    for cap in re.captures_iter(text) {
        let full_match = cap.get(0).unwrap().as_str();
        let var_name = cap.get(1).unwrap().as_str();
        
        let value = if var_name.starts_with("process.env.") {
            let env_var_name = &var_name["process.env.".len()..];
            env::var(env_var_name).ok()
        } else {
            environment.and_then(|e| e.vars.get(var_name).cloned())
        };
        
        if let Some(v) = value {
            result = result.replace(full_match, &v);
        } else {
            if !missing_vars.contains(&var_name.to_string()) {
                missing_vars.push(var_name.to_string());
            }
        }
    }
    
    if !missing_vars.is_empty() {
        return Err(format!("Missing variables: {}", missing_vars.join(", ")));
    }
    
    Ok(result)
}
