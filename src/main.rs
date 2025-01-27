use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hello, eGUI!",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

#[derive(Default)]
struct MyApp {
    counter: i32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to eGUI!");

            // Counter display and button to increment it
            ui.horizontal(|ui| {
                if ui.button("Increment").clicked() {
                    self.counter += 1;
                }
                if ui.button("Decrement").clicked() {

                    self.counter -= 1;
                }

                if self.counter <= 0 {
                    ui.label(format!("Can't go that low!!"));
                    self.counter = 0;
                }
                

            });


                ui.label(format!("Counter: {}", self.counter));

        });
    }
}

