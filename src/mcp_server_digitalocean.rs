use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "@digitalocean/mcp";
const PACKAGE_VERSION: &str = "latest";
const SERVER_PATH: &str = "node_modules/@digitalocean/mcp/index.js";

#[derive(Debug, Deserialize, JsonSchema)]
struct DigitalOceanMcpSettings {
    #[serde(default)]
    digitalocean_api_token: Option<String>,
    #[serde(default)]
    services: Option<String>,
    #[serde(default)]
    digitalocean_api_endpoint: Option<String>,
}

struct DigitalOceanMcpExtension;

impl zed::Extension for DigitalOceanMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        let settings = ContextServerSettings::for_project("mcp-server-digitalocean", project)?;
        let settings_struct: DigitalOceanMcpSettings = match settings.settings {
            Some(v) => serde_json::from_value(v).map_err(|e| e.to_string())?,
            None => DigitalOceanMcpSettings {
                digitalocean_api_token: None,
                services: None,
                digitalocean_api_endpoint: None,
            },
        };

        let token = settings_struct
            .digitalocean_api_token
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| "digitalocean_api_token is required".to_string())?;

        let mut args = Vec::new();
        args.push(
            env::current_dir()
                .map_err(|e| e.to_string())?
                .join(SERVER_PATH)
                .to_string_lossy()
                .to_string(),
        );

        if let Some(services) = settings_struct.services {
            let trimmed = services.trim();
            if !trimmed.is_empty() {
                args.push("--services".to_string());
                args.push(trimmed.to_string());
            }
        }

        let mut env_vars = Vec::new();
        env_vars.push(("DIGITALOCEAN_API_TOKEN".to_string(), token));

        if let Some(endpoint) = settings_struct.digitalocean_api_endpoint {
            let trimmed = endpoint.trim();
            if !trimmed.is_empty() {
                env_vars.push((
                    "DIGITALOCEAN_API_ENDPOINT".to_string(),
                    trimmed.to_string(),
                ));
            }
        }

        Ok(Command {
            command: zed::node_binary_path()?,
            args,
            env: env_vars,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();

        let settings = ContextServerSettings::for_project("mcp-server-digitalocean", project);
        let mut default_settings =
            include_str!("../configuration/default_settings.jsonc").to_string();

        if let Ok(user_settings) = settings {
            if let Some(settings_value) = user_settings.settings {
                if let Ok(do_settings) =
                    serde_json::from_value::<DigitalOceanMcpSettings>(settings_value)
                {
                    match do_settings.digitalocean_api_token {
                        Some(token) if !token.trim().is_empty() => {
                            default_settings = default_settings.replace(
                                "\"YOUR_DIGITALOCEAN_API_TOKEN\"",
                                &format!("\"{}\"", token),
                            );
                        }
                        _ => {
                            default_settings = default_settings
                                .replace("\"YOUR_DIGITALOCEAN_API_TOKEN\"", "\"\"");
                        }
                    }
                }
            }
        }

        let settings_schema = serde_json::to_string(&schemars::schema_for!(
            DigitalOceanMcpSettings
        ))
        .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(DigitalOceanMcpExtension);
