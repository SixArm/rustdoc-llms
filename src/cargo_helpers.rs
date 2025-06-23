use std::fs;
use toml::Value;
use std::path::Path;

/// Get a crate's lib name from its Cargo.toml file.
///
/// Steps:
///
/// 1. Read the `Cargo.toml` file
/// 2. Parse it as TOML
/// 3. If there is a `[lib]` section with a `name` field, then return it.
/// 4. If there is a `package.name`, then return it.
/// 5. Otherwise return an error.
///
/// This llb name priority follows Cargo's explicit rules about library naming:
/// `[lib].name` takes precedence over `package.name`.
///
pub fn lib_name(path: impl AsRef<Path>) -> Result<String, Box<dyn std::error::Error>> {

    // Read the Cargo.toml file into TOML
    let content = fs::read_to_string(path.as_ref())?;
    let toml: Value = toml::from_str(&content)?;

    // Get lib name from [lib] section
    if let Some(lib) = toml.get("lib") {
        if let Some(name) = lib.get("name") {
            if let Some(name_str) = name.as_str() {
                return Ok(name_str.to_string());
            }
        }
    }

    // Get package name
    if let Some(package) = toml.get("package") {
        if let Some(name) = package.get("name") {
            if let Some(name_str) = name.as_str() {
                return Ok(name_str.to_string());
            }
        }
    }

    Err("No library name found in Cargo.toml".into())
}
