use exports::plugins::main::definitions::Guest;
use plugins::main::{
    imports::log,
    toml::{Toml, TomlValue},
};
use wit_resource_fail::*;

struct Plugin;

impl Guest for Plugin {
    fn run() -> Toml {
        let values = vec![(
            "Test".into(),
            Toml::new(TomlValue::String("THIS A TEST".into())),
        )];

        let table = TomlValue::Table(values); // The error occurs here

        let toml = Toml::new(table);
        let value = toml.get();
        log(&format!("{value:?}"));
        toml
    }
}

export_plugin!(Plugin);
