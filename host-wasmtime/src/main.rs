use std::path::Path;
use std::env;

use wasmtime::{Config, Engine, ExternRef, component::{Linker, Component, List}, Store, TableType, Val};
use wasmtime_wasi::preview2::{command, Table, WasiCtx, WasiCtxBuilder, WasiView};

wasmtime::component::bindgen!({
    path: "../wasm_component_b/wit/mapgen.wit",
    world: "mapgen",
    async: true
});

struct ServerWasiView {
    table: Table,
    ctx: WasiCtx,
}

impl ServerWasiView {
    fn new() -> Self {
        let table = Table::new();
        let ctx = WasiCtxBuilder::new().inherit_stdio().build();

        Self { table, ctx }
    }
}

impl WasiView for ServerWasiView {
    fn table(&self) -> &Table {
        &self.table
    }

    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.ctx
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

#[tokio::main]
async fn main() {
    let mut config = Config::default();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = Engine::new(&config).unwrap();
    let wasi_view = ServerWasiView::new();
    let mut linker: Linker<ServerWasiView> = Linker::<ServerWasiView>::new(&engine);

    command::add_to_linker(&mut linker).unwrap();

    let mut store = Store::new(&engine, wasi_view);
    // skip arg 0
    let mut args = env::args();
    _ = args.next().unwrap();
    let path = args.next().unwrap();

    println!("load module {}", &path);
    let component = Component::from_file(&engine, Path::new(&path)).unwrap();

    let (instance, _) = Mapgen::instantiate_async(&mut store, &component, &linker).await.unwrap();

    let mut floats: Vec<f32> = vec![0.0, 1.0, 2.0];
    //let array = Box::new(vec![Val::Float32(0.0), Val::Float32(1.0), Val::Float32(2.0)]);
    //let array: Box<[Val; 3]> = Box::new([Val::F32(0), Val::F32(1), Val::F32(2)]);
    //let ty = TableType::new(wasmtime::ValType::ExternRef, 1, None);
    //let ty_list =
    //let list = List::new(&ty, array);
    //let table = Table::new(&mut store, ty, Val::List(array));
    //let table = wasmtime::Table::new(&mut store, ty, Val::F32(23));

    //let extern_ref = ExternRef::new(&floats);
    instance.interface0.call_run(&mut store, &mut floats, 1.0).await.unwrap();

    for val in floats {
        println!("{}", val);
    }
}
