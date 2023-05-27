
pub struct RootApp {
    //
}

impl Default for RootApp {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for RootApp {
    fn update( &mut self, ctx: &egui::Context, _frame: &mut eframe::Frame ) {
        egui::CentralPanel::default().show( ctx, |ui| {
            ui.label("the road to WASM testing");
            let btn = ui.button( "uder pressure" );
            if btn.clicked(){
                println!("clicked with PRESSURE!!!");
            }
        });
    }
}
