#![windows_subsystem = "windows"] // Questo nasconde la fastidiosa finestra nera del terminale su Windows!

use eframe::egui;
use chrono::{Local, Timelike, Utc};
use chrono_tz::Asia::Tokyo;
use std::f32::consts::PI;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0])
            .with_title("Orologi Globali"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Dual Clock App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    show_clocks: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { show_clocks: false }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Richiedi un aggiornamento continuo per animare la lancetta dei secondi
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            // Stile pulito e moderno (simile a web)
            ui.style_mut().spacing.item_spacing = egui::vec2(10.0, 20.0);

            if !self.show_clocks {
                ui.centered_and_justified(|ui| {
                    let btn_text = egui::RichText::new("Mostra Orologi")
                        .size(24.0)
                        .color(egui::Color32::WHITE);
                    let btn = egui::Button::new(btn_text)
                        .fill(egui::Color32::from_rgb(0, 110, 255))
                        .rounding(8.0); // Bordi arrotondati stile web
                        
                    if ui.add_sized([220.0, 60.0], btn).clicked() {
                        self.show_clocks = true;
                    }
                });
            } else {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.heading(egui::RichText::new("Fusi Orari").size(32.0).strong());
                    ui.add_space(20.0);
                    
                    ui.columns(2, |columns| {
                        // Colonna 1: Ora Locale
                        columns[0].vertical_centered(|ui| {
                            ui.label(egui::RichText::new("Ora Locale").size(20.0));
                            ui.add_space(10.0);
                            let now = Local::now();
                            draw_analog_clock(ui, now.hour(), now.minute(), now.second());
                        });
                        
                        // Colonna 2: Ora di Tokyo
                        columns[1].vertical_centered(|ui| {
                            ui.label(egui::RichText::new("Tokyo, Giappone").size(20.0));
                            ui.add_space(10.0);
                            let tokyo_time = Utc::now().with_timezone(&Tokyo);
                            draw_analog_clock(ui, tokyo_time.hour(), tokyo_time.minute(), tokyo_time.second());
                        });
                    });
                    
                    ui.add_space(40.0);
                    if ui.button("Nascondi Orologi").clicked() {
                        self.show_clocks = false;
                    }
                });
            }
        });
    }
}

// Funzione per disegnare l'orologio vettoriale
fn draw_analog_clock(ui: &mut egui::Ui, hour: u32, minute: u32, second: u32) {
    let (response, painter) = ui.allocate_painter(egui::vec2(220.0, 220.0), egui::Sense::hover());
    let center = response.rect.center();
    let radius = 100.0;

    // Sfondo dell'orologio
    painter.circle_filled(center, radius, egui::Color32::from_rgb(40, 40, 40));
    painter.circle_stroke(center, radius, egui::Stroke::new(3.0, egui::Color32::from_rgb(100, 100, 100)));

    // Calcolo degli angoli (in radianti) per le lancette
    let sec_angle = (second as f32 * 6.0) * PI / 180.0;
    let min_angle = (minute as f32 * 6.0 + second as f32 * 0.1) * PI / 180.0;
    let hr_angle = ((hour % 12) as f32 * 30.0 + minute as f32 * 0.5) * PI / 180.0;

    // Helper per tracciare le lancette
    let draw_hand = |angle: f32, length: f32, width: f32, color: egui::Color32| {
        let end = center + egui::vec2(angle.sin() * length, -angle.cos() * length);
        painter.line_segment([center, end], egui::Stroke::new(width, color));
    };

    // Tacche delle ore
    for i in 0..12 {
        let angle = (i as f32 * 30.0) * PI / 180.0;
        let p1 = center + egui::vec2(angle.sin() * (radius - 10.0), -angle.cos() * (radius - 10.0));
        let p2 = center + egui::vec2(angle.sin() * radius, -angle.cos() * radius);
        painter.line_segment([p1, p2], egui::Stroke::new(2.0, egui::Color32::GRAY));
    }

    // Lancetta delle Ore
    draw_hand(hr_angle, radius * 0.5, 6.0, egui::Color32::WHITE);
    // Lancetta dei Minuti
    draw_hand(min_angle, radius * 0.75, 4.0, egui::Color32::LIGHT_GRAY);
    // Lancetta dei Secondi
    draw_hand(sec_angle, radius * 0.9, 2.0, egui::Color32::from_rgb(255, 60, 60));

    // Perno centrale
    painter.circle_filled(center, 5.0, egui::Color32::WHITE);
}
