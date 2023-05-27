
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct RootApp {
    txt: String,

    #[serde(skip)]
    pressed: bool,
}

impl Default for RootApp {
    fn default() -> Self {
        Self {txt:"<empty>".to_owned(), pressed:false}
    }
}

impl eframe::App for RootApp {
    fn update( &mut self, ctx: &egui::Context, _frame: &mut eframe::Frame ) {
        egui::CentralPanel::default().show( ctx, |ui| {
            ui.label("WWWapp Template v1.05");
            ui.horizontal( |ui| {
                let btn = ui.button( "uder pressure" );
                ui.label( format!("->{}", self.pressed) );
                if btn.clicked(){
                    println!("clicked with PRESSURE!!!");
                    self.txt = "button has been pressed".to_owned();
                    self.pressed = true;
            }
            });
            ui.text_edit_singleline(&mut self.txt);
            ui.label( format!("just edited: [{}]", self.txt) );
        });
    }
}
