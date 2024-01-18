#[macro_export]
macro_rules! export_plugin {
    ($name:ident) => {
        ::wit_bindgen::generate!({
            inline: "package plugins:main;

interface definitions {
  use toml.{toml};

  run: func() -> toml;
}

interface toml {
  /// The handle for a `TomlValue`
  resource toml {
    /// Creates a value in table and returns the handle
    constructor(value: toml-value);

    get: func() -> toml-value;
  }

  variant toml-value {
    %string(string),
    %table(table),
  }

  type table = list<tuple<string, toml>>;
}

interface imports {
  /// Sends the input string to whatever logger is currently being used
  log: func(info: string);

}

world plugin-world {
  import imports;
  import toml;
  export definitions;
}
",
            world: "plugin-world",
            exports: {
                world: $name,
                "plugins:main/definitions": $name
            },
        });
    };
}
