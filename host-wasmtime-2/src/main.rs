use std::env;
use std::path::Path;

use clegane::mapgen::mapgen_api::{Host, TileId, TileValue};
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};
use wasmtime_wasi::preview2::{command, Table, WasiCtx, WasiCtxBuilder, WasiView};

wasmtime::component::bindgen!({
    path: "../wasm_component_c/wit",
    world: "mapgen",
    async: true
});

struct ServerWasiView {
    table: Table,
    ctx: WasiCtx,
    pub tiles: Vec<f32>,
}

impl ServerWasiView {
    fn new() -> Self {
        let table = Table::new();
        let ctx = WasiCtxBuilder::new().inherit_stdio().build();
        let tiles = vec![0.0f32; 16];

        Self { table, ctx, tiles }
    }

    fn print_tiles(&self) {
        for (i, t) in self.tiles.iter().enumerate() {
            println!("tile {}: {}", i, t);
        }
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

//struct CleganeApi {
//    pub tiles: Vec<f32>,
//}
//
//impl CleganeApi {
//    fn new(size: u64) -> CleganeApi {
//        CleganeApi { tiles: vec![0.0f32; size as usize], }
//    }
//}

impl Host for ServerWasiView {
    // this should just be sync
    // TODO: bindgen supports except_imports and only_imports
    fn update_tile<'life0, 'async_trait>(
        &'life0 mut self,
        tile: TileId,
        value: TileValue,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = wasmtime::Result<()>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.tiles[tile as usize] = value;
        println!("set tile {} = {}", tile, value);
        Box::pin(async move { Ok(()) })
    }
}

#[tokio::main]
async fn main() {
    let mut config = Config::default();
    config.wasm_component_model(true);
    config.async_support(true);

    //let num_tiles = 16;
    //let api = CleganeApi::new(num_tiles);

    let engine = Engine::new(&config).unwrap();
    let wasi_view = ServerWasiView::new();
    let mut linker: Linker<ServerWasiView> = Linker::<ServerWasiView>::new(&engine);

    command::add_to_linker(&mut linker).unwrap();
    Mapgen::add_to_linker(&mut linker, |state: &mut ServerWasiView| state).unwrap();

    let mut store = Store::new(&engine, wasi_view);
    // skip arg 0
    let mut args = env::args();
    _ = args.next().unwrap();
    let path = args.next().unwrap();

    println!("load module {}", &path);
    let component = Component::from_file(&engine, Path::new(&path)).unwrap();
    let (instance, _) = Mapgen::instantiate_async(&mut store, &component, &linker)
        .await
        .unwrap();


    instance.interface0.call_run(&mut store, 16).await.unwrap();

    //for (i, v) in wasi_view.tiles.iter().enumerate() {
    //    println!("{} = {}", i, v);
    //}
}
