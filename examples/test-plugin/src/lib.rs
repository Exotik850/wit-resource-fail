use exports::plugins::main::definitions::Guest;
use plugins::main::{
    imports::log,
    toml::{Toml, TomlValue},
};
use wit_resource_fail::*;

struct Plugin;

impl Guest for Plugin {
    fn run() -> Toml {
        // let val = TomlValue::String("THIS A TEST".to_string());

        let val = "THIS A TEST".to_string();
        log(&val);
        let val = TomlValue::String(val);
        
        let key = "Test".to_string();
        log(&key);

        let values: Vec<(String, Toml)> = vec![(
            key,
            Toml::new(val),
        )];

        let table = TomlValue::Table(values);

        let toml = Toml::new(table);
        let value = toml.get();
        log(&format!("{value:?}"));
        toml
    }
}

export_plugin!(Plugin);
