use slab::Slab;
use wasmtime::{component::*, Store};
use wasmtime::{Config, Engine, Result};
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiView, WasiCtxBuilder};

use plugins::main::imports::Host;
use plugins::main::toml::{Host as TomlHost, HostToml, Toml, TomlValue};

use async_trait::async_trait;

impl Clone for TomlValue {
    fn clone(&self) -> Self {
        match self {
            TomlValue::String(string) => TomlValue::String(string.clone()),
            TomlValue::Table(table) => TomlValue::Table(
                table
                    .iter()
                    .map(|(key, val)| (key.clone(), Resource::new_own(val.rep())))
                    .collect(),
            ),
        }
    }
}

struct PluginState {
    pub table: Table,
    pub ctx: WasiCtx,
    pub slab: slab::Slab<TomlValue>,
}

#[async_trait]
impl Host for PluginState {
    async fn log(&mut self, value: String) -> Result<()> {
        println!("{value}");
        Ok(())
    }
}

impl TomlHost for PluginState {}

#[async_trait]
impl HostToml for PluginState {
    async fn new(&mut self, value: TomlValue) -> Result<Resource<Toml>> {
        let key = self.slab.insert(value);
        Ok(Resource::new_own(key as u32))
    }

    async fn get(&mut self, value: Resource<Toml>) -> Result<TomlValue> {
        Ok(self.slab.get(value.rep() as usize).cloned().unwrap())
    }

    fn drop(&mut self, res_toml: Resource<Toml>) -> Result<()> {
        let key = res_toml.rep() as usize;
        if res_toml.owned() && self.slab.contains(key) {
            self.slab.remove(key);
        }
        Ok(())
    }
}

async fn run() {
    let engine = {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.async_support(true);
        Engine::new(&config).unwrap()
    };

    let path = "../../test.wasm";

    let component = Component::from_file(&engine, path).unwrap();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::preview2::command::add_to_linker(&mut linker).unwrap();
    PluginWorld::add_to_linker(&mut linker, |state: &mut PluginState| state).unwrap();

    let ctx = WasiCtxBuilder::new()
        .inherit_stderr()
        .inherit_stdin()
        .inherit_stdio()
        .inherit_stdout()
        .build();

    let table = Table::new();
    let mut store = Store::new(&engine, PluginState {
      table, ctx, slab: Slab::new()
    });

    let (bindings, instance) =
    PluginWorld::instantiate_async(&mut store, &component, &linker).await.unwrap();

    let resource = bindings.plugins_main_definitions().call_run(&mut store).await.unwrap();
    let value = store.data_mut().slab.get(resource.rep() as usize).unwrap();

    println!("{value:?}")

}

impl WasiView for PluginState {
    fn table(&self) -> &wasmtime_wasi::preview2::Table {
        &self.table
    }

    fn table_mut(&mut self) -> &mut wasmtime_wasi::preview2::Table {
        &mut self.table
    }

    fn ctx(&self) -> &wasmtime_wasi::preview2::WasiCtx {
        &self.ctx
    }

    fn ctx_mut(&mut self) -> &mut wasmtime_wasi::preview2::WasiCtx {
        &mut self.ctx
    }
}

bindgen!({
  path: "../../wit/plugin.wit",
  async: true,
});

fn main() {
  smol::block_on(run());
}