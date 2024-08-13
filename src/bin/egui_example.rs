use eframe::egui::{self, Color32};
use egui_extras::{Column, TableBuilder};

fn main() -> eframe::Result {
    let option = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Hello RustGui!",
        option,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<Calenders>::default())
        }),
    )
}

struct Time {
    day: u8,
    month: u8,
    year: u32,
    hour: u8,
    minute: u8,
    weekday: u8,
}

impl Time {
    pub fn new(weekday: u8, day: u8, month: u8, year: u32, hour: u8, minute: u8) -> Self {
        Self {
            day,
            month,
            year,
            hour,
            minute,
            weekday,
        }
    }
}

struct Event {
    start: Time,
    end: Time,
    name: String,
}

struct Calenders {
    name: String,
    events: Vec<Event>,
}

impl Default for Calenders {
    fn default() -> Self {
        Self {
            name: "Standard-Calender".to_owned(),
            events: vec![
                Event {
                    start: Time::new(6, 3, 8, 2024, 14, 0),
                    end: Time::new(6, 3, 8, 2024, 16, 0),
                    name: "Cool Zug-ride!".to_owned(),
                },
                Event {
                    start: Time::new(6, 3, 8, 2024, 17, 0),
                    end: Time::new(6, 3, 8, 2024, 19, 0),
                    name: "Work -.-".to_owned(),
                },
            ],
        }
    }
}

impl eframe::App for Calenders {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Calender", |ui| {
                    ui.label("Lol... ");
                    ui.label("... it works!");
                });
                ui.menu_button("Settings", |ui| {
                    ui.label("How nice is that ... ");
                    ui.label("... :3");
                });
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("THEE eGUI RUST APP CALENDERS woaaa!");
            TableBuilder::new(ui)
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::remainder())
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Monday").on_hover_ui(|ui| {
                            ui.style_mut().interaction.selectable_labels = true;
                            ui.label("Ufff, what a day... -.-");
                        });
                    });
                    header.col(|ui| {
                        ui.heading("Tuesday");
                    });
                    header.col(|ui| {
                        ui.heading("Wednesday");
                    });
                    header.col(|ui| {
                        ui.heading("Thursday");
                    });
                    header.col(|ui| {
                        ui.heading("Friday");
                    });
                    header.col(|ui| {
                        ui.heading("Saturday");
                    });
                    header.col(|ui| {
                        ui.heading("Sunday");
                    });
                })
                .body(|mut body| {
                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            ui.code_editor(&mut self.name);
                        });
                        row.col(|ui| {
                            ui.vertical(|ui| {
                                ui.button("Lol");
                                
                            });
                        })
                        .0
                        .set_height(1.0);
                        row.col(|ui| {
                            ui.code_editor(&mut self.name);
                        });
                        row.col(|ui| {
                            let f = egui::Frame::default().fill(Color32::from_rgb(255, 0, 0));
                            let item = ui.dnd_drop_zone::<i8,_>(f, |ui| {
                                egui::ScrollArea::vertical().show(ui, |ui| {})
                            });
                            match item.1 {
                                None => {},
                                Some(x) => println!("{x}")
                            }
                            f.paint(ui.min_rect());

                        });
                        row.col(|ui| {
                            ui.code_editor(&mut self.name);
                        });
                        row.col(|ui| {
                            egui::ScrollArea::vertical().show(ui, |ui| {});
                        });
                        row.col(|ui| {
                            ui.code_editor(&mut self.name);
                        });
                    });
                });
            //ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            //if ui.button("Increment").clicked() {
            //    //self.age += 1;
            //}
            //ui.label(format!("Hello '{}', age {}", self.name, self.age));
            //ui.image(egui::include_image!(
            //    "/usr/share/backgrounds/archlinux/simple.png"
            //))
        });
    }
}
