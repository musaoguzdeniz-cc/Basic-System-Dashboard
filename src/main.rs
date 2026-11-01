#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, Align, Color32, Frame, Layout, ProgressBar, RichText};
use std::time::{Duration, Instant};
use sysinfo::System;

const BACKGROUND: Color32 = Color32::from_rgb(23, 23, 23);
const FOREGROUND: Color32 = Color32::from_rgb(242, 242, 242);
const MUTED: Color32 = Color32::from_rgb(184, 184, 184);
const CPU: Color32 = Color32::from_rgb(93, 173, 226);
const RAM: Color32 = Color32::from_rgb(88, 214, 141);

struct ResourceWidget {
    system: System,
    last_refresh: Instant,
    cpu: f32,
    ram_percent: f32,
    used_gib: f64,
    total_gib: f64,
}

impl ResourceWidget {
    fn new(context: &eframe::CreationContext<'_>) -> Self {
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = BACKGROUND;
        visuals.window_fill = BACKGROUND;
        context.egui_ctx.set_visuals(visuals);
        let mut system = System::new().expect("Windows system metrics are unavailable");
        system.refresh_cpu_usage();
        system.refresh_memory();
        Self {
            system,
            last_refresh: Instant::now() - Duration::from_secs(1),
            cpu: 0.0,
            ram_percent: 0.0,
            used_gib: 0.0,
            total_gib: 0.0,
        }
    }

    fn refresh(&mut self) {
        if self.last_refresh.elapsed() < Duration::from_secs(1) {
            return;
        }
        self.system.refresh_cpu_usage();
        self.system.refresh_memory();
        self.cpu = self.system.global_cpu_usage().clamp(0.0, 100.0);
        let used = self.system.used_memory();
        let total = self.system.total_memory();
        self.ram_percent = if total == 0 {
            0.0
        } else {
            (used as f64 / total as f64 * 100.0) as f32
        };
        const GIB: f64 = 1024.0 * 1024.0 * 1024.0;
        self.used_gib = used as f64 / GIB;
        self.total_gib = total as f64 / GIB;
        self.last_refresh = Instant::now();
    }
}

impl eframe::App for ResourceWidget {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.refresh();
        ui.ctx().request_repaint_after(Duration::from_secs(1));
        ui.painter().rect_filled(ui.max_rect(), 0, BACKGROUND);
        Frame::new()
            .fill(BACKGROUND)
            .inner_margin(egui::Margin::symmetric(12, 7))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let title = ui.add(
                        egui::Label::new(
                            RichText::new("SYSTEM MONITOR")
                                .size(13.0)
                                .strong()
                                .color(FOREGROUND),
                        )
                        .sense(egui::Sense::drag()),
                    );
                    if title.drag_started() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
                    }
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui
                            .add(egui::Button::new(RichText::new("X").color(MUTED)).frame(false))
                            .clicked()
                        {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
                ui.add_space(9.0);
                ui.label(
                    RichText::new(format!("CPU   {:5.1}%", self.cpu))
                        .size(13.0)
                        .color(FOREGROUND),
                );
                ui.add(
                    ProgressBar::new(self.cpu / 100.0)
                        .fill(CPU)
                        .desired_height(7.0),
                );
                ui.add_space(6.0);
                ui.label(
                    RichText::new(format!(
                        "RAM   {:5.1}%   ({:.1} / {:.1} GB)",
                        self.ram_percent, self.used_gib, self.total_gib
                    ))
                    .size(13.0)
                    .color(FOREGROUND),
                );
                ui.add(
                    ProgressBar::new(self.ram_percent / 100.0)
                        .fill(RAM)
                        .desired_height(7.0),
                );
                ui.add_space(7.0);
                ui.label(
                    RichText::new("Güncelleme: 1 saniye  •  Sürükleyerek taşıyabilirsiniz")
                        .size(10.0)
                        .color(MUTED),
                );
            });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 150.0])
            .with_resizable(false)
            .with_decorations(false)
            .with_always_on_top(),
        ..Default::default()
    };
    eframe::run_native(
        "System Resource Widget",
        options,
        Box::new(|context| Ok(Box::new(ResourceWidget::new(context)))),
    )
}
