use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[derive(Default)]
struct ConfigReloader;

impl ZellijPlugin for ConfigReloader {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::Reconfigure,
            PermissionType::OpenTerminalsOrPlugins,
        ]);
    }

    fn pipe(&mut self, _pipe_message: PipeMessage) -> bool {
        reload_config_file();

        for plugin in read_config().blocks.load_plugins {
            if plugin.cwd.is_some() || !plugin.configuration.is_empty() {
                eprintln!(
                    "Skipping plugin {}: start_or_reload_plugin cannot preserve its cwd or configuration",
                    plugin.location
                );
                continue;
            }

            eprintln!("Reloading configured plugin: {}", plugin.location);
            start_or_reload_plugin(&plugin.location);
        }

        false
    }
}

register_plugin!(ConfigReloader);
