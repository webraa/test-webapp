
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct RootApp {
    txt: String,

    #[serde(skip)]
    is_wasm: bool,
    pressed: bool,
}

impl Default for RootApp {
    fn default() -> Self {
        Self {txt:"<empty>".to_owned(), is_wasm:is_wasm(), pressed:false}
    }
}


#[ cfg(not(target_arch = "wasm32")) ]
fn is_wasm() -> bool  {
    false
}

#[ cfg(target_arch = "wasm32") ]
fn is_wasm() -> bool  {
    true
}

impl RootApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage{
            println!("tryin to load..");
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}


impl eframe::App for RootApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        println!("saving..");
        eframe::set_value(storage, eframe::APP_KEY, self);
        println!("..saved");
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update( &mut self, ctx: &egui::Context, _frame: &mut eframe::Frame ) {
        egui::TopBottomPanel::bottom("bot_pan").show( ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("Powered by ");
                ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                ui.label(" and ");
                ui.hyperlink_to("eframe",
                    "https://github.com/emilk/egui/tree/master/crates/eframe",
                );
                ui.label(".");
            });
        });
        if self.is_wasm {
            egui::Window::new("mainWindow").show( ctx, |ui| {
                if self.is_wasm == true {
                    ui.label("WASM!!");
                }else{
                    ui.label("not wasm");
                }
                ui.label("egui test crossApp v0.3.0");
                ui.horizontal( |ui| {
                   let btn = ui.button( "try to save TEXT" );
                    ui.label( format!(" <{}>", self.pressed) );
                    if btn.clicked(){
                        println!("clicked with PRESSURE!!!");
                        self.pressed = true;
                    }
                });
                ui.text_edit_singleline(&mut self.txt);
                ui.label( format!("just edited: [{}]", self.txt) );
            });
        }else{
            egui::CentralPanel::default().show( ctx, |ui| {
                if self.is_wasm == true {
                    ui.label("WASM!!");
                }else{
                    ui.label("not wasm");
                }
                ui.label("egui test crossApp v0.2.0");
                ui.horizontal( |ui| {
                   let btn = ui.button( "try to save TEXT" );
                    ui.label( format!(" <{}>", self.pressed) );
                    if btn.clicked(){
                        println!("clicked with PRESSURE!!!");
                        self.pressed = true;
                    }
                });
                ui.text_edit_singleline(&mut self.txt);
                ui.label( format!("just edited: [{}]", self.txt) );
            });
        }
    }
}
