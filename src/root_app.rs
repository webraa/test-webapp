
pub struct RootApp {
    //
    txt: String,
}

impl Default for RootApp {
    fn default() -> Self {
        Self {txt:"<empty>".to_owned(),}
    }
}

impl eframe::App for RootApp {
    fn update( &mut self, ctx: &egui::Context, _frame: &mut eframe::Frame ) {
        egui::CentralPanel::default().show( ctx, |ui| {
            ui.label("WWWapp Template v0.2");
            ui.label("WWWapp Template v0.2");
            let btn = ui.button( "uder pressure" );
            if btn.clicked(){
                println!("clicked with PRESSURE!!!");
                self.txt = "button has been pressed".to_owned();
            }
            ui.text_edit_singleline(&mut self.txt);
            ui.label( format!("just edited: [{}]", self.txt) );
        });
    }
}
