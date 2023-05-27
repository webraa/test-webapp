#![allow(non_snake_case)]

use eframe::egui;

mod root_app;
use crate::root_app::*;


#[ cfg(not(target_arch = "wasm32")) ]
fn main() -> Result<(), eframe::Error> {
    println!("MAIN has beed entered..");

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(200., 300.)),
        ..Default::default()
    };

    eframe::run_native(
        "some text 1",
        options,
        Box::new( |_cc| Box::<RootApp>::default()),
   )
}


#[ cfg(target_arch = "wasm32") ]
fn main() {
    println!("WAAAAAAAASMMMMM..");

    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    let options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "raa_canvas_id",
            options,
            Box::new( |_cc| Box::<RootApp>::default()),
        )
        .await
        .expect("failure with starting EFRAME");
    });
}


